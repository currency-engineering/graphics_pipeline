// A thing that implements `IntoResources`. Its purpose is to take responsibility for finding files
// in the contents directory. For example if it is a CssResourceType it knows where to find CSS 
// files, to check that the directories it is using are clean, and to pass on all the paths to the
// `Resources` type. The interface for a request for files is then the resource type and a path to
// the contents directory root.

use anyhow::{anyhow, bail, Result};
use crate::{
    countries::Country,
    primitives::DataType,
    file_resources::from_path_arg,
    file_resources::IntoResources,
    file_resources::join_paths,
    file_resources::Resources,
};
use std::{path::{Path, PathBuf}};

// === PidGraphicCss ============================================================================

/// All CSS files.
pub struct PidGraphicCss;

impl IntoResources for PidGraphicCss {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        join_paths(data_root, vec!("pid_graphics", "css"))
    }

    /// Takes a path to the root contents and returns the "style.css" file.
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let f = self.dir(data_root)?.join("style.css");
        Ok(Resources(vec!(f)))
    }

}

// === CsvRawData ============================================================================

/// All CSV data files.
#[derive(Debug)]
pub struct CsvRawData {
    pub country: Country,
    pub data_type: DataType,
}

impl IntoResources for CsvRawData {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        let root: PathBuf = from_path_arg(data_root);
        let path: PathBuf = root
            .join("raw_data")
            .join(self.data_type.to_string())
            .join(self.country.as_filepath());

        path
            .canonicalize()
            .map_err(|_| anyhow!("File '{}' not found", path.display()))
    }

    /// ```
    /// # use graphics_pipeline::primitives::DataType;
    /// # use graphics_pipeline::countries::Country;
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::CsvRawData;
    /// let cd = CsvRawData {
    ///     country: Country::Australia,
    ///     data_type: DataType::U,
    /// };
    /// let _resources = cd.into_resources("../../shared_data").unwrap();
    /// assert!(cd.has_file("../../shared_data", "LRHUTTTTAUA156N.csv").unwrap());
    /// ```
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let root: PathBuf = from_path_arg(data_root);
        let all: Resources = self.all_files_in_dir(&root)?;

        all.only_ext(vec!["csv", "meta"])?;
        Ok(all.filter_by_ext(vec!["csv"]))
    }
}

// === CsvTransformedData ============================================================================

/// All CSV data files.
#[derive(Debug)]
pub struct CsvTransformedData {
    pub country: Country,
    pub data_type: DataType,
}

impl IntoResources for CsvTransformedData {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        let root: PathBuf = from_path_arg(data_root);
        let path: PathBuf = root
            .join("transformed_data")
            .join(self.data_type.to_string())
            .join(self.country.as_filepath());

        path
            .canonicalize()
            .map_err(|_| anyhow!("File '{}' not found", path.display()))
    }

    /// ```
    /// # use graphics_pipeline::primitives::DataType;
    /// # use graphics_pipeline::countries::Country;
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::CsvTransformedData;
    /// let cd = CsvTransformedData {
    ///     country: Country::Australia,
    ///     data_type: DataType::U,
    /// };
    /// let _resources = cd.into_resources("../../shared_data").unwrap();
    /// // assert!(cd.has_file("../../shared_data", "LRHUTTTTAUA156N.csv").unwrap());
    /// ```
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let root: PathBuf = from_path_arg(data_root);
        let all: Resources = self.all_files_in_dir(&root)?;

        all.only_ext(vec!["csv", "meta"])?;
        Ok(all.filter_by_ext(vec!["csv"]))
    }
}

// === MetaData ===================================================================================

/// All metadata files.
pub struct MetaData {
    pub country: Country,
    pub data_type: DataType,
}

impl IntoResources for MetaData {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        let root: PathBuf = data_root.as_ref().to_path_buf();
        let path = root
            .join("raw_data")
            .join(self.data_type.to_string())
            .join(self.country.as_filepath());

        path
            .canonicalize()
            .map_err(|_| anyhow!("Directory '{}' not found", path.display()))
    }

    /// A collection of files of one type.
    /// ```
    /// # use graphics_pipeline::primitives::DataType;
    /// # use graphics_pipeline::countries::Country;
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::MetaData;
    /// let md = MetaData {
    ///     country: Country::Australia,
    ///     data_type: DataType::U,
    /// };
    /// let _resources = md.into_resources("../../shared_data").unwrap();
    /// assert!(md.has_file("../../shared_data", "LRHUTTTTAUA156N.meta").unwrap())
    /// ```
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let root = from_path_arg(data_root);
        let all = self.all_files_in_dir(&root)?;

        all.only_ext(vec!["csv", "meta"])?;

        Ok(all.filter_by_ext(vec!["meta"]))
    }
}

// === Spec =======================================================================================

#[derive(Debug)]
pub struct Spec;

impl IntoResources for Spec {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        join_paths(data_root, vec!("specs"))
    }

    /// A collection of files of one type.
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let mut acc = Vec::new();

        let dir = self.dir(data_root)?;

        for res_entry in std::fs::read_dir(&dir)? {
            let entry = res_entry?;
            let pb = entry.path();

            // Accept ".spec" files in the directory
            if pb.extension() == Some("keytree".as_ref()) { 
                acc.push(pb);
                continue;
            }
        
            bail!("{} is not of type .keytree", pb.display());
        }
        Ok(acc.into_iter().collect())
    }
}

// === PidGraphicsFavIcon ========================================================================

/// The favicon file.
pub struct PidGraphicsFavIcon;

