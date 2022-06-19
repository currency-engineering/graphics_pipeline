//! Deserialize filter specification into [`FilterSpec`](struct.FilterSpec.html).

use anyhow::{anyhow, Result};
use crate::{
    countries::Country,
    file_resources::IntoResources,
    file_resources::impls::Spec,
    primitives::DataType,
};
use key_tree::{KeyTree, KeyTreeError};
use std::{convert::TryInto, ffi::OsStr, path::Path};

/// Return the data-structures representing a filter specification.
/// Return the data-structures representing a source specification.
/// ```
/// # use graphics_pipeline::filter_spec::filter_spec_from_file;
///
/// let _ = filter_spec_from_file("../../shared_data", "filter_spec.keytree").unwrap();
/// ```
pub fn filter_spec_from_file<S, P>(data_root: P, file: S) -> Result<FilterSpec>
where
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let path = Spec.full_path(data_root, file)?;
    KeyTree::parse(&path)?.try_into().map_err(|_| anyhow!("File {} not found", path.display()))
}

// === FilterSpec =================================================================================

/// A specification of what tags and filters to use to select Fred series, which are then
/// converted into a generic data specification.
///
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::filter_spec::FilterSpec;
///   let s = "
///       selectors:
///           series:
///               country:    Australia
///               data_type:  u
///               tag:        unemployment
///               exclude:    Male
///               exclude:    Female
///               exclude:    55-64
///               exclude:    25-54
///               exclude:    15-24
///               exclude:    20 to 24
///               exclude:    Youth
///               exclude:    Women
///               exclude:    Teenagers
///               require:    Rate
///       
///           series:
///               country:    Austria
///               data_type:  u
///               tag:        unemployment
///               exclude:    Male
///               exclude:    Female
///               exclude:    55-64
///               exclude:    25-54
///               exclude:    15-24";
/// # let _: FilterSpec = KeyTree::parse_str(s).unwrap().try_into().unwrap();
/// ```
pub struct FilterSpec(Vec<TagSelector>);

impl FilterSpec {
    pub fn iter<'a>(&'a self) -> FilterSpecIter<'a> {
        FilterSpecIter {
            filter_spec: &self,
            count: 0,
            len: self.0.len(),
        }
    }
}

impl TryInto<FilterSpec> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> std::result::Result<FilterSpec, Self::Error> {
        dbg!();
        let v: Vec<TagSelector> = self.opt_vec_at("selectors::series")?;
        dbg!();
        Ok(FilterSpec(v))
    }
}

// === FilterSpecIter =============================================================================

pub struct FilterSpecIter<'a>{
    filter_spec: &'a FilterSpec,
    count: usize,
    len: usize,
}

impl<'a> Iterator for FilterSpecIter<'a> {
    type Item = &'a TagSelector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.len {
            None
        } else {
            self.count += 1;
            Some(&self.filter_spec.0[self.count - 1])
        }
    }
}

// === TagSelector ================================================================================

/// Specification for how to select data series from Fred. A component of
/// [`FredSeriesFilter`](struct.FredSeriesFilter.html).
/// ```
/// # use key_tree::KeyTree;
/// # use graphics_pipeline::filter_spec::TagSelector;
///   let s = "
///       series:
///           country:    France
///           data_type:  u
///           tag:        unemployment
///           exclude:    Male
///           exclude:    Men
///           exclude:    Female
///           exclude:    Women
///           exclude:    Youth
///           exclude:    Teenagers
///           exclude:    15-24
///           exclude:    15-64
///           exclude:    25-54
///           exclude:    55-64
///           require:    Rate";
/// # let _: TagSelector = KeyTree::parse_str(s).unwrap().try_into().unwrap();
/// ```
#[derive(Debug)]
pub struct TagSelector {
    pub (crate) country:    Country,
    pub (crate) data_type:  DataType,
    pub (crate) tags:       Vec<String>,
    pub (crate) enumerate:  Vec<String>,
    pub (crate) exclude:    Vec<String>,
    pub (crate) require:    Vec<String>,
}

impl TryInto<TagSelector> for KeyTree {
    type Error = KeyTreeError;

    fn try_into(self) -> Result<TagSelector, Self::Error> {
        Ok(
            TagSelector {
                country:    self.from_str("series::country")?,
                data_type:  self.from_str("series::data_type")?,
                tags:       self.opt_vec_from_str("series::tag")?,
                enumerate:  self.opt_vec_from_str("series::enumerate")?,
                exclude:    self.opt_vec_from_str("series::exclude")?,
                require:    self.opt_vec_from_str("series::require")?,
            }
        )
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    #[test]
    fn read_spec_should_fail_if_file_missing() {
        if let Err(e) = super::filter_spec_from_file("../../shared_data", "missing") {
            assert_eq!(
                e.to_string(),
                "File 'missing' not found in '/home/eric/currency.engineering/shared_data/specs'",
            );
        }
    }

    #[test]
    fn read_spec_should_fail_if_contents_dir_missing() {
        if let Err(e) = super::filter_spec_from_file("../missing", "anything") {
            assert_eq!(
                e.to_string(), "Directory '../missing/specs' not found")
        }
    }

    #[test]
    fn read_spec_should_work() {
        if let Ok(_) = super::filter_spec_from_file(
            &PathBuf::from("filter.keytree"),
            &PathBuf::from("./test_contents")
        ) { 
            assert!(true)
        }
    }
}
