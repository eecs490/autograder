use lib;
use std::env;
use submission;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("Must provide one argument representing path to write results file.");
    return submission::write(&lib::json_string(), &path);
}
