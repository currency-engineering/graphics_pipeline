//! Use this module to read a filter spec such as `filter_spec.keytree` which is a specification
//! which can generate a list of series that need to be downloaded from FRED economic database.
//! This new list is a source spec such as `source_spec.keytree`.

// Note: We need to be careful with isolating responsibilities.

use anyhow::{Result};
use crate::{
    countries::Country,
    primitives::SeriesId,
    filter_spec::FilterSpec, 
    filter_spec::TagSelector,
    series_spec::{SeriesSpec, SeriessSpec},
    file_resources::IntoResources,
    file_resources::impls::Spec,
};
use fred_api::FredClient;
use key_tree::KeyTree;
use std::{ffi::OsStr, path::{Path, PathBuf}};

/// TODO
pub fn filter_spec_to_generic_source_spec() -> Result<()> {
    unimplemented!()
}

/// These 28 countries are all the countries with good data.
pub fn countries_with_data() -> Vec<Country> {
    vec!(
        Country::Australia,
        Country::Austria,
        Country::Belgium,
        Country::Canada,
        Country::Chile,
        Country::CzechRepublic,
        Country::Denmark,
        Country::Estonia,
        Country::Finland,
        Country::France,
        Country::Germany,
        Country::Greece,
        Country::Ireland,
        Country::Israel,
        Country::Italy,
        Country::Japan,
        Country::Latvia,
        Country::Netherlands,
        Country::NewZealand,
        Country::Norway,
        Country::Poland,
        Country::Serbia,
        Country::SouthKorea,
        Country::Spain,
        Country::Sweden,
        Country::Switzerland,
        Country::UnitedKingdom,
        Country::UnitedStates,
    )
}

/// Takes a filter specification and returns a source specification, printing out details about
/// which series are selected and which are dropped, for example
/// ```ignore
/// let source_spec = source_spec_from_filter_spec("filter_spec.keytree")?;
/// ```
/// The printout looks something like
/// ```text
/// Australia u
/// drop: AUSUEMPNA Adjusted Unemployment in Australia (DISCONTINUED)
/// drop: AUSUR24NAA Adjusted Unemployment Rate for Persons Ages 20 to 24 in Australia (DISCONTINUED)
///       AUSURAMS Adjusted Unemployment Rate in Australia (DISCONTINUED)
///       AUSURAMS Adjusted Unemployment Rate in Australia (DISCONTINUED)
///       AUSURANAA Adjusted Unemployment Rate for Adults in Australia (DISCONTINUED)
/// ```
pub fn series_spec_from_filter_spec<P, S>(file: S, root_data: P) -> Result<SeriessSpec>
where
    P: AsRef<Path>,
    S: AsRef<OsStr>,
{
    let f: &OsStr = file.as_ref();
    let pb: PathBuf = root_data.as_ref().to_path_buf();

    let path = Spec.dir(pb)?.join(f);

    let filter_spec: FilterSpec = KeyTree::parse(path)?.try_into()?;

    let mut acc = Vec::new();

    for tag_selector in filter_spec.iter() {

        println!("");
        println!("{} {}", tag_selector.country, tag_selector.data_type);

        let tag = tag(&tag_selector);

        let series_items = FredClient::tags_series(&tag)?.seriess;

        for series_item in series_items.iter() {

            let series_id = SeriesId::new(&series_item.id.clone());

            if is_selected(&tag_selector, series_item) {
                println!("      {} {}", series_item.id, series_item.title);
                acc.push(SeriesSpec::new(tag_selector.data_type, tag_selector.country, series_id));
                println!("      {} {}", series_item.id, series_item.title);
                
            } else {
                println!("drop: {} {}", series_item.id, series_item.title);
            }
        }
    }
    Ok(SeriessSpec { series: acc })
}

    // /// Takes a [`FredSeriesFilter`](struct.FredSeriesFilter.html) and returns a [`SeriesSpecMap`](struct.SeriesSpecMap.html).
    // pub fn resume_into_data_spec(&self, country: Country, data_type: DataType) -> Result<SeriesSpecMap> {

    //     let mut data_spec = SeriesSpecMap::new();

    //     for tag_selector in self.0.iter().skip_while(|tag_selector| {
    //         tag_selector.country != country|| 
    //         tag_selector.data_type != data_type
    //     }) {
    //     
    //         println!("{} {}", tag_selector.country, tag_selector.data_type);


    //         let series_items = FredClient::tags_series(&tag_selector.tag())?.seriess;

    //         for series_item in series_items.iter() {

    //             let series_id = SeriesId::new(&series_item.id.clone());

    //             if tag_selector.is_selected(series_item) {
    //                 println!("    {} {}", series_item.id, series_item.title);
    //                 let series_spec = SeriesSpec {
    //                     country:    Some(tag_selector.country),
    //                     data_type:  tag_selector.data_type,
    //                     series_id:  series_id,
    //                     transforms: Vec::new(),
    //                     parser:     ParserSpec::FredSpec(FredSpec::new()),
    //                 };
    //                 println!("    {} {}", series_item.id, series_item.title);
    //                 data_spec.insert(&series_spec);
    //                 
    //             }
    //         }
    //     }
    //     Ok(data_spec)
    // }

