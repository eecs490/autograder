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

pub fn get_max_score(name: &String, scores: &ScoreMap) -> f32 {
    *scores.get(name).unwrap_or(&1.0)
}
