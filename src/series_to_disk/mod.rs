use crate::{
    countries::Country,
    primitives::{DataType, SeriesId},
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

/// A specification of all the series to be downloaded from FRED. Output looks like
///seriess:
/// ```text
/// seriess:
///     series:
///         data_type:          u
///         country:            Australia
///         series_id:          AUSURAMS
///     series:
///         data_type:          u
///         country:            Australia
///         series_id:          AUSURANAA
///     series:
///         data_type:          u
///         country:            Australia
///         series_id:          AUSURAQS
///     series:
///         data_type:          u
///         country:            Australia
///         series_id:          AUSURHARMADSMEI
///     series:
///         data_type:          u
///         country:            Australia
///         series_id:          AUSURHARMMDSMEI
/// ```
#[derive(Debug)]
pub struct SeriesSpecMap {
    map: BTreeMap<(DataType, Country), Vec<SeriesSpec>>,
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
        let seriess = self.map.get(&key).unwrap();      

        match seriess.iter().find(|&series| {
            &series.series_id == series_id
        }) {
            Some(series_spec) => Some(series_spec.clone()),
            None => None,
        } 
    }

    /// Insert a `SeriesSpec`.
    pub (crate) fn insert(&mut self, series_spec: &SeriesSpec) {

        match self.map.get_mut(&(series_spec.data_type, series_spec.country)) {

            // No existing key so insert key and value.
            None => {
                self.map.insert(
                    (series_spec.data_type, series_spec.country),
                    vec!(series_spec.clone())
                );

                self.reverse.insert(
                    series_spec.series_id.clone(),
                    (series_spec.data_type, series_spec.country),
                );
            },

            // Key already exists so push to value.
            Some(value) => {
                value.push(series_spec.clone());

                self.reverse.insert(
                    series_spec.series_id.clone(),
                    (series_spec.data_type, series_spec.country),
                );
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
            map.insert(&series_spec)
        }
        map
    }
}


    // // /// Append `other` to `Self`.
    // // pub fn append(&mut self, other: &mut SeriesSpecMap) {
    // //     self.0.append(&mut other.0)
    // // }

    // // fn series_from_index(&self, i: usize) -> &Series {
    // //     &self.0[i]
    // // }

    // /// Read in keytree data from file.
    // pub fn from_file(path: &Path) -> Result<Self> {

    //     let source_spec = fs::read_to_string(path)?;

    //     let kt = KeyTree::parse(&source_spec, Some(path.to_str().unwrap())).map_err::<Error, _>(|e| e.into())?;

    //     let res_data_spec: std::result::Result<_, Error> = kt.to_ref().try_into();

    //     res_data_spec.map_err(|e| e.into())
    // }

    // /// Save FRED data to disk as csv, using full path. Will fail if an existing filepath is
    // /// encountered.
    // /// ```
    // /// let mut source = SeriesSpecMap::from_file("checked_data.keytree");
    // /// source.write(&root_dir);
    // /// ```
    // /// To path from root is "/{data_type}/{country}/LRUNTTTTSIQ156S.csv"
    // /// 
    // pub fn write(&self, root_path: &Path) -> Result<()> {
    //     for (_, series_specs) in &self.map {
    //         for series_spec in series_specs {
    //             series_spec.write_data_to_file(root_path)?;
    //             series_spec.write_meta_to_file(root_path)?; 
    //         }
    //     }
    //     Ok(())
    // }

    // // Need to keep a record of all files and remove files that shouldn't exist.
    // // 
    // // i.e. its not sufficient to check if a file exists. We need to check if a file remains. So we
    // // need to create a file list.

    // /// Only make requests and updata data to Fred for files that are in `SeriesSpecMap` but do not exist
    // /// as data files.
    // pub fn update_write(&self, root_path: &Path) -> Result<()> {
    //     for (_, series_specs) in &self.map {
    //         for series_spec in series_specs {
    //             if !series_spec.exists(&series_spec.country(), root_path)? {
    //                 series_spec.write_data_to_file(root_path)?;
    //                 series_spec.write_meta_to_file(root_path)?;
    //             }
    //         }
    //     }
    //     self.remove_old(root_path)?;
    //     Ok(())
    // }

    // /// Run through data files, query and remove any files that are not in directory.
    // pub fn remove_old(&self, root_path: &Path) -> Result<()> {

    //     for entry in WalkDir::new(root_path) {

    //         let entry = entry.unwrap();

    //         if !entry.file_type().is_dir() {

    //             let pathbuf = entry.path(); 

    //             let mut path_iter = pathbuf
    //                 .iter()
    //                 .rev()
    //                 .map(|os_str| os_str.to_str()
    //                 .unwrap());

    //             let mut file_parts = path_iter
    //                 .next()
    //                 .unwrap()
    //                 .split('.');

    //             let file_stem = file_parts.next().unwrap();
    //             let file_ext = file_parts.next().unwrap();

    //             if file_ext == "csv" || file_ext == "meta" {

    //                 let series_id = SeriesId::new(file_stem);

    //                 match self.get_series_spec(&series_id) {
    //                     Some(_) => {},
    //                     None => {
    //                         println!("remove file: {}", entry.path().display());
    //                         // match fs::remove_file(entry.path()) {
    //                         //     Ok(_) => {},
    //                         //     Err(_) => {
    //                         //         return Err(file_error(file!(), line!()))
    //                         //      },
    //                         // }
    //                     },
    //                 }
    //             }
    //         };
    //     }
    //     Ok(())
    // }

    // /// Same as `write()` except that it starts in the specification at `series_id`. Useful if there
    // /// is a break in the connection when writing.
    // pub fn resume_write(&self, series_id: &str, root_path: &Path) -> Result<()> {
    //     let sid = SeriesId::new(series_id);
    //     for (_, series_specs) in self
    //         .map
    //         .iter()
    //         .skip_while(|_| {
    //             match self.get_series_spec(&sid) {
    //                 Some(series_spec) => sid != series_spec.series_id,
    //                 None => true,
    //             }
    //         })
    //     {
    //         for series_spec in series_specs {
    //             series_spec.write_data_to_file(root_path)?;
    //             series_spec.write_meta_to_file(root_path)?;
    //         }
    //     }
    //     Ok(())
    // }
// }

// impl<'a> TryInto<SeriesSpecMap> for KeyTreeRef<'a> {
//     type Error = Error;
// 
//     fn try_into(self) -> std::result::Result<SeriesSpecMap, Self::Error> {
//         let v: Vec<SeriesSpec> = self.vec_at("seriess::series")?;
// 
//         Ok(SeriesSpecMap::from_vec(v))
//     }
// }

// impl IntoKeyTree for SeriesSpecMap {
//     fn keytree(&self) -> KeyTreeString {
//         let mut kt = KeyTreeString::new();
//         kt.push_key(0, "seriess");
//         for (_, series_specs) in &self.map {
//             for series_spec in series_specs {
//                 kt.push_keytree(1, series_spec.keytree());
//             }
//         }
//         kt
//     }
// }

