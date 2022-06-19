//! #### Usage Examples
//!
//! ```
//! # use graphics_pipeline::primitives::DataType;
//! # use graphics_pipeline::countries::Country;
//! # use graphics_pipeline::file_resources::IntoResources;
//! # use graphics_pipeline::file_resources::impls::CsvRawData;
//! let cd = CsvRawData {
//!     country: Country::Australia,
//!     data_type: DataType::U,
//! };
//! let _resources = cd.into_resources("../../shared_data").unwrap();
//! assert!(cd.has_file("../../shared_data", "LRHUTTTTAUA156N.csv").unwrap());
//! ```
//!
//! #### Implementation Example
//!
//! This implementation looks in the path `/pid_graphics/js`, and fails if there are other files in
//! this path.
//!
//! ```ignore
//! pub struct PidGraphicsJs;
//! 
//! impl IntoResources for PidGraphicsJs {
//!     fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
//!         join_paths(data_root, vec!("pid_graphics", "js"))
//!     }
//! 
//!     fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
//!         let root = from_path_arg(data_root);
//!         let all = self.all_files_in_dir(&root)?;
//!         all.only_ext(vec!["js"])?;
//!         Ok(all.filter_by_ext(vec!["js"]))
//!     }
//! }
//! ```
//!
//! This example sets up paths depending on the values in  `CsvData`. It allows "meta" files in the
//! path, by no other files.
//!
//! ```ignore
//! #[derive(Debug)]
//! pub struct CsvData {
//!     pub country: Country,
//!     pub data_type: DataType,
//! }
//! 
//! impl IntoResources for CsvData {
//!     fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
//!         let root: PathBuf = from_path_arg(data_root);
//!         let path: PathBuf = root
//!             .join("data")
//!             .join(self.data_type.to_string())
//!             .join(self.country.as_filepath());
//! 
//!         path
//!             .canonicalize()
//!             .map_err(|_| anyhow!("File [{}] not found", path.display()))
//!     }
//! 
//!     fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
//!         let root: PathBuf = from_path_arg(data_root);
//!         let all: Resources = self.all_files_in_dir(&root)?;
//! 
//!         all.only_ext(vec!["csv", "meta"])?;
//!         Ok(all.filter_by_ext(vec!["csv"]))
//!     }
//! }
//! ```

pub mod impls;

use anyhow::{anyhow, bail, Result};
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, fs};

// === Helper functions ===========================================================================

/// Useful for converting a `Path` or a `&str` into a PathBuf.
pub fn from_path_arg<P: AsRef<Path>>(p: P) -> PathBuf {
    p.as_ref().to_path_buf()
}

/// Joins paths and also checks that this path exists. 
pub fn join_paths<P: AsRef<Path>>(root: P, paths: Vec<&str>) -> Result<PathBuf> {
    let mut path = from_path_arg(root);
    for s in paths {
        path = path.join(s);
    }
    path  
        .canonicalize()
        .map_err(|_| anyhow!("Directory '{}' not found", path.display()))
}

/// Returns true if the path has the right extension.
/// ```
/// # use std::path::Path;
/// # use graphics_pipeline::file_resources::extension_is;
/// assert!(extension_is(Path::new("/australia/u/LRUN.csv"), "csv"));
/// ```
pub fn extension_is(path: &Path, extension: &str) -> bool {
    match path.extension() {
        Some(ext) => { ext == OsStr::new(extension) },
        None => false,
    }
}

// === ResourceIter ===============================================================================

pub struct ResourcesIter<'a> {
    data: &'a Resources,
    count: usize
}

impl<'a> Iterator for ResourcesIter<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.data.0.len() {
            return None
        } else {
            self.count += 1;
            Some(self.data.0[self.count - 1].clone())
        }
    }
}

// === Resources ==================================================================================

/// A collect of files of a given type.
#[derive(Debug)]
pub struct Resources(Vec<PathBuf>);

impl FromIterator<PathBuf> for Resources {

    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = PathBuf>
    {
        let v: Vec<PathBuf> = iter.into_iter().collect();
        Resources(v)
    }
}

impl Resources {
    pub fn iter(&self) -> ResourcesIter {
        ResourcesIter {
            data: &self,
            count: 0,
        }
    }

    pub fn filter_by_ext(&self, ext: Vec<&str>) -> Resources {
        let mut acc = Vec::new();

        for pb in self.iter() {
            let ext_str = match pb.extension() {
                Some(s) => match s.to_str() {
                    Some(s) => s,
                    None => continue,
                },
                None => continue,
            };
            if ext.contains(&ext_str) {
                acc.push(pb)
            }
        }
        Resources(acc)
    }

