
pub mod countries;
pub mod data_transforms;
pub mod file_resources;
pub mod filter_spec;
pub mod filter_to_series;

/// Implement actix web data for html. 
// pub mod html_to_server;

pub mod http_state;

pub mod meta_data;

pub mod os_setup;

// pub mod page_spec;

pub mod primitives;

/// KeyTree wrapper for `series_spec.keytree`.
pub mod series_spec;

/// Use `series_spec.keytree` to write retrieve csv data from FRED and save to disk.  
pub mod series_to_disk;

pub mod series_to_meta;

pub mod ts_graphics;

pub mod ui_spec;

// pub mod ui_to_server;
