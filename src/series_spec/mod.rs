use anyhow::Result;
use crate::{
    countries::Country,
    file_resources::IntoResources,
    file_resources::impls::Spec,
    primitives::{DataType, SeriesId},
};
use key_tree::{KeyTree, KeyTreeError};
use std::path::{Path, PathBuf};

pub fn series_spec_from_file<P: AsRef<Path>>(data_root: P, file: P) -> Result<SeriessSpec> {
    let root: PathBuf = data_root.as_ref().to_path_buf();
    let spec_file: PathBuf = file.as_ref().to_path_buf();

    let spec_path = Spec.full_path(root, spec_file)?;

    let spec: SeriessSpec = KeyTree::parse(spec_path)?.try_into()?;
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

// === SeriessSpec ================================================================================

/// Represents a set of series as it moves through the graphics pipeline. It includes data on how
/// to transforms.
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::series_spec::SeriessSpec;
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
/// let spec: SeriessSpec = KeyTree::parse_str(s).unwrap().try_into().unwrap();
/// ```
#[derive(Debug)]
pub struct SeriessSpec {
    pub(crate) series: Vec<SeriesSpec>
}

impl SeriessSpec {
    pub(crate) fn iter(&self) -> SeriessSpecIter {
        SeriessSpecIter {
            data: &self,
            count: 0,
        }
    }
}

impl TryInto<SeriessSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<SeriessSpec, Self::Error> {
        Ok(SeriessSpec { series: self.opt_vec_at("seriess::series")? })
    }
}

// === SeriessSpecIter ============================================================================

pub struct SeriessSpecIter<'a> {
    data: &'a SeriessSpec,
    count: usize,
}

impl<'a> Iterator for SeriessSpecIter<'a> {
    type Item = SeriesSpec;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.data.series.len() {
            return None
        } else {
            self.count += 1;
            return Some(self.data.series[self.count - 1].clone())
        }
    }
}

// === SeriessSpec ================================================================================

/// A component of [`SeriesSpec`](struct.SeriesSpec.html].
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::series_spec::SeriesSpec;
/// # let s = r#"
///       series:
///           data_type:          u
///           country:            Australia
///           series_id:          AUSURAMS
/// # "#;
/// # let _: SeriesSpec = KeyTree::parse_str(s).unwrap().try_into().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SeriesSpec {
    data_type:   DataType,
    country:     Country,
    series_id:   SeriesId,
}

impl SeriesSpec {
    pub(crate) fn new(data_type: DataType, country: Country, series_id: SeriesId) -> Self {
        SeriesSpec {
            data_type,
            country,
            series_id,
        }
    }

    pub(crate) fn data_type(&self) -> DataType {
        self.data_type
    }

    pub(crate) fn country(&self) -> Country {
        self.country
    }

    pub(crate) fn series_id(&self) -> SeriesId {
        self.series_id.clone()
    }
}

/// ```text
/// seriess:
///     series:
///         country:    United States
///         data_type:  int
///         series_id:  DPRIME
///     ..
/// ```
impl TryInto<SeriesSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<SeriesSpec, Self::Error> {
        Ok(
            SeriesSpec {
                country:    self.from_str("series::country")?,
                data_type:  self.from_str("series::data_type")?, 
                series_id:  self.from_str("series::series_id")?,
            }
        )
    }
}

#[cfg(test)]
pub mod test {

    use key_tree::KeyTree;
    use crate::series_spec::SeriessSpec;

    #[test]
    fn spec_from_keytree_should_work() {

        let s = r#"
            seriess:
                series:
                    data_type:          u
                    country:            Australia
                    series_id:          AUSURAMS
                series:
                    data_type:          u
                    country:            Australia
                    series_id:          AUSURANAA
        "#;
        dbg!(KeyTree::parse_str(s).unwrap());
        let spec: SeriessSpec = KeyTree::parse_str(s).unwrap().try_into().unwrap();
        let mut iter = spec.iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
    }
}
