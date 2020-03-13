#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod error;
mod report;
mod run;
mod score_map;
mod test_result;
use clap;
use error::Result;
use run::run;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
