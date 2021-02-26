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
fn timeledger_valid() -> Result<(), Box<dyn Error>> {

    let past_day_in_future = String::from(r#"
{
    "timeledger":
    [
        {
            "2019-08-05":
            [
                [ "09:30", "09:40", "Task A", "tag1", "tag2", "tag3" ],
                [ "09:40", "10:40", "Task B", "tag2", "tag3", "tag4" ],
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

    let actual = test_out.buffer.borrow_mut().pop().unwrap();

    assert_eq!(actual, "Ledger contains at least one day that is not in order");

    Ok(())
}
