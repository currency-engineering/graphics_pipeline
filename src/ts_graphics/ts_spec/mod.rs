#![allow(dead_code)]

use anyhow::{bail, Context, Error, Result};
use crate::{
    countries::Country,
    ts_graphics::TSGraphicCategory,
    primitives::{DataType, SeriesId},
};
use std::str::FromStr;
use key_tree::{KeyTree, KeyTreeError};
use key_tree::serialize::{KeyTreeString, IntoKeyTree};
use serde::Serialize;
use std::fmt;

// impl SpecFromFile for TSSpec {}

// === TSSpec ===================================================================================

#[derive(Debug)]
pub struct TSSpec {
    pub (crate) pages: Vec<PageSpec>,
}

impl TryInto<TSSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<TSSpec, Self::Error> {
        Ok(TSSpec{ pages: self.vec_at("ts_spec::graphic")? })
    }
}

// === PageSpec ===================================================================================

/// Component of [`TSSpec`](struct.TSSpec.html).
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::ts_graphics::ts_spec::PageSpec;
/// # let s = "
///     page:
///         country:        Australia
///         data_type:      u
///         index:          0
///
///         series:
///             data_type:  u
///             series_id:  AUSURAMS
///         series:
///             data_type:  u
///             series_id:  AUSURANAA
///
///         graphic:
///             category:   collation
///             series_id:  AUSURAMS
///             series_id:  AUSURANAA
/// # ";
/// # let _ps: PageSpec = KeyTree::parse_str(s)
///     .unwrap()
///     .try_into()
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct PageSpec {
    pub(crate) country: Country,
    pub(crate) data_type: DataType,
    pub(crate) index: usize,
    pub(crate) height_opt: Option<f32>,
    pub(crate) seriess: Vec<Series>,
    pub(crate) graphics: Vec<GraphicSpec>,
}

impl TryInto<PageSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<PageSpec, Self::Error> {
        Ok(
            PageSpec {
                country:    self.from_str("page::country")?, 
                data_type:  self.from_str("page::data_type")?,
                index:      self.from_str("page::index")?,
                height_opt: self.opt_from_str("page::height")?,
                seriess:    self.vec_at("page::series")?,
                graphics:   self.vec_at("page::graphic")?,
            }
        )
    }
}

// === Series =====================================================================================

/// The specification for a series, that is used across the build pipeline. The keytree representation
/// looks like
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::ts_graphics::ts_spec::Series;
/// # let spec = r#"
///   series:
///       data_type:          u
///       series_id:          LRHUTTTTAUA156S
/// # "#;
/// let _: Series = KeyTree::parse_str(spec)
///     .unwrap()
///     .try_into()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Series {
    pub(crate) data_type:      DataType,
    pub(crate) series_id:      SeriesId,
}

impl<'a> TryInto<Series> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<Series, Self::Error> {
        Ok(
            Series {
                data_type:  self.from_str("series::data_type")?, 
                series_id:  self.from_str("series::series_id")?,
            }
        )
    }
}

// === GraphicSpec ================================================================================

/// Component of a [`TSSpec`](struct.TSSpec.html).
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::ts_graphics::ts_spec::GraphicSpec;
/// # use graphics_pipeline::ts_graphics::TSGraphicCategory;
/// # let s = "
///     graphic:
///         category:   collation
///         series_id:  AUSURAMS
///         series_id:  AUSURANAA";
/// let gs: GraphicSpec = KeyTree::parse_str(s)
///     .unwrap()
///     .try_into()
///     .unwrap();
/// # assert_eq!(gs.category_opt, Some(TSGraphicCategory::Collation));
/// # assert_eq!(gs.series_ids[0].to_string(), "AUSURAMS");
/// ```
#[derive(Debug)]
pub struct GraphicSpec {
    pub category_opt:   Option<TSGraphicCategory>,
    pub series_ids:     Vec<SeriesId>,
    pub graphic_range:  Option<GraphicRange>,
    pub note:           Option<String>,
}

impl GraphicSpec {
    pub (crate) fn assert_has_one_series(&self) -> bool {
        self.series_ids.len() == 1
    }
}

impl TryInto<GraphicSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<GraphicSpec, Self::Error> {
        Ok(
            GraphicSpec {
                category_opt:   self.opt_from_str("graphic::category")?,
                series_ids:     self.vec_from_str("graphic::series_id")?,
                graphic_range:  self.opt_from_str("graphic::range")?,
                note:           self.opt_from_str("graphic::note")?,
            }
        )
    }
}

impl IntoKeyTree for GraphicSpec {
    fn keytree(&self) -> KeyTreeString {
        let mut kt = KeyTreeString::new();
        kt.push_key(0, "graphic" );

        if let Some(class) = &self.category_opt {
            kt.push_keyvalue(1, "class", class);
        }

        if let Some(range) = &self.graphic_range {
            kt.push_keyvalue(1, "graphic", range);
        }

        if let Some(note) = &self.note {
            kt.push_keyvalue(1, "note", note);
        }

        for series_id in &self.series_ids {
            kt.push_keyvalue(1, "series_id", series_id);
        }
        kt
    }
}

// === GraphicRange ===============================================================================

#[derive(Clone, Copy, Debug, Serialize)]
/// Specifies the range of a graphic
pub struct GraphicRange {
    min:    f32,
    max:    f32,
}

impl FromStr for GraphicRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        let segment: Vec<&str> = s.split(" to ").collect();

        if segment[0].is_empty() || segment[1].is_empty() {
            bail!("Parse into GraphicRange failed.")
        };

        let min = segment[0].parse().context("Parse into GraphicRange failed.")?;

        let max = segment[1].parse().context("Parse into GraphicRange failed.")?;
        
        Ok(GraphicRange { min, max })
    }
}

impl fmt::Display for GraphicRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.min, self.max)
    }
}

// /// The United States prime interest rate data is daily. To buid a monthly time-series, we read
// /// through raw csv data, calculate a monthly value and add to to the time-series. The data include
// /// missing days, so we need the mechanism to ignore datepoints with value ".".
// 
//     ts.try_into().map_err(|err: time_series::error::Error| {
//         external(
//             file!(),
//             line!(),
//             &err.to_string(),
//         )
//     })
// }

#[cfg(test)]
pub mod test {

    use key_tree::KeyTree;
    use crate::ts_graphics::ts_spec::PageSpec;

    #[test]
    fn pagespec_from_keytree_should_work() {
        let s = r#"
          page:
              country:        Australia
              data_type:      u
              index:          0
        
              series:
                  data_type:  u
                  series_id:  AUSURAMS
              series:
                  data_type:  u
                  series_id:  AUSURANAA
        
              graphic:
                  category:   collation
                  series_id:  AUSURAMS
                  series_id:  AUSURANAA
        "#;
        let ps: PageSpec = KeyTree::parse_str(s)
          .unwrap()
          .try_into()
          .unwrap();
        assert_eq!(ps.seriess[0].series_id.to_string(), "AUSURAMS");
    }
}
