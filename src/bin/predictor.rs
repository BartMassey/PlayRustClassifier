#![feature(test, custom_derive, plugin)]
#[macro_use(stack)]
extern crate ndarray;

extern crate clap;
extern crate csv;
extern crate rustc_serialize;
// extern crate pencil;
#[macro_use(time)]
extern crate playrust_alert;

extern crate rsml;

use clap::{Arg, App};
use ndarray::{Axis, ArrayBase};
use playrust_alert::reddit;
// use pencil::{Pencil, Request, Response, PencilResult};
use playrust_alert::reddit::{RawPostFeatures, ProcessedPostFeatures};
use playrust_alert::feature_extraction::{convert_author_to_popularity, convert_is_self,
                                         tfidf_reduce_selftext, subs_to_float, symbol_counts,
                                         interesting_word_freq};

use rsml::random_forest::RandomForest;
use rsml::traits::SupervisedLearning;

use rustc_serialize::{json, Decodable};

use std::fs::File;
use std::io::prelude::*;

fn load_json<T: Decodable>(path: &str) -> T {
    let mut f = File::open(path).unwrap();
    let mut json_str = String::new();

    let _ = f.read_to_string(&mut json_str).unwrap();
    json::decode(&json_str).unwrap()
}

fn load_list(path: &str) -> Vec<String> {
    let mut f = File::open(path).unwrap();
    let mut unpslit_str = String::new();
    let _ = f.read_to_string(&mut unpslit_str).unwrap();
    unpslit_str.lines().map(String::from).collect()
}

fn normalize_post_features(raw_posts: &[RawPostFeatures]) -> (Vec<ProcessedPostFeatures>, Vec<f64>) {
    let selfs: Vec<_> = raw_posts.iter().map(|r| convert_is_self(r.is_self)).collect();
    let downs: Vec<_> = raw_posts.iter().map(|r| r.downs as f64).collect();
    let ups: Vec<_> = raw_posts.iter().map(|r| r.ups as f64).collect();
    let scores: Vec<_> = raw_posts.iter().map(|r| r.score as f64).collect();
    let subreddits: Vec<_> = raw_posts.iter().map(|r| r.subreddit.as_ref()).collect();
    let sub_floats = subs_to_float(&subreddits[..]);

    let authors: Vec<&str> = raw_posts.iter().map(|s| &s.author[..]).collect();
    let rust_authors = load_list("./data/rust_author_list");
    let rust_authors: Vec<_> = rust_authors.iter().map(|s| &s[..]).collect();
    let titles: Vec<&str> = raw_posts.iter().map(|r| r.title.as_ref()).collect();
    let posts: Vec<&str> = raw_posts.iter().map(|r| r.selftext.as_ref()).collect();

    let interesting_words = load_list("./static_data/words_of_interest");

    let mut terms = Vec::new();

    for (post, title) in posts.iter().zip(titles.iter()) {
        let mut comb = String::new();
        comb.push_str(post);
        comb.push_str(" ");
        comb.push_str(title);
        terms.push(comb);
    }

    let terms: Vec<&str> = terms.iter().map(|s| s.as_str()).collect();

    let term_frequencies = interesting_word_freq(&terms[..], &interesting_words[..]);

    let symbol_frequences = symbol_counts(&posts[..]);

    let mut freqs = Vec::with_capacity(symbol_frequences.len() + term_frequencies.len());

    freqs.extend_from_slice(&term_frequencies[..]);
    freqs.extend_from_slice(&symbol_frequences[..]);

    let author_popularity = convert_author_to_popularity(&authors[..], &rust_authors[..]);

    let mut processed = Vec::with_capacity(raw_posts.len());

    for index in 0..raw_posts.len() {
        let p = ProcessedPostFeatures {
            is_self: selfs[index],
            author_popularity: author_popularity[index],
            downs: downs[index],
            ups: ups[index],
            score: scores[index],
            word_freq: freqs[index].clone(),
        };
        processed.push(p);
    }
    (processed, sub_floats)
}

fn construct_matrix(post_features: &[ProcessedPostFeatures]) -> ArrayBase<Vec<f64>, (usize, usize)> {
    let auth_pop: Vec<_> = post_features.iter().map(|p| p.author_popularity).collect();
    let downs: Vec<_> = post_features.iter().map(|p| p.downs).collect();
    let ups: Vec<_> = post_features.iter().map(|p| p.ups).collect();
    let score: Vec<_> = post_features.iter().map(|p| p.score).collect();

    assert_eq!(auth_pop.len(), post_features.len());
    assert_eq!(downs.len(), post_features.len());
    assert_eq!(ups.len(), post_features.len());
    assert_eq!(score.len(), post_features.len());

    let term_count = post_features.iter().last().unwrap().word_freq.iter().count();
    let term_frequencies: Vec<_> = post_features.iter().map(|p| &p.word_freq[..]).collect();

    let mut row = vec![auth_pop[0], downs[0], ups[0], score[0]];
    row.extend_from_slice(term_frequencies[0]);
    let mut a = stack!(Axis(0), row);

    for index in 1..post_features.len() {
        let mut row = vec![auth_pop[index], downs[index], ups[index], score[index]];
        row.extend_from_slice(term_frequencies[index]);
        a = stack!(Axis(0), a, row);
    }
    a.into_shape((post_features.len(), term_count + 4)).expect("Could not reshape a")
}

fn get_pred_data() -> Vec<RawPostFeatures> {
    let matches = App::new("PlayRust Predictor")
                      .version("1.0")
                      .about("Given a series of reddit posts, predicts which sub they came from")
                      .arg(Arg::with_name("pred")
                               .help("The CSV to predict against")
                               .required(true)
                               .index(1))
                      .get_matches();

    let pred_path = matches.value_of("pred").unwrap();

    let mut rdr = csv::Reader::from_file(pred_path).unwrap();

    rdr.decode()
       .map(|raw_post| raw_post.unwrap())
       .collect()
}

// fn predict(r: &mut Request) -> PencilResult {
//     let url = r.get_json().unwrap().as_string();
//     let reddit_client = reddit::RedditClient::new();
//     let raw_features = vec![reddit_client.get_raw_features_from_url(&url)];
//
//     let raw_posts: Vec<_> = raw_posts.into_iter().filter(|p| p.subreddit == "playrust").collect();
//     let (features, _) = normalize_post_features(&raw_posts[..2]);
//     let feat_matrix = construct_matrix(&features[..]);
//
//     let rf = load_model();
//     let pred = rf.predict(&feat_matrix).unwrap();
//     let sub = if pred.round() == 0 {
//         "playrust"
//     } else {
//         "rust"
//     };
//
//     Ok(Response::from(&sub))
// }

fn main() {
    let mut reddit_client = reddit::RedditClient::new();
    let raw = reddit_client.get_raw_features_from_url("https://www.reddit.com/r/rust/comments/4tz6e5/are_aliased_mutable_raw_pointers_ub");
    let raw_posts = reddit::get_posts(raw);

    let (features, _) = normalize_post_features(&raw_posts[..]);
    let feat_matrix = construct_matrix(&features[..]);

    let rf: RandomForest = load_json("./models/clf");

    println!("{:?}", time!(rf.predict(&feat_matrix).unwrap()));

}
