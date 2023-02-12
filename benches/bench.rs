#[macro_use]
extern crate lazy_static;
extern crate criterion;
extern crate weighted_trie;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;
use weighted_trie::WeightedTrie;

lazy_static! {
    static ref TRIE: WeightedTrie = {
        let path = Path::new("/tmp/data/benchmark/weighted_strings.txt");
        let contens: String = fs::read_to_string(&path).unwrap();
        let mut trie = weighted_trie::WeightedTrie::new();
        for line in contens.lines() {
            let line_splitted: Vec<&str> = line.split('\t').collect();
            let string = line_splitted[0].to_owned();
            let weight = line_splitted[1].parse::<i32>().unwrap();
            trie.insert(string, weight);
        }
        trie
    };
}

fn insert() {
    let mut trie = weighted_trie::WeightedTrie::new();
    trie.insert("pie".to_owned(), 5);
    trie.insert("pita".to_owned(), 2);
    trie.insert("pi".to_owned(), 1);
    trie.insert("pizza".to_owned(), 10);
    trie.insert("pineapples".to_owned(), 1);
    trie.insert("pistachios".to_owned(), 4);
}

fn create_and_lookup() {
    TRIE.search("pi");
    TRIE.search("pis");
    TRIE.search("p");
    TRIE.search("pineapple");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert", |b| b.iter(|| insert()));
    c.bench_function("lookup", |b| b.iter(|| create_and_lookup()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
