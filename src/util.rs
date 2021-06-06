use ndarray::{ArrayBase, Dimension, ViewRepr};
use serde::{Serialize, de::DeserializeOwned};
use bincode::{serialize_into, deserialize_from};

use std::fs::File;
use std::io::prelude::*;

use csv::Writer;

// Stores the list of words, separated by new line
// The first line is the length of the list, for preallocation purposes
pub fn write_list(list: &[&str], filename: &str) {
    let mut f = File::create(filename).unwrap();
    for item in list {
        writeln!(f, "{}", item).unwrap();
    }
    let _ = f.flush();
}

pub fn load_list(path: &str) -> Vec<String> {
    let mut f = File::open(path).unwrap();
    let mut unpslit_str = String::new();
    let _ = f.read_to_string(&mut unpslit_str).unwrap();
    unpslit_str.lines().map(String::from).collect()
}

pub fn write_ndarray<T: Dimension>(nd: ArrayBase<ViewRepr<&f64>, T>, path: &str) {
    let mut wtr = Writer::from_path(format!("./data/{}.csv", path)).unwrap();
    for record in nd.rows() {
        wtr.serialize(record.as_slice()).expect("could not write");
    }
}

pub fn write_csv_vec<T: Serialize>(v: &[Vec<T>], path: &str) {
    let mut wtr = Writer::from_path(path).unwrap();
    for record in v {
        wtr.serialize(&record).expect("could not write");
    }
}

pub fn write_csv<T: Serialize>(nd: &T, path: &str) {
    let mut wtr = Writer::from_path(path).unwrap();
    wtr.serialize(nd).expect("could not write");
}

pub fn deserialize_from_file<'a, T: DeserializeOwned>(path: &'a str) -> T {
    let f = File::open(path).unwrap();
    deserialize_from(f).expect("could not read")
}

pub fn serialize_to_file<T>(s: &T, path: &str)
    where T: Serialize
{
    let f = File::create(path).unwrap();
    serialize_into(f, s).expect("could not write");
}
