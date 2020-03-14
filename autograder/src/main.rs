mod args;
//mod cargo_test_output;
mod error;
//mod report;
//mod score_map;
mod run;
use run::run;

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);
    }
}
