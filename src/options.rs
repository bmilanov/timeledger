pub struct Options {
    pub ledger: String
}

impl Options {
    pub fn new(cli_args: clap::ArgMatches) -> Options {
        Options {
            // clap ensures Some(T)
            ledger: cli_args.value_of("ledger").unwrap().to_string()
        }
    }
}
