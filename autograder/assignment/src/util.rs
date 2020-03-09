use crate::error::Error;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

pub type ScoreMap = BTreeMap<String, f32>;

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), $value);
            )+
            m
        }
     };
);

pub fn cargo_test(path: PathBuf) -> Result<Output, Error> {
    //cargo test --manifest-path="../../Cargo.toml"  -- -Z unstable-options --format json -q
    Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(path)
        .arg("--")
        .arg("-Z")
        .arg("unstable-options")
        .arg("--format")
        .arg("json")
        .output()
        .map_err(Error::from)
}

pub fn replace_solution(our_solution: PathBuf, their_solution: PathBuf) -> Result<(), Error> {
    Ok(())
}

pub fn get_max_score(name: &String, scores: &ScoreMap) -> f32 {
    *scores.get(name).unwrap_or(&1.0)
}
