use crate::{
    countries::Country,
    primitives::{DataType, SeriesId},
    series_spec::{SeriesSpec},
};

use std::collections::BTreeMap;

// === SeriesSpecMap ==============================================================================

// `SeriesSpecMap` is set up in what seems like an overly complex way in order to maintain the
// order is which data is `SeriesSpecMap` orders the series. We want to maintain an ordering by
// (DataType, Country). But sometimes we also need to search by SeriesId so also need a reverse
// lookup.
//
// `SeriesSpecMap` is used by all the steps from the start to saving data, and therefore by several
// crates.

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
     
    // How are they being ordered?

    #[test]
    fn more_tests_here() {
        assert!(false)
    }
}
