use crate::{
    // FromFile,
    primitives::{
        SeriesId,
    }
};
use key_tree::{
    KeyTree,
    KeyTreeError,
    serialize::{
        KeyTreeString,
        IntoKeyTree,
    },
};

// impl MetaData {

    // pub fn meta_resource_type(&self) -> ResourceType {
    //     ResourceType::MetaData { data_type: self.data_type, country: self.country, series_id: SeriesId }
    // }

// impl FromFile for MetaData {}

// === MetaData ===================================================================================

/// Component of [`MetaData`](struct.Series.html).
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::meta_data::Series;
/// #  let spec = r"
///    series_meta:
///        realtime:               2021-06-03
///        series_id:              AUSCPALTT01IXNBQ
///        title:                  Consumer Price Index: All items: Total: Total for Australia
///        observation_start:      1960-01-01
///        observation_end:        2021-01-01
///        frequency:              Quarterly
///        seasonal_adjustment:    Not Seasonally Adjusted
///        notes:                  (see JSON data for notes)
/// # ";
/// #   let _: Series = KeyTree::parse_str(spec).unwrap().try_into().unwrap();
/// ```
#[derive(Debug)]
pub struct Series {
    realtime: String,
    series_id: SeriesId,
    title: String,
    observation_start: String,
    observation_end: String,
    frequency: String,
    seasonal_adjustment: String,
}  

impl TryInto<Series> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<Series, Self::Error> {
        Ok(
            Series {
                realtime:               self.from_str("series_meta::realtime")?,
                series_id:              self.from_str("series_meta::series_id")?,
                title:                  self.from_str("series_meta::title")?,
                observation_start:      self.from_str("series_meta::observation_start")?,
                observation_end:        self.from_str("series_meta::observation_end")?,
                frequency:              self.from_str("series_meta::frequency")?,
                seasonal_adjustment:    self.from_str("series_meta::seasonal_adjustment")?,
            }
        )
    }
}

impl IntoKeyTree for Series {
    fn keytree(&self) -> KeyTreeString {
        let mut kt = KeyTreeString::new();

        kt.push_key(0, "series_item");
        kt.push_keyvalue(1, "realtime", &self.realtime);
        kt.push_keyvalue(1, "series_id", &self.series_id.to_string());
        kt.push_keyvalue(1, "title", &self.title);
        kt.push_keyvalue(1, "observation_start", &self.observation_start);
        kt.push_keyvalue(1, "observation_end", &self.observation_end);
        kt.push_keyvalue(1, "frequency", &self.frequency);
        kt.push_keyvalue(1, "seasonal_adjustment", &self.seasonal_adjustment);

        kt
    }
}
