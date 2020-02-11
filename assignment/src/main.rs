extern crate submission;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    return submission::write(&args[1]);
}
