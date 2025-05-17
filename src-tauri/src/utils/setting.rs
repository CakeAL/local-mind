use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub ollama_url: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".into(),
        }
    }
}

impl Setting {
    pub fn load(app: &tauri::AppHandle) -> Result<Self> {
        let setting_file = path::get_data_dir(app)?.join("setting.json");
        match File::open(setting_file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let json = serde_json::from_reader(reader)?;
                Ok(json)
            }
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn save(&self, app: &tauri::AppHandle) -> Result<()> {
        let setting_file = path::get_data_dir(app)?.join("setting.json");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(setting_file)?;
        file.write_all(&serde_json::to_vec(&self)?)?;
        Ok(())
    }

    pub fn set_ollama_url(&mut self, url: String, app: &tauri::AppHandle) -> Result<()> {
        self.ollama_url = url;
        self.save(app)
    }
}
