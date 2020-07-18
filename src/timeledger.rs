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
}
