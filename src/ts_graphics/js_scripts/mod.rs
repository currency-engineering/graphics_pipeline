/// Loads all the JS scripts in a HashMap that returns Strings.

use actix_web::HttpResponse;
use anyhow::{bail, Result};
use crate::{
    file_resources::IntoResources,
    file_resources::impls::{
        TSGraphicsJs,
    },
    http_state::HttpState,
};
use std::{collections::HashMap, fmt, fs, path::{Path, PathBuf}, str::FromStr};

#[derive(Eq, PartialEq, Hash)]
pub struct Key(String);

impl Key {
    pub fn from_path(path: &Path) -> Result<Self> {
        match path.file_stem() {
            Some(os_str) => {
                match os_str.to_str() {
                    Some(s) => Ok(Key(s.to_string())),
                    None => bail!("Could not convert path to string."),
                }
            },
            None => bail!("Found empty path."),
        }
    }
}

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Key(s.to_string()))
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Keys are the short filename without the extension.
pub struct JsScripts(HashMap<Key, String>);

impl JsScripts {
    pub fn new<P: AsRef<Path>>(data_root: P) -> Result<Self> {
        let mut hm = HashMap::new();
        let pb: PathBuf = data_root.as_ref().to_path_buf();

        for path in TSGraphicsJs.into_resources(pb)?.iter() {
            let key = Key::from_path(&path)?; 
            let value = fs::read_to_string(path)?;
            hm.insert(key, value);
        }
        Ok(JsScripts(hm))
    }
}

impl HttpState for JsScripts {
    /// The filename without the extension
    type Key = Key;

    fn get(&self, key: Key) -> HttpResponse {
        match self.0.get(&key) {
            Some(s) => {
                HttpResponse::Ok()
                    .content_type("text/javascript")
                    .body(s.clone())
            },
            None => not_found()
        }
    }
}

pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}
