use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub reference: String,
    pub title: String,
}

pub fn read_default_themes() -> Result<Vec<Theme>, Error> {
    let file = File::open("src/resources/themes.json")?;
    let reader = BufReader::new(file);
    let themes = serde_json::from_reader(reader)?;
    Ok(themes)
}
