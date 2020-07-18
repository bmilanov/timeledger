mod json_timeledger;

use chrono::{DateTime, Utc};
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

    pub fn report_hours_per_day(self) -> String {
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
            report.push_str(&format!("{}: {:.5}\n", day.date.format("%Y-%m-%d").to_string(),
                                                    (day.total_time.num_minutes() as f64 / 60.0).to_string()));
        }

        report.push_str("# ------------- #\n");

        report
    }
}
