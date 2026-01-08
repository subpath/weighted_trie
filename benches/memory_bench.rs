use std::alloc::{GlobalAlloc, Layout, System};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use weighted_trie::{WeightedString, WeightedTrie};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn reset_counters() {
    ALLOCATED.store(0, Ordering::SeqCst);
    DEALLOCATED.store(0, Ordering::SeqCst);
}

fn get_net_allocated() -> usize {
    ALLOCATED.load(Ordering::SeqCst) - DEALLOCATED.load(Ordering::SeqCst)
}

fn load_data(path: &str, count: usize) -> Vec<WeightedString> {
    let path = Path::new(path);
    let contents = fs::read_to_string(&path).unwrap();
    let mut data = Vec::new();

    for line in contents.lines().take(count) {
        let parts: Vec<&str> = line.split('\t').collect();
        data.push(WeightedString {
            word: parts[0].to_owned(),
            weight: parts[1].parse::<u32>().unwrap(),
        });
    }
    data
}

fn benchmark_memory(dataset_path: &str, word_count: usize, label: &str) {
    println!("\n{:=<60}", "");
    println!("{}: {} words", label, word_count);
    println!("{:=<60}", "");

    reset_counters();
    let before_alloc = get_net_allocated();

    let data = load_data(dataset_path, word_count);
    let data_size = get_net_allocated() - before_alloc;

    reset_counters();
    let trie = WeightedTrie::build(data);
    let trie_size = get_net_allocated();

    let stats = trie.memory_stats();

    println!("\n{:<30} {:>15} {:>12}", "Component", "Bytes", "MB");
    println!("{:-<60}", "");

    println!(
        "{:<30} {:>15} {:>12.2}",
        "Data loading overhead",
        data_size,
        data_size as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Trie (tracked allocations)",
        trie_size,
        trie_size as f64 / 1_048_576.0
    );
    println!();

    println!("{:<30} {:>15}", "Nodes count", stats.nodes_count);
    println!("{:<30} {:>15}", "Nodes capacity", stats.nodes_vec_capacity);
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Nodes struct size",
        stats.nodes_struct_size,
        stats.nodes_struct_size as f64 / 1_048_576.0
    );
    println!();

    println!("{:<30} {:>15}", "Words count", stats.words_count);
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Words storage",
        stats.words_storage_bytes,
        stats.words_storage_bytes as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Words capacity",
        stats.words_capacity_bytes,
        stats.words_capacity_bytes as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15}",
        "Word map capacity", stats.word_map_capacity
    );
    println!();

    println!(
        "{:<30} {:>15}",
        "Total suggestions", stats.suggestions_total
    );
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Suggestions heap",
        stats.suggestions_heap_bytes,
        stats.suggestions_heap_bytes as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15}",
        "Avg suggestions/node",
        stats.suggestions_total / stats.nodes_count.max(1)
    );
    println!();

    println!(
        "{:<30} {:>15}",
        "Children (Small)", stats.children_small_count
    );
    println!(
        "{:<30} {:>15}",
        "Children (Large)", stats.children_large_count
    );
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Children heap",
        stats.children_heap_bytes,
        stats.children_heap_bytes as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15.1}%",
        "Small ratio",
        (stats.children_small_count as f64 / stats.nodes_count as f64) * 100.0
    );
    println!();

    println!("{:-<60}", "");
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Total calculated",
        stats.total_bytes,
        stats.total_bytes as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Total tracked alloc",
        trie_size,
        trie_size as f64 / 1_048_576.0
    );
    println!(
        "{:<30} {:>15} {:>12.2}",
        "Difference",
        (trie_size as i64 - stats.total_bytes as i64).abs(),
        ((trie_size as i64 - stats.total_bytes as i64).abs() as f64) / 1_048_576.0
    );
    println!();

    let bytes_per_word = trie_size as f64 / stats.words_count as f64;
    println!("{:<30} {:>15.1}", "Bytes per word", bytes_per_word);
}

fn main() {
    println!("\n╔{:═<58}╗", "");
    println!("║{:^58}║", "WEIGHTED TRIE - ACCURATE MEMORY BENCHMARK");
    println!("╚{:═<58}╝", "");

    // Check if running in CI environment
    let is_ci = std::env::var("CI").is_ok();

    if is_ci {
        println!("\n⚠️  Running in CI mode - skipping large datasets\n");
    }

    benchmark_memory(
        "/tmp/data/benchmark/weighted_strings.txt",
        10_000,
        "Small Dataset (10K)",
    );

    benchmark_memory(
        "/tmp/data/benchmark/weighted_strings.txt",
        50_000,
        "Medium Dataset (50K)",
    );

    if !is_ci {
        benchmark_memory(
            "/tmp/data/benchmark/weighted_strings.txt",
            100_000,
            "Large Dataset (100K)",
        );

        benchmark_memory(
            "/tmp/data/benchmark/weighted_strings_1m.txt",
            500_000,
            "Extra Large Dataset (500K)",
        );

        benchmark_memory(
            "/tmp/data/benchmark/weighted_strings_1m.txt",
            1_000_000,
            "Massive Dataset (1M)",
        );
    } else {
        println!("\n⏭️  Skipped: 100K, 500K, and 1M datasets (CI mode)\n");
    }

    println!("\n{:=<60}", "");
    println!("Benchmark complete!");
    println!("{:=<60}\n", "");
}
