
pub mod js_scripts;
pub mod ts_spec;

use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use std::{fmt, str::FromStr};

// === GraphicCategory ============================================================================

#[derive(Debug, PartialEq, Serialize)]
pub enum TSGraphicCategory {
    /// Generally the top graphic which displays all time-series. 
    Collation,

    /// A single time-series which displays un-transformed data directly from source.
    Source,

    /// Data that has been selected and transformed.
    Cleaned,
}

impl FromStr for TSGraphicCategory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "collation" => Ok(TSGraphicCategory::Collation),
            "source"    => Ok(TSGraphicCategory::Source),
            "cleaned"   => Ok(TSGraphicCategory::Cleaned),
            _           => Err(anyhow!(format!("Failed to parse a TSGraphicCategory from [{}]", s))),
        }
    }
}

impl fmt::Display for TSGraphicCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TSGraphicCategory::Collation  => "collation",
            TSGraphicCategory::Source    => "source",
            TSGraphicCategory::Cleaned   => "cleaned",
        };
        write!(f, "{}", s)
    }
}