    pub fn only_ext(&self, ext: Vec<&str>) -> Result<()> {
        for pb in self.iter() {
            let ext_str = match pb.extension() {
                Some(s) => match s.to_str() {
                    Some(s) => s,
                    None => continue,
                },
                None => continue,
            };
            if !ext.contains(&ext_str) {
                bail!(
                    "Directory '{}' contained a file with extension '{}'",
                    self.0[0].parent().unwrap().display(),
                    ext_str
                )
            }
        }
        Ok(())
    }
}

// This should be implemented as a trait rather than as an enum, because the different resource
// types are open-ended.

///A trait for that takes a resource type and a path to the root contents director and returns a
/// `Resources`.
pub trait IntoResources {
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources>;

    /// Given a path to the root directory, return the directory of of the file.
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::PidGraphicsJs;
    /// # use std::path::PathBuf;
    /// assert!(
    ///     PidGraphicsJs.dir("../../shared_data")
    ///         .unwrap()
    ///         .ends_with("shared_data/pid_graphics/js")
    /// );
    /// ```
    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf>;

    /// Given a path to the root directory and a filename, return a `String` of the contents of the
    /// file.
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::PidGraphicsJs;
    /// let pid_js = PidGraphicsJs.from_file("../../shared_data", "test.js");
    /// ```
   fn from_file<P, S>(&self, data_root: P, file: S) -> Result<String>
   where
        S: AsRef<OsStr>,
        P: AsRef<Path>,
    {
        let f = file.as_ref();

        let root: PathBuf = data_root.as_ref().to_path_buf();
        let dir = self.dir(&data_root)?;

        let resources = self.into_resources(&root)?;

        match resources.iter().find(|pb| pb.ends_with(f)) {
            Some(found) => {
                fs::read_to_string(&found)
                    .map_err(|_| anyhow!( "File [{}] not found in [{}]", &found.to_str().unwrap(), dir.display()))
            },
            None => {
                Err(anyhow!("File '{}' not found in '{}'", f.to_str().unwrap(), dir.display()))
            },
        }
    }

    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::FredDataSpec;
    /// # use std::path::PathBuf;
    /// let spec: PathBuf = FredDataSpec.full_path("../../shared_data", "series_spec.keytree").unwrap();
    /// ```
   fn full_path<P, S>(&self, data_root: P, file: S) -> Result<PathBuf>
   where
        S: AsRef<OsStr>,
        P: AsRef<Path>,
    {
        let f = file.as_ref();
        let dir = self.dir(&data_root)?;

        dir
            .join(f)
            .canonicalize()
            .map_err(|_| anyhow!("File '{}' not found in '{}'", f.to_str().unwrap(), dir.display()))
    }

    /// Verify that a file is in `Resources`.
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::PidGraphicsJs; 
    /// assert!(PidGraphicsJs.has_file("../../shared_data", "test.js").unwrap());
    /// ```
    fn has_file<S: AsRef<OsStr>, P: AsRef<Path>>(
        &self,
        data_root: P,
        file: S) -> Result<bool>
    {
        let f = file.as_ref();
        let root: PathBuf = from_path_arg(data_root);

        // let dir = self.dir(&root)?;
        let resources = self.into_resources(root)?;
        Ok(
            resources.iter()
                .any(|pb| pb.ends_with(f))
        )
    }

    /// Return all files in the directory.  
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::PidGraphicsJs; 
    /// let _all = PidGraphicsJs.all_files_in_dir("../../shared_data").unwrap();
    /// ```
    fn all_files_in_dir<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let root = from_path_arg(data_root);
        let dir = self.dir(root)?;

        let mut acc = Vec::new();
        for res_entry in std::fs::read_dir(&dir)? {
            let entry = res_entry?;
            let pb = entry.path();
            acc.push(pb);
        }
        Ok(acc.into_iter().collect())
    }
}

// === AllMetaData ================================================================================

// /// Return all the metadata types from
// pub fn all_metadata<P: AsRef<Path>>(data_root: P) -> Result<Vec<DataType>> {
//     let root: PathBuf = data_root.as_ref().to_path_buf();
// 
//     let series_spec = series_spec_from_file(root, "series_spec.keytree")?;
// 
//     Ok(Vec::new())
// }

