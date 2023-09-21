#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::{Deserialize, Serialize};

// Serialization
// DeSerialization
// -> SerDe

// JSON, YAML, TOML, XML, CSV, INI, HOCON, HCL, JSON5
//

// [0x01, 0x02...]
// json, yaml, ..., csv
#[derive(Deserialize, Serialize, Default)]
struct Point {
    x: i32,
    y: i32,
}

// SemVer
// ZeroVer
// 1.0.4
// ^ "DONE"
// 1.0.
// 2.1.

// Duplicate
// Clone = this CAN be duplicated
// Copy = this CAN be duplicated and preserve the semantics of a real dupliate
// by using a memcpy

// ['h', 'i', '!' , 0, 0]
// ^ x.ptr //
// ['h', 'i', '!']
// ^ y.ptr

fn foo2(x: String) {
    let y = x.clone();
    let z = x;
}

// struct String {
//     pub ptr: *mut u8,
//     pub len: usize,
//     pub capacity: usize,
// }

// trait = interface
// contract of methods that ur class must implement
#[derive(Debug)]
struct Foo {
    x: i32,
    y: i32,
    // not_clone: Bar,
}

// Copy ?

// when u compile, santa's elves get put to work writing some code
// they will write an implementation of Clone for Foo
struct Bar;

impl Clone for Foo {
    fn clone(&self) -> Self {
        return Foo {
            x: self.x.clone(),
            y: self.y.clone(),
        };
    }
}

fn foo(f: Foo) {
    foo(f.clone());
    foo(f);
}

// Vec
// hashmap
//  methods present on them :
// Iterators?
// serde?
// Derive macros?
//  Clone, Copy
//  enums -
//  match
// how can smth be an enum type + cover their variants
//  - how many variants of an i32?
// sum types / product types

// Vec?
// why do we use them?
// what advantage do they have over arrays
// some common methods
// iter methods on vec

// struct Vec {
//     size: usize,
//     capacity: usize,
//     values: &[i32],
// }
//
// how big is usize?
// - only positive unsigned
// negative capactiy?
// u32? u64?
// u32 ; u64 ; u16
// 2^64
// u32 /
// u64 -

// whty might this fail?
fn sum_of_vec_c(v: Vec<i32>) -> i32 {
    // C style for loop
    let mut sum = 0;
    //  0 to 10
    //  size = 5
    //  access elem 6
    for i in 0..v.len() {
        // This is fallible
        sum += v[i];
    }
    return sum;
}

//
fn sum_of_vec_for_iter(mut v: Vec<i32>) -> i32 {
    // For loop using implicit iter
    let mut sum = 0;
    for i in v {
        // ints are Copy
        sum += i;
    }
    sum
}

fn count_stuff(s: Vec<char>) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    // "aab"
    // {}
    // ch = 'a'
    // counts['a'] = 0
    // { 'a': 0 }
    // counts['a'] += 1
    // { 'a': 2, 'b': 1 }
    // this counts the number of times each character appears in the string
    for ch in s {
        // Struct Entry<K, V, '...>
        // woah!
        // if the entry exists, returns map[key]
        // if it not exist, map[key] = 1
        // return &mut map[key]
        //                    ^ this is the key
        *counts.entry(ch).or_insert(0) += 1;
        // mutates the value inside of the map itself
    }
    counts
}

// map, filter, fold (reduce)
// map:
// filter:
// [x] fold:

// pros?:
// - less code? - usually?
// - efficient? "zero cost abstractions"
// - optimisation? --
//  - vectorisation
// cons?:
// - confusing
// - hard to read?
// "readibility is a product of exposure"
// pro: unidirectional data flow - no mutation
fn sum_vec_func(v: Vec<i32>) -> i32 {
    // B: initial value
    // (acc, curr_vale) -> acc
    // B = i32
    // Self::Item = i32
    // | | holds the arguments
    let closure = |acc: i32, curr: i32| -> i32 { acc + curr };
    //               ^ B       ^ Self::item
    let sum = v.into_iter().fold(0, closure);
    return sum;
}

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

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("trains.csv");

    let entries: Vec<CSVEntry> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    println!("Entries: {entries:?}");

    Ok(())
}

// Product vs Sum Type
//
// SUM -> OR
//
//
//
