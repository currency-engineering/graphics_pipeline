//! Transform data
//!
//! The transformed data set is a mapping of raw data to transformed data. Because
//! series_spec.keytree defines the _pipeline_ of data rather than just a set of data, the
//! transforms should be defined in it.
//!
//! So this crate takes series_spec.keytree, and /raw_data/ data on disk, and builds
//! /transformed_data/.
//!
//! We'll start by just copying the data from /raw_data/ directly using series_spec.

// load series_spec

// loop through all the series and copy.

use crate::{
    series_spec::SeriesSpec,
};
use time_series::{
    Date,
    RegularTimeSeries,
    Value,
};

// pub fn save_transforms<P, S>(root_dir: P, ts_spec_path: S) -> Result<()>
// where
//     P: AsRef<Path>,
//     S: AsRef<OsStr>,
// {
//     let root: PathBuf = root_dir.as_ref().to_path_buf();
//     let path: &OsStr = ts_spec_path.as_ref();
//     let spec_map: SeriesSpecMap = spec_map_from_spec(root, path)?;
//     
//     Ok(())
// 
// }

#[cfg(test)]
pub mod tests {

    #[test]
    fn series_in_spec_should_also_be_in_transformed_data() {
    // get a series from series_spec
    // check if it exists in /transformed_data
    }
}

// A `Transform` takes a `RegularTimeSeries` and the transform information in a `SeriesSpec` and
// outputs another `RegularTimeSeries`.  pub trait Transform {
pub trait Transform<D1: Date, V1: Value, D2: Date, V2: Value> {
    fn transform(time_series: RegularTimeSeries<D1, V1>, series_spec: SeriesSpec) -> RegularTimeSeries<D2, V2>;
}

