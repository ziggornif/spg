use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub reference: String,
    pub title: String,
}

pub fn read_default_themes() -> Result<Vec<Theme>, Error> {
    let themes = serde_json::from_str(include_str!("./resources/themes.json"))?;
    Ok(themes)
}
