// The primitives module contains data-structures that represent information, rather than code
// related constructs.

use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use std::{fmt,str::FromStr};

// === DataType ===================================================================================

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataType {
    U,
    Cpi,
    Inf,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            DataType::U => "u",
            DataType::Cpi => "cpi",
            DataType::Inf => "inf",
        };
        write!(f, "{}", label)
    }
}

impl FromStr for DataType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "u" => Ok(DataType::U),
            "cpi" => Ok(DataType::Cpi),
            "inf" => Ok(DataType::Inf),
            _ => Err(anyhow!("Failed to read data type")),
        }
    }
}

// === SeriesId ===================================================================================

/// Represents a FRED series id like `LRHUTTTTAUA156N` or a transformation on a FRED series_id
/// like `LRHUTTTTAUA156N_a`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SeriesId(String);

impl SeriesId {

    /// Create a new SeriesId from a string.
    pub fn new(s: &str) -> Self {
        SeriesId(s.to_string())
    }

    /// Return the component without transformation modifications.
    pub fn stem(&self) -> Self {
        let inner = self.0.split('_').next().unwrap().clone();
        SeriesId(String::from(inner)) 
    }
}

impl FromStr for SeriesId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(SeriesId(String::from(s)))
    }
}

impl fmt::Display for SeriesId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