impl IntoResources for PidGraphicsFavIcon {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        join_paths(data_root, vec!("pid_graphics", "favicon"))
    }

    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let root: PathBuf = from_path_arg(data_root);
        let dir = self.dir(&root)?;
        let pb = dir.join("favicon.png");
        Ok(Resources(vec!(pb)))
    }
}

// === PidGraphicsJS ========================================================================

/// All Javascript helper files
pub struct PidGraphicsJs;

impl IntoResources for PidGraphicsJs {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        join_paths(data_root, vec!("pid_graphics", "js"))
    }

    /// Build `Resources` for a GraphicsJs.
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::PidGraphicsJs;
    /// let js = PidGraphicsJs.into_resources("../../shared_data").unwrap();
    /// // assert_eq!(js.into_iter().next().unwrap().extension().unwrap(), "js");
    /// ```
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {
        let root = from_path_arg(data_root);
        let all = self.all_files_in_dir(&root)?;
        all.only_ext(vec!["js"])?;
        Ok(all.filter_by_ext(vec!["js"]))
    }
}

// === TSPageSpec ========================================================================

pub struct TSPageSpec;

impl IntoResources for TSPageSpec {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        join_paths(data_root, vec!("ts_graphics", "spec"))
    }

    /// Build `Resources` for a GraphicsJs.
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::PidGraphicsJs;
    /// let js = PidGraphicsJs.into_resources("../../shared_data").unwrap();
    /// // assert_eq!(js.into_iter().next().unwrap().extension().unwrap(), "js");
    /// ```
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {

        let dir = self.dir(data_root)?;

        let mut acc = Vec::new();
        for res_entry in std::fs::read_dir(&dir)? {
            let entry = res_entry?;
            let pb = entry.path();

            if pb.extension() == Some("keytree".as_ref()) {
                acc.push(pb);
                continue;
            }
            // Reject all other file types.
            bail!("{} is not of type MetaData", pb.display());
        }
        Ok(acc.into_iter().collect())
    }
}

pub struct TSGraphicsJs;

impl IntoResources for TSGraphicsJs {

    fn dir<P: AsRef<Path>>(&self, data_root: P) -> Result<PathBuf> {
        join_paths(data_root, vec!("ts_graphics", "js"))
    }

    /// Build `Resources` for a GraphicsJs.
    /// ```
    /// # use graphics_pipeline::file_resources::IntoResources;
    /// # use graphics_pipeline::file_resources::impls::TSGraphicsJs;
    /// let js = TSGraphicsJs.into_resources("../../shared_data").unwrap();
    /// // assert_eq!(js.into_iter().next().unwrap().extension().unwrap(), "js");
    /// ```
    fn into_resources<P: AsRef<Path>>(&self, data_root: P) -> Result<Resources> {

        let dir = self.dir(data_root)?;

        let mut acc = Vec::new();
        for res_entry in std::fs::read_dir(&dir)? {
            let entry = res_entry?;
            let pb = entry.path();

            if pb.extension() == Some("js".as_ref()) {
                acc.push(pb);
                continue;
            }
            // Reject all other file types.
            bail!("{} is not of type MetaData", pb.display());
        }
        Ok(acc.into_iter().collect())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        countries::Country,
        primitives::DataType,
        file_resources::IntoResources,
    };

    #[test]
    fn pid_graphic_js_should_have_file() {
        assert!(PidGraphicsJs.has_file("../../shared_data", "test.js").unwrap());
    }

    #[test]
    fn pic_graphic_js_should_load_file() {
        if let Ok(s) = PidGraphicsJs.from_file("../../shared_data", "test.js") {
            assert_eq!(s, "some js\n");
        } else {
            assert!(false)
        }
    }

    #[test]
    fn csv_data_should_have_file() {
        let csv_data = CsvRawData {
            country: Country::Belgium,
            data_type: DataType::Inf,
        };
        assert!(csv_data.has_file("../../shared_data", "FPCPITOTLZGBEL.csv").unwrap())
    }

    #[test]
    fn pid_graphics_css_should_have_file() {
        assert!(PidGraphicCss.has_file("../../shared_data", "style.css").unwrap());
    }

    #[test]
    fn meta_data_should_have_file() {
        let meta_data = MetaData {
            country: Country::Japan,
            data_type: DataType::U,
        };
        assert!(meta_data.has_file("../../shared_data", "LRHUTTTTJPM156S.meta").unwrap())
    }

    #[test]
    fn meta_data_should_load() {
        let meta_data = MetaData {
            country: Country::Japan,
            data_type: DataType::U,
        };
        meta_data.from_file("../../shared_data", "LRHUTTTTJPM156S.meta").unwrap();
    }

    #[test]
    fn pid_graphic_fav_icon_should_have_file() {
        assert!(PidGraphicsFavIcon.has_file("../../shared_data", "favicon.png").unwrap());
    }

    #[test]
    fn spec_should_load() {
        Spec.from_file("../../shared_data", "filter_spec.keytree").unwrap();
    }

    #[test]
    fn ts_page_spec_should_load() {
        TSPageSpec.from_file("../../shared_data", "ts_page_spec.keytree").unwrap();
    }

    #[test]
    fn missing_file_should_error() {
        if let Err(e) = Spec.from_file("../../shared_data", "missing") {
            assert_eq!(
                e.to_string(),
                "File 'missing' not found in '/home/eric/currency.engineering/shared_data/specs'",
            );
        } else {
            assert!(false)
        }
    }

    // #[test]
    // fn fred_data_series_spec_from_file() { 
    //     let spec = fred_data::series_spec_from_file("../../shared_data", "series.keytree");
    // }
}
