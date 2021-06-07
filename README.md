# PlayRustClassifier

This is a system for classifying [Reddit](http://reddit.com)
posts on the [/r/rust subreddit](http://reddit.com/r/rust)
that were intended for the
[/r/playrust subreddit](http://reddit.com/r/playrust).

# Status Chart

* [x] Compile with `rustc` 1.51 with current dependencies.
* [ ] Upstream fixes to `rustlearn` to get off the current
      [vendor branch](/BartMassey-upstream/rustlearn).
* [ ] Replace missing `get_unique_word_list()` function in
      `generate_freqs.rs`, forever lost to an unported version
      of the `rsml` crate.
* [ ] Find/build corpora and try this thing out.

---

This repository is a modern (2021) approved fork of
/insanitybit/PlayRustClassifier. That codebase was written in
2016; this project aims to bring it up to date to run on
current Rust. All the good things here are from the original
authors. The bad things were introduced by me (@BartMassey).

This is very much a work-in-progress. Please consider contributing.

This work is made available under the "MIT License." Please
see the file `LICENSE` in this repo for license terms.