// /// Get the title of a series from FRED.
// /// ```ignore
// /// println!("{}", title("LFACTTTTKRA657N"));
// /// ```
// pub fn title(series_id: &str) -> Result<String> {
//     let sid = SeriesId::from_str(series_id).unwrap();
//     let mut s = String::new();
// 
//     let seriess = FredClient::series(&sid.to_string())?.seriess;
// 
//     for series in seriess.iter() {
//         s.push_str(&series.title)
//     }
//     Ok(s)
// }

/// To use countries in FredClient tags, some adjustments need to be made over standard country names.
pub fn fred_country(country: Country) -> String {
    match country {
        Country::SouthKorea => "korea".into(),
        Country::UnitedStates => "usa".into(),
        _ => country.to_string().to_lowercase(),
    }
}

/// Return a compiled tag from parts. This is required because we want to keep the `fred_api`
/// flexible,so it takes a `&str` rather than some more strongly typed tags. So we need to build
/// tags outside of the `fred_api` crate.
pub fn tag(tag_selector: &TagSelector) -> String {
    let mut s = String::new();
    for tag in &tag_selector.tags {
        s.push_str(&tag.trim());
        s.push(';');
    }
    s.push_str(&fred_country(tag_selector.country));
    s
}

// Note: originally a method of TagSelector.
// /// Return
// pub fn into_series_items(&self) -> Result<Vec<fred_api::SeriesItem>, Error> {

//     let mut v: Vec<fred_api::SeriesItem> = Vec::new();

//     let series_items = match FredClient::tags_series(&self.tag()) {
//         Ok(tags_series) => {
//             tags_series.seriess
//         },
//         Err(err) => { return Err(failed_fred_request(&err.to_string())) },
//     };

//     for series_item in series_items.iter() {

//         if self.is_selected(series_item) {
//             println!("    {}", series_item.id);
//             v.push(series_item.clone());
//         }
//     }
//     Ok(v)
// }

fn is_selected(tag_selector: &TagSelector, series_item: &fred_api::SeriesItem) -> bool {

    let title = &series_item.title.clone();

    // Return false if self.enumerate is not empty and none match.

    if !tag_selector.enumerate.is_empty() &&
    !tag_selector.enumerate.iter().any(|enum_title| enum_title == title)
    {
        return false
    }

    // Return false if self.exclude is not empty and there is an exclusion

    if !tag_selector.exclude.is_empty() &&
    tag_selector.exclude.iter().any(|exclusion| title.contains(exclusion))
    {
        return false
    }

    // Return false if self.require is not empty and a requirement is not met

    if !tag_selector.require.is_empty() &&
    tag_selector.require.iter().any(|requirement| !title.contains(requirement))
    {
        return false
    }

    true
}

// /// Return series from tags. Tags look like "loans;australia".
// pub fn interest_rate_series(tags: &str) {
//     let tags_series = FredClient::tags_series(tags).unwrap();
//     let series_items = tags_series.seriess;
//     let iter = series_items.iter();
//     for item in iter {
//         println!();
//         println!("{}", item.title);  
//         println!("{}", item.id);
//         let tags = FredClient::series_tags(&item.id).unwrap();
//         println!("{}", tags.one_line());
//     }  
// }

