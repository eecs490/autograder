use crate::error::Argument;
use crate::error::Result;
use clap::{value_t, App, Arg, ArgMatches};
use snafu::ResultExt;
use std::path::PathBuf;

pub struct Args<'a>(ArgMatches<'a>);

impl Args<'_> {
    pub fn get() -> Self {
        Args(
            App::new("MyApp")
                .arg(
                    Arg::with_name("our_test_results")
                        .long("our-test-results")
                        .help("path to output of running our tests on their solution")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("their_test_results")
                        .long("their-test-results")
                        .help("path to output of running their tests on our solution")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("submission")
                        .long("submission")
                        .help("path to submission/Cargo.toml")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .help("path where results.json will be written")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("lcov")
                        .long("lcov")
                        .help("path to lcov.info")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("scores")
                        .help("path to scores.yaml")
                        .long("scores")
                        .takes_value(true)
                        .required(true),
                )
                .get_matches(),
        )
    }

    pub fn get_path_buf(&self, arg: &str) -> Result<PathBuf> {
        let matches: &clap::ArgMatches = &self.0;
        value_t!(matches, arg.clone(), PathBuf).context(Argument { arg })
    }
}
