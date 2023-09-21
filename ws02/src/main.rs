#![allow(dead_code)]

use std::path::Path;
use std::{collections::HashMap, error::Error};

use geoutils::Location;
use serde::Deserialize;
mod useful_code;
use useful_code::{convert_csventry_to_entry, Entry};

// Deserialize
// Serialize
// SerDe

#[derive(Deserialize, Debug)]
struct CSVEntry {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

// Use the provided starter code (and the useful code in a seperate file) to represent the CSV data in a useful way. The extra code you have been provided should be copied into your main function ( you may need to fix bugs that arise when you copy it across).
// Find the most and least used stations on the NSW network at each time of day.
// Allow a user to search for a station, and show it's busiest times of day, and busiest year.
// Which station has had it's yearly utilisation increase the most in the last 5 years?
// Which station had the biggest percentage change in use over 2020?
// What is the north-most, south-most, east-most and west-most station?
// Find the two closest stations and the two furthest away from eachother.
// Sort stations by their distance from central, and by their usage. Is the order similar, or different? Are there outliers?
// (hard) A meteor is headed for sydney! It is headed for a train station, but we don't know which one. It will destroy all stations within 2 kilometers of it. For each 4-hour period, make a list of which stations would be best or worst to hit.
// (very hard) Make a graph of an interesting result found above, and post it to /r/dataisbeautiful.
fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("trains.csv");

    let entries = csv::Reader::from_path(&path)?
        .deserialize() // Vec<CSVEntry>
        .collect::<Result<Vec<CSVEntry>, _>>()?;
    // .into_iter()
    // .map(convert_csventry_to_entry)
    // .collect::<Vec<_>>;
    // map.entry('station_name').or_insert(Vec::new()).push(e);
    //                            ^ if Err(e)
    //                            return Err(e)
    // ^ What is going on here?
    // can you figure out what types are being used in the `_`?

    println!("Entries: {entries:#?}");

    Ok(())
}

// # Stuff?
// Vec
// HashMap
// Simple derive macros (Copy, Clone, Debug).
// The purpose of the serde crate (with reference to the starter code).
//
// # Other:
//
//
//

// VEC?
// vector array -

fn fo_vec() {
    // Fixed size! Can be stack allocated
    let arr = [1, 2, 3];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // macro to create a vector
    let v = vec![1, 2, 3];
    //         ^ does magic (not really but for now we don't need to know)
    let mut v = Vec::from(arr);
    let x = arr[0];
    // ^ also inline
    // what can we do with vec?

    v.remove(10);
    let x = v[10];
    //
    v.get(10);
    v.pop(); // removes from end
}

fn sum_of_vec_c(v: Vec<i32>) -> i32 {
    // for (int i = 0; i < v.lne(); i++)
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i];
        //v.pop(i);
    }
    return sum;
    // why is this bad?????
    // out of bounds index
}

fn sum_range(mut v: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut iter = v.into_iter();
    while let Some(elem) = iter.next() {
        sum += elem;
    }
    for elem in &v {
        sum += elem;
        v.remove(0);
        let x = v[0];
    }
    return sum;
}

//
// filter, map, fold (reduce)
// filter: takes ur iterator, only keeps the elems that match the predicate
// map: takes each element and transforms it
// fold: take an iterator and fold / reduce it into a single elemtn
//  takes: intial value
//  takes: function that takes an acumutator and current elem, returns new acc
fn functional_sum(v: Vec<i32>) -> i32 {
    // |a, b| - this is the args
    // js -> `()` can be ambigious
    return v
        .into_iter()
        .filter(|x| {
            print!("filter: {x}");
            x % 2 != 0
        })
        .map(|x| x * 2)
        .take(10)
        .fold(0, |acc, curr| acc + curr);
    //                           ^?
    //                                    ^    ^
}

struct Point {
    x: i32,
    y: i32,
}

fn uses_point(p: Point) {
    uses_point(p);
    uses_point(p);
}

// copy
fn count_c(s: Vec<String>) {
    let mut map = HashMap::new();
    let s2 = s.clone();
    // wk 3
    let into_it = s.into_iter(); // consumes and u get ownership
    dbg!(s);
    s.into_iter().map(|elem| {
        elem.push_str("hello");
        elem
    });
    let ite = s.iter(); // borrows
    for c in &s {
        // Entry<'_, K, V>
        // Occupied
        // Vacant
        // and
        // or
        map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        //                     ^ modify   if exists
        //                                 ^ insert 1 if not exists
    }
}
