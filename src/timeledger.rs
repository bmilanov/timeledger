mod json_timeledger;

use chrono::{prelude::*, DateTime, Utc};
use math::round;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Task {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    description: String,
    tags: Vec<String>
}

#[derive(Debug)]
struct Day {
    date: DateTime<Utc>,
    tasks: Vec<Task>
}

#[derive(Debug)]
pub struct Timeledger {
    days: Vec<Day>,
}

impl Timeledger {
    pub fn from_json(json: &String) -> Result<Timeledger, std::io::Error> {
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

        Ok(Timeledger {
            days: days
        })
    }

    pub fn from_file(filename: &String) -> Result<Timeledger, std::io::Error> {
        let json = fs::read_to_string(filename)?;

        Timeledger::from_json(&json)
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

        report.push_str("# -------------- #\n");
        report.push_str("# Hours per Week #\n");
        report.push_str("# -------------- #\n");
        for (date, total_time) in m.iter() {
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
