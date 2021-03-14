use std::cell::RefCell;
use std::error::Error;
use std::fmt::Arguments;

use ::timeledger::timeledger;
use ::timeledger::output::{ LogLevel, Log, Output, Out };

#[derive(Clone, Debug)]
pub struct TestOut {
    pub buffer: RefCell<Vec<String>>
}

impl Output for TestOut {
    fn write(&self, args: Arguments) {
        self.buffer.borrow_mut().push(format!("{}", args));
    }
}

impl Log for TestOut {
    fn log(&self, _level: LogLevel, args: Arguments) {
        self.buffer.borrow_mut().push(format!("{}", args));
    }
}

impl Out for TestOut {}

#[test]
fn days_out_of_order() -> Result<(), Box<dyn Error>> {

    let past_day_in_future = String::from(r#"
{
    "timeledger":
    [
        {
            "2019-08-05":
            [
                [ "09:30", "09:39", "Task A", "tag1", "tag2", "tag3" ],
                [ "09:40", "10:39", "Task B", "tag2", "tag3", "tag4" ],
                [ "10:40", "10:50", "Task C", "tag3", "tag4", "tag5" ]
            ]
        },
        {
            "2019-08-03":
            [
                [ "11:49", "12:20", "Task A", "tag1", "tag2", "tag3" ],
                [ "12:30", "12:54", "Task D", "tag2", "tag3", "tag4" ],
                [ "15:20", "17:11", "Task F", "tag3", "tag4", "tag5" ]
            ]
        },
        {
            "2019-08-06":
            [
                [ "11:49", "12:20", "Task A", "tag1", "tag2", "tag3" ],
                [ "12:30", "12:54", "Task D", "tag2", "tag3", "tag4" ],
                [ "15:20", "17:11", "Task F", "tag3", "tag4", "tag5" ]
            ]
        }
    ]
}
"#);

    let test_out = TestOut { buffer: RefCell::new(vec![]) };

    let _timeledger = timeledger::Timeledger::from_json(&test_out, &past_day_in_future)?;

    let actual = test_out.buffer.borrow().clone();

    let expected = vec![
        "2019-08-05 00:00:00 UTC appears before 2019-08-03 00:00:00 UTC",
        "Ledger contains at least one issue, e.g. days or tasks are out of order, or tasks overlap"
    ];

    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn tasks_overlap() -> Result<(), Box<dyn Error>> {

    let past_day_in_future = String::from(r#"
{
    "timeledger":
    [
        {
            "2019-08-05":
            [
                [ "12:30", "14:40", "Task A", "tag1", "tag2", "tag3" ],
                [ "09:40", "10:39", "Task B", "tag2", "tag3", "tag4" ],
                [ "10:40", "10:50", "Task C", "tag3", "tag4", "tag5" ]
            ]
        },
        {
            "2019-08-06":
            [
                [ "14:49", "15:19", "Task A", "tag1", "tag2", "tag3" ],
                [ "12:30", "12:54", "Task D", "tag2", "tag3", "tag4" ],
                [ "15:20", "17:11", "Task F", "tag3", "tag4", "tag5" ]
            ]
        },
        {
            "2019-08-07":
            [
                [ "11:49", "12:20", "Task A", "tag1", "tag2", "tag3" ],
                [ "12:30", "15:20", "Task D", "tag2", "tag3", "tag4" ],
                [ "15:20", "17:11", "Task F", "tag3", "tag4", "tag5" ]
            ]
        }
    ]
}
"#);

    let test_out = TestOut { buffer: RefCell::new(vec![]) };

    let _timeledger = timeledger::Timeledger::from_json(&test_out, &past_day_in_future)?;

    let actual = test_out.buffer.borrow().clone();

    let expected = vec![
        "Task \"2019-08-05 12:30:00 UTC - 2019-08-05 14:40:00 UTC: Task A\" overlaps with task \"2019-08-05 09:40:00 UTC - 2019-08-05 10:39:00 UTC: Task B\"",
        "Task \"2019-08-05 12:30:00 UTC - 2019-08-05 14:40:00 UTC: Task A\" overlaps with task \"2019-08-05 10:40:00 UTC - 2019-08-05 10:50:00 UTC: Task C\"",
        "Task \"2019-08-06 14:49:00 UTC - 2019-08-06 15:19:00 UTC: Task A\" overlaps with task \"2019-08-06 12:30:00 UTC - 2019-08-06 12:54:00 UTC: Task D\"",
        "Task \"2019-08-07 12:30:00 UTC - 2019-08-07 15:20:00 UTC: Task D\" overlaps with task \"2019-08-07 15:20:00 UTC - 2019-08-07 17:11:00 UTC: Task F\"",
        "Ledger contains at least one issue, e.g. days or tasks are out of order, or tasks overlap"
    ];

    assert_eq!(actual, expected);

    Ok(())
}
