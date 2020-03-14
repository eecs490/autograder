mod args;
//mod cargo_test_output;
mod error;
//mod report;
mod run;
mod score_map;
use run::run;

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);
    }
}
