use std::fs::{create_dir, create_dir_all, read_to_string, remove_dir_all};
use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::root_dir;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Registry(HashMap<String, PathBuf>);

impl Registry {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn path() -> Result<PathBuf> {
        Ok(root_dir()?.join("games.json"))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if !path.exists() {
            create_dir_all(&root_dir()?)?;
        }

        let data = to_string_pretty(&self)?;
        std::fs::write(&path, data)?;

        Ok(())
    }
    
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::new());
        }

        let data = read_to_string(&path)?;
        if data.trim().is_empty() {
            return Ok(Self::new());
        }

        Ok(Self(from_str(&data)?))
    }

    pub fn contains(&self, value: &str) -> bool {
        self.0.contains_key(value)
    }
    pub fn insert(&mut self, k: String, v: PathBuf) -> Result<()> {
        create_dir(crate::game_dir(&k)?)?;
        
        self.0.insert(k, v);

        Ok(())
    }
    pub fn remove(&mut self, value: &str) -> Result<()> {
        self.0.remove(value);
        
        remove_dir_all(crate::game_dir(value)?)?;

        Ok(())
    }
    pub fn get(&self, value: &str) -> Option<&PathBuf> {
        self.0.get(value)
    }
}