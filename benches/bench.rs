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
        for line in contens.lines().take(100000) {
            let line_splitted: Vec<&str> = line.split('\t').collect();
            let string = line_splitted[0].to_owned();
            let weight = line_splitted[1].parse::<i32>().unwrap();
            trie.insert(string, weight);
        }
        trie
    };
}

fn insert() {
    // Note: to get a benchmark data
    // wget https://gist.githubusercontent.com/subpath/c19778c9549e5dde02a405dd97fa7014/raw/6fe9433996607be9ceca6dc29e1d88582d64f5d1/weighted_strings.txt -P /tmp/data/benchmark
    let path = Path::new("/tmp/data/benchmark/weighted_strings.txt");
    let contens: String = fs::read_to_string(&path).unwrap();
    let mut trie = weighted_trie::WeightedTrie::new();
    for line in contens.lines().take(100000) {
        let line_splitted: Vec<&str> = line.split('\t').collect();
        let string = line_splitted[0].to_owned();
        let weight = line_splitted[1].parse::<i32>().unwrap();
        trie.insert(string, weight);
    }
}

fn lookup() {
    TRIE.search("pi");
    TRIE.search("pis");
    TRIE.search("p");
    TRIE.search("pineapple");
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("weighted_trie");
    group.sample_size(10);
    group.bench_function("insert", |b| b.iter(|| insert()));
    group.bench_function("lookup", |b| b.iter(|| lookup()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
