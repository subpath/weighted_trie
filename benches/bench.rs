use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use memory_stats::memory_stats;
use std::fs;
use weighted_trie::{WeightedString, WeightedTrie};

const BENCHMARK_FILE: &str = "/tmp/data/benchmark/weighted_strings.txt";
const WORD_COUNT: usize = 100_000;
const MB: usize = 1_024 * 1_024;

fn load_data(limit: usize) -> Vec<WeightedString> {
    fs::read_to_string(BENCHMARK_FILE)
        .expect("benchmark data file not found")
        .lines()
        .take(limit)
        .filter_map(|line| {
            let mut parts = line.split('\t');
            Some(WeightedString {
                word: parts.next()?.to_owned(),
                weight: parts.next()?.parse().ok()?,
            })
        })
        .collect()
}

lazy_static! {
    static ref TRIE: WeightedTrie = WeightedTrie::build(load_data(WORD_COUNT));
    static ref DATA: Vec<WeightedString> = load_data(WORD_COUNT);
}

fn insert_single() {
    let mut trie = WeightedTrie::new();
    trie.insert("test_word".to_owned(), 100);
}

fn insert_100k() {
    let mut trie = WeightedTrie::new();
    for WeightedString { word, weight } in DATA.iter() {
        trie.insert(word.clone(), *weight);
    }
}

fn lookup() {
    for query in ["pi", "pis", "p", "pineapple"] {
        TRIE.search(query);
    }
}

fn build_100k() {
    let _ = WeightedTrie::build(DATA.clone());
}

fn memory_footprint() {
    println!("\n=== Memory Footprint Benchmark ===");

    let Some(usage_before) = memory_stats() else {
        println!("Failed to get memory stats - may not be supported on this platform");
        println!("==================================\n");
        return;
    };

    println!("Memory before building trie:");
    println!("  Physical: {} MB", usage_before.physical_mem / MB);
    println!("  Virtual:  {} MB", usage_before.virtual_mem / MB);

    let _trie = WeightedTrie::build(load_data(WORD_COUNT));

    if let Some(usage_after) = memory_stats() {
        println!("\nMemory after building trie:");
        println!("  Physical: {} MB", usage_after.physical_mem / MB);
        println!("  Virtual:  {} MB", usage_after.virtual_mem / MB);

        let physical_diff = usage_after.physical_mem as i64 - usage_before.physical_mem as i64;
        let virtual_diff = usage_after.virtual_mem as i64 - usage_before.virtual_mem as i64;

        println!("\nMemory increase (approximate):");
        println!("  Physical: {} MB", physical_diff / MB as i64);
        println!("  Virtual:  {} MB", virtual_diff / MB as i64);
    }

    println!("==================================\n");
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("weighted_trie");

    // Single insert benchmark - high sample size for accuracy
    group.bench_function("insert_single", |b| b.iter(insert_single));

    // Bulk operations - lower sample size
    group.sample_size(10);
    group.bench_function("insert_100k", |b| b.iter(insert_100k));
    group.bench_function("lookup", |b| b.iter(lookup));
    group.bench_function("build_100k", |b| b.iter(build_100k));

    memory_footprint();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
