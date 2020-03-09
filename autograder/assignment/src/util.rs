use crate::error::Error;
use std::collections::HashMap;
use std::process::Command;
use std::process::Output;

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

pub fn cargo_test(path: String) -> Result<Output, Error> {
    //cargo test --manifest-path="../../Cargo.toml"  -- -Z unstable-options --format json -q
    Command::new("cargo")
        .arg("test")
        .arg(format!("--manifest-path={}", path))
        .arg("--")
        .arg("-Z")
        .arg("unstable-options")
        .arg("--format")
        .arg("json")
        .output()
        .map_err(Error::from)
}

pub fn get_max_score(name: &String, scores: &HashMap<String, f32>) -> f32 {
    *scores.get(name).unwrap_or(&1.0)
}
