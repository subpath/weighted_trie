//! 🦀 Rust crate that allows creating weighted prefix trees that can be used in autocomplete
//!
//! [Released API Docs](https://docs.rs/crate/weighted_trie/latest)
//!
//! [![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/subpath/weighted_trie/blob/main/LICENSE)
//!
//! ## Features
//!
//! - **Speed-optimized**: Single insert in 272ns, lookup in 244ns, bulk build (100K) in 33.7ms
//! - **Memory-efficient**: Only 243 bytes per word at 1M scale
//! - **In-memory**: Pure memory-based data structure, no disk I/O
//! - **Dynamic**: Supports incremental inserts on-the-fly
//! - **Prefix-based**: Returns all matches for a given prefix, sorted by weight
//! - **Weight-sorted**: Results pre-sorted by descending weight
//! - **Configurable limits**:
//!   - Max suggestions per query (default: 10)
//!   - Max word length (default: 100 characters)
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
//! use weighted_trie::WeightedTrie;
//!
//! let mut trie = WeightedTrie::new();
//! // build trie with words and associated weights
//! trie.insert("pie", 5);
//! trie.insert("pita", 2);
//! trie.insert("pi", 1);
//! trie.insert("pizza", 10);
//!
//! // get prefix based suggestions sorted by weight
//! let suggestions = trie.search("pi");
//! assert_eq!(suggestions, vec!["pizza", "pie", "pita", "pi"]);
//!
//! let suggestions = trie.search("piz");
//! assert_eq!(suggestions, vec!["pizza"]);
//!
//! // out of vocabulary
//! let suggestions = trie.search("apple");
//! assert_eq!(suggestions.len(), 0);
//! ```
//! Alternatively you can use `.build`  method
//!
//! ```rust
//! use weighted_trie::{WeightedString, WeightedTrie};
//!
//! let weighted_strings = vec![
//!     WeightedString::new("pie", 5),
//!     WeightedString::new("pita", 2),
//!     WeightedString::new("pi", 1),
//!     WeightedString::new("pizza", 10),
//! ];
//!
//! let trie = WeightedTrie::build(weighted_strings);
//! ```
//!
//! ### Custom Configuration
//!
//! ```rust
//! use weighted_trie::WeightedTrie;
//!
//! // Create trie with custom max word length of 50 characters
//! let mut trie = WeightedTrie::with_max_word_length(50);
//!
//! // This succeeds
//! assert!(trie.insert("short", 10));
//!
//! // This fails - word too long
//! let very_long_word = "a".repeat(51);
//! assert!(!trie.insert(very_long_word, 5));
//!
//! // Create trie with custom max suggestions limit
//! let mut trie = WeightedTrie::with_max_suggestions(5);
//! for i in 0..20 {
//!     trie.insert(format!("word{}", i), 20 - i as u32);
//! }
//! assert_eq!(trie.search("word").len(), 5); // Only top 5 returned
//!
//! // Configure both word length and suggestions limit
//! let mut trie = WeightedTrie::with_config(100, 5);
//! ```
//!
//! ## Benchmarks
//!
//! ### Performance
//! ```text
//! Single insert:           272 ns
//! Lookup (per query):      244 ns
//! Build (100K words):      33.7 ms
//! Insert 100K (incremental): 32.6 ms
//! ```
//!
//! ### Memory Footprint
//! ```text
//! Dataset      Memory      Bytes/Word
//! ------------------------------------
//! 10K          2.4 MB      254
//! 50K          13.0 MB     273
//! 100K         29.5 MB     309
//! 500K         130.8 MB    274
//! 1M           231.4 MB    243
//! ```
//!
//! Run detailed memory analysis:
//! ```bash
//! cargo bench --bench memory_bench
//! ```
//!
//! ## Guidelines
//! `README.md` is generated from `cargo readme` command.
//! Do not manually update `README.md` instead edit `src/lib.rs`
//! and then run `cargo readme > README.md`.
//!
//! ## Optimizations
//! - **String interning**: Each word stored once, nodes use indices
//! - **Packed suggestions**: Weight+index packed into u64 (bit manipulation)
//! - **compact_str**: 12-byte strings vs 24-byte std String
//! - **SmallVec**: Stack allocation for small collections (≤2 suggestions, ≤4 children)
//! - **Arena allocation**: All nodes in Vec, use indices instead of Box pointers
//! - **hashbrown HashMap**: Faster than std HashMap
//! - **Pre-allocation**: Vec capacity = words × 2 (avoids reallocations)
//! - **shrink_to_fit**: Removes over-allocation after build
//! - **Top-K limiting**: 10 suggestions per node max
//! - **Deduplication**: Automatic on insert
//!
//!
//! ## License
pub use trie::MemoryStats;
pub use trie::WeightedString;
pub use trie::WeightedTrie;
pub mod trie;
