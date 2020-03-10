use crate::error::Error;
use std::collections::btree_map::Values;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ScoreMap {
    map: BTreeMap<String, f32>,
}

impl ScoreMap {
    pub fn from_path(path: &PathBuf) -> Result<Self, Error> {
        let string = fs::read_to_string(path)?;
        Ok(Self {
            map: serde_yaml::from_str(&string)?,
        })
    }

    pub fn get(&self, name: &String) -> Result<f32, Error> {
        match self.map.get(name) {
            None => Err(Error::ScoreError(name.clone())),
            Some(x) => Ok(*x),
        }
    }

    pub fn values(&self) -> Values<String, f32> {
        self.map.values()
    }
}
