use crate::error::Error;
use std::collections::BTreeMap;

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

pub fn get_max_score(name: &String, scores: &ScoreMap) -> Result<f32, Error> {
    let score = scores.get(name)?;
    Ok(*score)
}
