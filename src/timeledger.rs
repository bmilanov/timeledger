mod json_timeledger;

use chrono::{prelude::*, DateTime, Utc};
use math::round;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{ Debug, Display, Formatter };
use std::fs;

use super::output::{ LogLevel, Out };

#[derive(Debug, PartialOrd)]
struct Task {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    description: String,
    tags: Vec<String>
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}: {}", self.start, self.end, self.description)
    }
}

#[derive(Debug, PartialOrd)]
struct Day {
    date: DateTime<Utc>,
    tasks: Vec<Task>
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start) &&
            (self.end == other.end)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.date)
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

#[derive(Debug)]
pub struct Timeledger<'a, T: Out + Debug> {
    out: &'a T,
    days: Vec<Day>
}

impl<'a, T> Timeledger<'a, T>
  where T: Out + Debug,
{
    /* TODO: In nightly: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.is_sorted */
    fn are_days_valid(out: &T, days: &Vec<Day>) -> bool {
        if days.len() < 2 {
            return true;
        }

        let mut prev_day = &days[0];
        let mut are_days_valid = Timeledger::<T>::are_hours_valid(out, &days[0].tasks);

        for day in days[1..].iter() {
            if !Timeledger::<T>::are_hours_valid(out, &day.tasks) {
                are_days_valid = false;
            }

            if prev_day >= day {
                are_days_valid = false;
                out.log(LogLevel::Warn, format_args!("{} appears before {}", prev_day, day));
            }
            prev_day = day;
        }

        are_days_valid
    }

    fn are_hours_valid(out: &T, tasks: &Vec<Task>) -> bool {

        let mut are_hours_valid = true;

        for (i, task) in tasks.iter().enumerate() {
            for other in tasks[(i+1)..].iter() {
                if other.start <= task.end {
                    out.log(LogLevel::Warn, format_args!("Task \"{}\" overlaps with task \"{}\"", task, other));

                    are_hours_valid = false;
                }
            }
        }

        are_hours_valid
    }

    pub fn from_json(out: &'a T, json: &String) -> Result<Timeledger<'a, T>, std::io::Error>
    {
        let json_timeledger = json_timeledger::JsonTimeledger::new(json)?;

        let days: Vec<Day> = json_timeledger.timeledger.iter().map(
            |day| {
                day.iter().map(
                    |(date, tasks)| {
                        Day {
                            date: format!("{}T00:00:00Z", date).parse::<DateTime<Utc>>().unwrap(),
                            tasks: tasks.iter().map(
                                |task| {
                                    Task {
                                        start: format!("{}T{}:00Z", date, task[0]).parse::<DateTime<Utc>>().unwrap(),
                                        end: format!("{}T{}:00Z", date, task[1]).parse::<DateTime<Utc>>().unwrap(),
                                        description: task[2].clone(),
                                        tags: task[3..].to_vec()
                                    }
                                }
                            ).collect::<Vec<Task>>()
                        }
                    }
                ).collect::<Vec<Day>>()
            }
        ).flatten().collect();

        if !Timeledger::<T>::are_days_valid(out, &days) {
            out.log(LogLevel::Warn, format_args!("Ledger contains at least one issue, e.g. days or tasks are out of order, or tasks overlap"));
        }

        Ok(Timeledger {
            out: out,
            days: days
        })
    }

    pub fn from_file(out: &'a T, filename: &String) -> Result<Timeledger<'a, T>, std::io::Error>
    {
        let json = fs::read_to_string(filename)?;

        Timeledger::from_json(out, &json)
    }

    pub fn report_hours_per_day(&self) -> String {
        let mut report = String::new();

        report.push_str("# ------------- #\n");
        report.push_str("# Hours per Day #\n");
        report.push_str("# ------------- #\n");

        struct DayTotal {
            date: DateTime<Utc>,
            total_time: chrono::Duration
        }

        let days: Vec<DayTotal> = self.days.iter().map(
            |day| {
                let total_time = day.tasks.iter().map(
                    |task| {
                        task.end - task.start
                    }
                ).fold(chrono::Duration::zero(),
                    |acc, x| {
                        acc + x
                    }
                );

                DayTotal {
                    date: day.date,
                    total_time: total_time
                }
            }
        ).collect();

        for day in days.iter() {
            report.push_str(&format!("{}: {}hrs\n", day.date.format("%Y-%m-%d").to_string(),
                                                    round::half_away_from_zero(day.total_time.num_minutes() as f64 / 60.0, 2).to_string()));
        }

        report.push_str("# ------------- #\n");

        report
    }

    pub fn report_hours_per_week(&self) -> String {
        pub fn first_dow(date: &chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
            *date - chrono::Duration::days((date.weekday().number_from_monday()-1) as i64)
        }

        let mut report = String::new();

        let days: Vec<_> = self.days.iter().map(
            |day| {
                let total_time = day.tasks.iter().map(
                    |task| {
                        task.end - task.start
                    }
                ).fold(chrono::Duration::zero(),
                    |acc, x| {
                        acc + x
                    }
                );

                (first_dow(&day.date), total_time)
            }
        ).collect();

        let mut m: HashMap<chrono::DateTime<Utc>, chrono::Duration> = HashMap::new();

        days.iter().for_each(
            |x| {
                if m.contains_key(&x.0) {
                    m.insert(x.0, *m.get(&x.0).unwrap() + x.1);
                }
                else {
                    m.insert(x.0, x.1);
                }
            }
        );

        let mut days = m.iter().collect::<Vec<(&chrono::DateTime<Utc>, &chrono::Duration)>>();
        days.sort_by(|a, b| a.0.cmp(&b.0));

        report.push_str("# -------------- #\n");
        report.push_str("# Hours per Week #\n");
        report.push_str("# -------------- #\n");
        for (date, total_time) in days.iter() {
            report.push_str(&format!("{}: {}hrs\n", date.format("%Y-%m-%d").to_string(),
                                                    round::half_away_from_zero(total_time.num_minutes() as f64 / 60.0, 2).to_string()));
        }
        report.push_str("# -------------- #\n");

        report
    }

    pub fn report_hours_per_tag(&self) -> String {
        let mut report = String::new();

        let days: Vec<_> = self.days.iter().map(
            |day| {
                day.tasks.iter().map(
                    |task| {
                        task.tags.iter().map(
                            |tag| {
                                (tag.clone(), task.end - task.start)
                            }
                        ).collect::<Vec<(String, chrono::Duration)>>()
                    }
                ).collect::<Vec<Vec<(String, chrono::Duration)>>>()
            }
        ).flatten().flatten().collect::<Vec<(String, chrono::Duration)>>();

        let mut m: HashMap<String, chrono::Duration> = HashMap::new();

        days.iter().for_each(
            |x| {
                if m.contains_key(&x.0) {
                    m.insert(x.0.clone(), *m.get(&x.0).unwrap() + x.1);
                }
                else {
                    m.insert(x.0.clone(), x.1);
                }
            }
        );

        let mut days = m.iter().collect::<Vec<(&String, &chrono::Duration)>>();
        days.sort_by(|a, b| a.1.cmp(&b.1));

        report.push_str("# ------------- #\n");
        report.push_str("# Hours per Tag #\n");
        report.push_str("# ------------- #\n");
        for (tag, duration) in days.iter() {
            report.push_str(&format!("{}: {}hrs\n", tag, round::half_away_from_zero(duration.num_minutes() as f64 / 60.0, 2).to_string()));
        }
        report.push_str("# ------------- #\n");

        report
    }
}
