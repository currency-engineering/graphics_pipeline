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



#[cfg(test)]
pub mod tests {

    #[test]
    fn series_in_spec_should_also_be_in_transformed_data() {
    // get a series from series_spec
    // check if it exists in /transformed_data
    }
}



