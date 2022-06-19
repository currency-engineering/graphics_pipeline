//! HashMap of HTML and CSS.

use anyhow::{anyhow, Result};
use crate::{
    primitives::Key,
    resources::{CssResourceType},
};
use std::{collections::HashMap, fs, path::{Path, PathBuf}};

pub struct Html(HashMap<Key, String>);

impl Html {

    /// ```
    /// let html = Html::new("test_contents")?; 
    /// assert!(false) // Should load other HTML files too.
    /// ```
    pub fn new<P: AsRef<Path>>(contents_dir: P) -> Result<Self> {
        let contents: PathBuf = contents_dir.as_ref().to_path_buf();

        let style = CssResourceType::into_resources(contents)
            .find(|pb| pb.ends_with("style.css"))
            .ok_or(anyhow!("Missing 'style.css'"))?;

        let mut map = HashMap::new();
        map.insert(Key::Style, fs::read_to_string(style)?);
        Ok(Html(map))
    }

    pub fn get(&self, key: &Key) -> Option<String> {
        self.0.get(key).cloned()
    }
}
