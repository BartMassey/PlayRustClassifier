#[macro_export]
macro_rules! time {
    ($expression:expr) => (
        {

            let mut sw = $crate::Stopwatch::start_new();
            let exp = $expression;
            sw.stop();
            println!("{} took {},ms",stringify!($expression) , sw.elapsed_ms());
            exp
        }
    );
    ($expression:expr, $s:expr) => (
        {

            let mut sw = $crate::Stopwatch::start_new();
            let exp = $expression;
            sw.stop();
            println!("{} took {},ms", stringify!($s), sw.elapsed_ms());
            exp
        }
    );
}


extern crate ndarray;
extern crate lazy_static;
extern crate bincode;
extern crate clap;
extern crate csv;
extern crate serde;
extern crate rayon;
extern crate regex;
extern crate rustlearn;
extern crate serde_json;
extern crate stopwatch;
extern crate tiny_keccak;
extern crate tfidf;
extern crate fnv;

pub mod feature_extraction;
pub mod reddit;
pub mod util;

pub use ndarray::prelude::*;
pub use stopwatch::Stopwatch;
