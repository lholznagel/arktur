use clap::ArgMatches;

mod content;
mod ping;
mod key;

pub fn execute(args: &ArgMatches) {
    match args.subcommand() {
        ("content", Some(sub_matches)) => content::execute(sub_matches),
        ("genkey", Some(sub_matches))  => key::genkey(sub_matches),
        ("pubkey", Some(sub_matches))  => key::pubkey(sub_matches),
        ("ping", Some(sub_matches))    => ping::execute(sub_matches),
        _                              => error!("Not valid")
    }
}