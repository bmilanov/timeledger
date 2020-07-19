pub struct Options {
    pub ledger: String
}

impl Options {
    pub fn new(cli_args: clap::ArgMatches) -> Options {
        Options {
            ledger: cli_args.value_of("ledger").unwrap().to_string()
        }
    }
}
