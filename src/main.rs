#[macro_use]

extern crate clap;

use clap::App;
use std::error::Error;

mod timeledger;
mod options;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let app_options = options::Options::new(matches);

    let timeledger = timeledger::Timeledger::from_file(&app_options.ledger)?;

    println!("{}", timeledger.report_hours_per_day());

    Ok(())
}
