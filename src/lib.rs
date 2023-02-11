//! # weighted_trie
//! 🦀 Rust crate that allows to create weighted prefix trees that can be used in autocomplete
//!
//! ## Quickstart
//! To use weigthed-trie, add the following to your Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! weighted_trie = "0.1"  # NOTE: Replace to latest minor version.
//! ```
//!
//! ## Usage overview
//!
//! ```rust
//!
//! use weighted_trie::WeightedTrie;
//!
//! fn main() {
//!    let mut trie = trie::WeightedTrie::new();
//!
//!     trie.insert("pie".to_owned(), 5);
//!     trie.insert("pita".to_owned(), 2);
//!     trie.insert("pi".to_owned(), 1);
//!     trie.insert("pizza".to_owned(), 10);
//!
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
//! ## Guidelines
//! `README.md` is generated from $ cargo readme command.
//! Do not manually update `README.md` instead edit `src/lib.rs`
//! and then run `cargo readme > README.md`.
//!
//! ## License
pub use trie::WeightedTrie;
pub mod trie;
