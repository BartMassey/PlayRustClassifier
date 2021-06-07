extern crate playrust_alert;
extern crate ndarray;
extern crate clap;
extern crate csv;
extern crate dedup_by;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate stopwatch;
extern crate tfidf;

use clap::{Arg, App};
use dedup_by::dedup_by;
use playrust_alert::reddit::RawPostFeatures;

use std::collections::{BTreeSet, BTreeMap};

fn get_train_data() -> Vec<RawPostFeatures> {
    let matches = App::new("Model Generator")
                      .version("1.0")
                      .about("Generates a random forest based on a training set")
                      .arg(Arg::with_name("train")
                               .help("The CSV to train on")
                               .required(true)
                               .index(1))
                      .get_matches();

    let train_path = matches.value_of("train").unwrap();

    let mut rdr = csv::Reader::from_path(train_path).unwrap();

    let mut posts: Vec<RawPostFeatures> = rdr.deserialize()
                                             .map(|raw_post| raw_post.unwrap())
                                             .collect();

    posts.sort_by(|a, b| a.title.cmp(&b.title));
    dedup_by(&mut posts, |a, b| a.title == b.title);
    posts
}


// This function is lost in the mists of time. It was originally
// part of the long-dead `rsml` crate, but apparently in some branch
// that never made it to `crates.io`. Who knows?
//
// I will make a guess here.
fn get_unique_word_list(post: &str) -> Vec<String> {
    let mut words = BTreeSet::new();
    for w in post.split_whitespace() {
        words.insert(w.trim().to_owned());
    }
    words.into_iter().collect()
}

fn word_freqs(posts: &[RawPostFeatures]) -> BTreeMap<String, u64> {
    let mut map = BTreeMap::new();

    for post in posts {
        for word in get_unique_word_list(post.selftext.as_str()) {
            *map.entry(word).or_insert(0) += 1;
        }
    }
    map
}

fn main() {
    let posts = get_train_data();
    let (rust, _play): (Vec<RawPostFeatures>, Vec<RawPostFeatures>) = posts.into_iter()
                                                                          .partition(|post| {
                                                                              post.subreddit ==
                                                                              "rust"
                                                                          });

    let mut rust_word_freq: Vec<(String, u64)> = word_freqs(&rust[..]).into_iter().collect();
    rust_word_freq.sort_by(|a, b| a.1.cmp(&b.1));

    // let mut play_word_freq: Vec<(String, u64)> = word_freqs(&play[..]).into_iter().collect();
    // play_word_freq.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:#?}", rust_word_freq);

}
