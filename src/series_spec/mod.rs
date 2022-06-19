use anyhow::Result;
use crate::{
    countries::Country,
    file_resources::IntoResources,
    file_resources::impls::FredDataSpec,
    primitives::{DataType, SeriesId},
};
use key_tree::{KeyTree, KeyTreeError};
use std::path::{Path, PathBuf};

pub fn series_spec_from_file<P: AsRef<Path>>(data_root: P, file: P) -> Result<SeriesSpec> {
    let root: PathBuf = data_root.as_ref().to_path_buf();
    let spec_file: PathBuf = file.as_ref().to_path_buf();

    let spec_path = FredDataSpec.full_path(root, spec_file)?;

    let spec: SeriesSpec = KeyTree::parse(spec_path)?.try_into()?;
    Ok(spec)
}

/// Return the deserialization of a series specification.
/// ```
/// # use graphics_pipeline::series_spec::series_spec_from_file;
/// # use std::path::PathBuf;
///
/// if let Ok(_spec) = series_spec_from_file(
///     &PathBuf::from("series_spec.keytree"),
///     &PathBuf::from("./test_contents")
/// ) {
///     assert!(true);
/// }
/// ```

// impl SpecFromFile for SeriesSpec {}

/// Deserialization of a series specification.
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::series_spec::SeriesSpec;
/// # let s = r#"
///       seriess:
///           series:
///               data_type:          u
///               country:            Australia
///               series_id:          AUSURAMS
///           series:
///               data_type:          u
///               country:            Australia
///               series_id:          AUSURANAA
/// # "#;
/// # let _: SeriesSpec = KeyTree::parse(s).unwrap().try_into().unwrap();
/// ```
pub struct SeriesSpec {
    pub (crate) series: Vec<Series>
}

impl TryInto<SeriesSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<SeriesSpec, Self::Error> {
        Ok(SeriesSpec { series: self.opt_vec_at("seriess::series")? })
    }
}

/// A component of [`SeriesSpec`](struct.SeriesSpec.html].
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::series_spec::Series;
/// # let s = r#"
///       series:
///           data_type:          u
///           country:            Australia
///           series_id:          AUSURAMS
/// # "#;
/// # let _: Series = KeyTree::parse(s).unwrap().try_into().unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Series {
    pub(crate) data_type:   DataType,
    pub(crate) country:     Country,
    pub(crate) series_id:   SeriesId,
}

/// ```text
/// seriess:
///     series:
///         country:    United States
///         data_type:  int
///         series_id:  DPRIME
///     ..
/// ```
impl TryInto<Series> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<Series, Self::Error> {
        Ok(
            Series {
                country:    self.from_str("series::country")?,
                data_type:  self.from_str("series::data_type")?, 
                series_id:  self.from_str("series::series_id")?,
            }
        )
    }
}
