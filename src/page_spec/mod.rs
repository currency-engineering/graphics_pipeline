
use crate::{
    countries::Country,
    file_resources::DataType,
    primitives::SeriesId,
    ts_graphics::ts_spec::GraphicSpec,
};
use key_tree::{KeyTree, KeyTreeError};
use key_tree::serialize::{IntoKeyTree, KeyTreeString};
use std::collections::BTreeMap;


// The data-structures which hold data that are used by a server.

// === PageSpec ===================================================================================

// There are two important functions that need to be built.
// 1. SeriesSpec.into_json(). This is done in lib.rs.
// 2. PageJson.into_data()

/// Component of time-series specification.
/// ```
/// page:
///     country:            Australia
///     data_type:          u
///     index:              0
///     graphic:
///         series:
///             data_type:  u
///             series_id:  AUSURAMS
///         series:
///             data_type:  u
///             series_id:  AUSURANAA
/// ```
#[derive(Debug)]
pub struct PageSpec {
    country:    Country,
    data_type:  DataType,
    index:      usize,
    height_opt: Option<f32>,
    pub seriess:    BTreeMap<SeriesId, series_spec::SeriesSpec>,
    pub graphics:   Vec<GraphicSpec>,
}

impl TryInto<PageSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<PageSpec, Self::Error> {

        let seriess_vec: Vec<SeriesSpec> = self.vec_at("page::series")?;
        let mut map = BTreeMap::new();
        for series_spec in seriess_vec {
            map.insert(series_spec.series_id.clone(), series_spec);
        } 

        Ok(
            PageSpec {
                country:    self.from_str("page::country")?, 
                data_type:  self.from_str("page::data_type")?,
                index:      self.from_str("page::index")?,
                height_opt: self.opt_from_str("page::height")?,
                seriess:    map, 
                graphics:   self.vec_at("page::graphic")?,
            }
        )
    }
}

impl IntoKeyTree for PageSpec {
    fn keytree(&self) -> KeyTreeString {
        let mut kt = KeyTreeString::new();
        kt.push_key(0, "page");
        kt.push_keyvalue(1, "country", self.country);
        kt.push_keyvalue(1, "data_type", self.data_type);
        kt.push_keyvalue(1, "index", self.index);

        for (_, series) in self.seriess.iter() {
            kt.push_keytree(1, series.keytree());
        }

        for graphic in &self.graphics {
            kt.push_keytree(1, graphic.keytree());
        }
        kt
    }
}
