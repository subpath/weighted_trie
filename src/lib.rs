//! 🦀 Rust crate that allows creating weighted prefix trees that can be used in autocomplete
//!
//! [Released API Docs](https://docs.rs/crate/weighted_trie/latest)
//!
//! [![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/subpath/weighted_trie/blob/main/LICENSE)
//!
//! ## Quickstart
//! To use weigthed-trie, add the following to your Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! weighted_trie = "0.1.0"  # NOTE: Replace to latest minor version.
//! ```
//!
//! ## Usage overview
//!
//! ```rust
//!
//! use weighted_trie::WeightedTrie;
//!
//! fn main() {
//!    let mut trie = WeightedTrie::new();
//!     // build trie with words and assoicated weights
//!     trie.insert("pie".to_owned(), 5);
//!     trie.insert("pita".to_owned(), 2);
//!     trie.insert("pi".to_owned(), 1);
//!     trie.insert("pizza".to_owned(), 10);
//!     
//!     // get prefix based suggestions sorted by weight
//!     let suggestions = trie.search("pi");
//!     assert_eq!(suggestions, vec!["pizza", "pie", "pita", "pi"]);
//!
//!     let suggestions = trie.search("piz");
//!     assert_eq!(suggestions, vec!["pizza"]);
//!
//!     // out of vocabulary
//!     let suggestions = trie.search("apple");
//!     assert_eq!(suggestions.len(), 0);
//!
//! }
//!
//! ```
//! Alternatively you can use `.build`  method
//!
//! ```rust
//! use weighted_trie::{WeightedString, WeightedTrie};
//!
//! fn main() {
//!     let weighted_strings = vec![
//!            WeightedString {
//!                word: "pie".to_owned(),
//!                weight: 5,
//!            },
//!            WeightedString {
//!                word: "pita".to_owned(),
//!                weight: 2,
//!            },
//!            WeightedString {
//!                word: "pi".to_owned(),
//!                weight: 1,
//!            },
//!            WeightedString {
//!                word: "pizza".to_owned(),
//!                weight: 10,
//!            },
//!        ];
//!
//!     let trie = WeightedTrie::build(weighted_strings);
//!
//! }
//!
//! ```
//!
//! ## Benchmarks
//! Using 100k weighted strings
//!
//! ```text
//! weighted_trie/insert    time:   [374.13 ms 377.97 ms 382.13 ms]
//! weighted_trie/lookup    time:   [709.69 µs 725.45 µs 751.34 µs]
//! weighted_trie/build     time:   [375.60 ms 380.36 ms 385.45 ms]
//! ```
//!
//! ## Guidelines
//! `README.md` is generated from `cargo readme` command.
//! Do not manually update `README.md` instead edit `src/lib.rs`
//! and then run `cargo readme > README.md`.
//!
//! ## TODO:
//! 1. ~~Add tests~~
//! 2. ~~Benchmark lookup speed~~
//! 3. ~~Benchmark insert speed~~
//! 4. Measure memory footprint
//! 5. Try low hanging fruit optimizations like usage of `hashbrown` instead of standart HashMap
//!
//!
//! ## License
pub use trie::WeightedString;
pub use trie::WeightedTrie;
pub mod trie;
