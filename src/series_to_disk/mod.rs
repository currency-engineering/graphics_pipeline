use anyhow::Result;
use crate::{
    countries::Country,
    file_resources::impls::{CsvRawData, Spec},
    file_resources::IntoResources,
    primitives::{DataType, SeriesId},
    series_spec::{SeriesSpec, SeriessSpec},
};
use key_tree::KeyTree;
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// Take the time series specification and take series data from the `/data/raw_data` directory on
/// disk and save to the `/data/transformed_data` directory.
pub fn spec_map_from_spec<P, S>(root_dir: P, ts_spec_path: S) -> Result<SeriesSpecMap>
where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
{
    let root: PathBuf = root_dir.as_ref().to_path_buf();
    let spec_path = Spec.full_path(&root, ts_spec_path)?;
    let spec: SeriessSpec = KeyTree::parse(spec_path)?.try_into()?;
    Ok(spec.iter().collect())
}

/// Checks if raw data is synced to ts_spec and displays results.
pub fn verify_raw<P, S>(root_dir: P, ts_spec_path: S) -> Result<()>
where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
{
    let root: PathBuf = root_dir.as_ref().to_path_buf();
    let path: &OsStr = ts_spec_path.as_ref();

    let spec_map: SeriesSpecMap = spec_map_from_spec(&root, path)?;
    for ((data_type, country), inner_map) in spec_map.map.iter() {

        let csv_raw_data = CsvRawData {country: *country, data_type: *data_type};

        csv_raw_data.into_resources(&root)?;

        for (_series_id, series_spec) in inner_map.iter() {

            let filename = PathBuf::from(&series_spec.series_id().to_string()).with_extension("csv");

            match csv_raw_data.has_file(&root, &filename)? {
                true => println!(" ok  {}", filename.display()),
                false => println!("none {}", filename.display()),
            }
        }
    }
    Ok(())
}

// === SeriesSpecMap ==============================================================================

// `SeriesSpecMap` is set up in what seems like an overly complex way in order to maintain the
// order is which data is `SeriesSpecMap` orders the series. We want to maintain an ordering by
// (DataType, Country). But sometimes we also need to search by SeriesId so also need a reverse
// lookup.
//
// `SeriesSpecMap` is used by all the steps from the start to saving data, and therefore by several
// crates.
//

#[derive(Debug)]
pub struct SeriesSpecMap {
    map: BTreeMap<(DataType, Country), BTreeMap<SeriesId, SeriesSpec>>,
    reverse: BTreeMap<SeriesId, (DataType, Country)>,
}

impl SeriesSpecMap {

    /// Used to build a generic specification.
    fn new() -> Self {
        SeriesSpecMap {
            map: BTreeMap::new(),
            reverse: BTreeMap::new(),
        }
    }

    /// Get a `SeriesSpec` from a `SeriesId`.
    pub fn get_series_spec(&self, series_id: &SeriesId) -> Option<SeriesSpec> {
        let key = match self.reverse.get(&series_id) {
            Some(key) => key,
            None => { return None },
        };
        // If the series_id is in reverse it must also be in map.
        let inner_map = self.map.get(&key).unwrap();
        let series_spec = inner_map.get(series_id).unwrap(); // &SeriesSpec
        Some((*series_spec).clone())
    }

    // Convenience method for testing.
    #[allow(dead_code)]
    fn get_inner_map(&self, key: &(DataType, Country)) -> Option<BTreeMap<SeriesId, SeriesSpec>> {
        self.map.get(key).cloned()
    }

    pub fn insert(&mut self, series_spec: &SeriesSpec) {
        let key = (series_spec.data_type(), series_spec.country());
        match self.map.get_mut(&key) {
            Some(inner_map) => {
                inner_map.insert(series_spec.series_id(), series_spec.clone());
            },
            None => {
                let mut value = BTreeMap::new();
                value.insert(series_spec.series_id(), (*series_spec).clone());
                self.map.insert(key, value);
                self.reverse.insert(series_spec.series_id(), key);
            },
        }
    }
}

impl FromIterator<SeriesSpec> for SeriesSpecMap {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = SeriesSpec>
    {
        let mut map = Self::new();
        for series_spec in iter {
            map.insert(&series_spec);
        }
        map
    }
}

// === Tests ======================================================================================

#[cfg(test)]
pub mod test {

    use crate::series_to_disk::SeriesSpecMap;
    use crate::series_spec::SeriesSpec;
    use crate::primitives::{DataType, SeriesId};
    use crate::countries::Country;

    #[test]
    fn insert_should_work() {
        let series_id = SeriesId::new("first");

        let input = SeriesSpec::new(
            DataType::U,
            Country::Australia,
            series_id.clone(),
        );

        let mut map = SeriesSpecMap::new();
        map.insert(&input);
        if let Some(output) = map.get_series_spec(&series_id) {
            assert_eq!(output.country(), Country::Australia);
        } else {
            assert!(false);
        }
    }
     
    #[test]
    fn inserts_with_same_key_should_work() {
        let series_id = SeriesId::new("first");
        let series_id2 = SeriesId::new("second");

        let input = SeriesSpec::new(
            DataType::U,
            Country::Australia,
            series_id.clone(),
        );

        let input2 = SeriesSpec::new(
            DataType::U,
            Country::Australia,
            series_id2.clone(),
        );

        let mut map = SeriesSpecMap::new();
        map.insert(&input);
        map.insert(&input2);
        let bt = map.get_inner_map(&(DataType::U, Country::Australia)).unwrap();
        assert_eq!(bt.get(&series_id).unwrap(), &input);
        assert_eq!(bt.get(&series_id2).unwrap(), &input2);
    }

    #[test]
    fn inserts_should_be_ordered() {
        let series_id = SeriesId::new("first");
        let series_id2 = SeriesId::new("second");

        let input = SeriesSpec::new(
            DataType::U,
            Country::Australia,
            series_id.clone(),
        );

        let input2 = SeriesSpec::new(
            DataType::U,
            Country::Australia,
            series_id2.clone(),
        );

        let mut map = SeriesSpecMap::new();
        // Insert in reverse order
        map.insert(&input2);
        map.insert(&input);
        let bt = map.get_inner_map(&(DataType::U, Country::Australia)).unwrap();
        let mut iter = bt.iter();
        assert_eq!(iter.next().unwrap().1, &input);
        assert_eq!(iter.next().unwrap().1, &input2);
    }
}
