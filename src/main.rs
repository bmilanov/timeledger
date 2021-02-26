#[macro_use]
extern crate clap;

use clap::App;
use std::error::Error;
use std::fmt::{ Arguments, Debug };

mod output;
mod timeledger;
mod options;

use output::{Log, Output};

use log::{trace, debug, info, warn, error, log_enabled, Level, LevelFilter};

use env_logger::Builder;

#[derive(Debug)]
pub struct MainOut;

impl Output for MainOut {
    fn write(&self, args: Arguments) {
        println!("{}", args);
    }
}

impl Log for MainOut {
    fn log(&self, level: output::LogLevel, args: Arguments) {
        match level {
            output::LogLevel::Error => error!("{}", args),
            output::LogLevel::Warn  => warn!("{}", args),
            output::LogLevel::Info  => info!("{}", args),
            output::LogLevel::Debug => debug!("{}", args),
            output::LogLevel::Trace => trace!("{}", args)
        }
    }
}

impl output::Out for MainOut {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new();
    builder.filter(None, LevelFilter::Warn).init();

    let my_main_out = MainOut {};
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let app_options = options::Options::new(matches);

    let timeledger = timeledger::Timeledger::from_file(&my_main_out, &app_options.ledger)?;

    println!("{}", timeledger.report_hours_per_tag());
    println!("{}", timeledger.report_hours_per_day());
    println!("{}", timeledger.report_hours_per_week());

    Ok(())
}
