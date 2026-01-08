use weighted_trie::{MemoryStats, WeightedString, WeightedTrie};

fn create_test_data() -> Vec<WeightedString> {
    vec![
        WeightedString::new("pie", 5),
        WeightedString::new("pita", 2),
        WeightedString::new("pi", 1),
        WeightedString::new("pizza", 10),
    ]
}

#[test]
fn test_weighted_trie_insert() {
    let mut trie = WeightedTrie::new();
    trie.insert("pie", 5);
    trie.insert("pita", 2);
    trie.insert("pi", 1);
    trie.insert("pizza", 10);

    assert_eq!(trie.search("pi"), vec!["pizza", "pie", "pita", "pi"]);
    assert_eq!(trie.search("piz"), vec!["pizza"]);
    assert!(trie.search("apple").is_empty());
}

#[test]
fn test_build_weighted_trie() {
    let trie = WeightedTrie::build(create_test_data());

    assert_eq!(trie.search("pi"), vec!["pizza", "pie", "pita", "pi"]);
    assert_eq!(trie.search("piz"), vec!["pizza"]);
    assert!(trie.search("apple").is_empty());
}

#[test]
fn test_duplicate_insert_higher_weight() {
    let mut trie = WeightedTrie::new();
    trie.insert("pizza", 5);
    trie.insert("pizza", 10);

    let suggestions = trie.search("piz");
    assert_eq!(suggestions, vec!["pizza"]);
    assert_eq!(suggestions.len(), 1);
}

#[test]
fn test_duplicate_insert_lower_weight() {
    let mut trie = WeightedTrie::new();
    trie.insert("pizza", 10);
    trie.insert("pizza", 5);

    let suggestions = trie.search("piz");
    assert_eq!(suggestions, vec!["pizza"]);
    assert_eq!(suggestions.len(), 1);
}

#[test]
fn test_empty_trie() {
    let trie = WeightedTrie::new();
    assert!(trie.search("anything").is_empty());
    assert!(trie.search("").is_empty());
}

#[test]
fn test_empty_string() {
    let mut trie = WeightedTrie::new();
    trie.insert("", 1);
    assert!(trie.search("").is_empty());
}

#[test]
fn test_single_character() {
    let mut trie = WeightedTrie::new();
    trie.insert("a", 1);
    trie.insert("b", 2);
    trie.insert("c", 3);

    assert_eq!(trie.search("a"), vec!["a"]);
    assert_eq!(trie.search("b"), vec!["b"]);
    assert_eq!(trie.search("c"), vec!["c"]);
    assert!(trie.search("d").is_empty());
}

#[test]
fn test_unicode_characters() {
    let mut trie = WeightedTrie::new();
    trie.insert("café", 10);
    trie.insert("naïve", 5);
    trie.insert("日本", 3);
    trie.insert("🦀rust", 8);

    assert_eq!(trie.search("caf"), vec!["café"]);
    assert_eq!(trie.search("na"), vec!["naïve"]);
    assert_eq!(trie.search("日"), vec!["日本"]);
    assert_eq!(trie.search("🦀"), vec!["🦀rust"]);
}

#[test]
fn test_weight_ordering() {
    let mut trie = WeightedTrie::new();
    trie.insert("apple", 100);
    trie.insert("append", 50);
    trie.insert("application", 200);
    trie.insert("apply", 75);
    trie.insert("appetite", 25);

    let results = trie.search("app");
    assert_eq!(
        results,
        vec!["application", "apple", "apply", "append", "appetite"]
    );
}

#[test]
fn test_max_suggestions_limit() {
    let mut trie = WeightedTrie::new();
    for i in 0..150 {
        trie.insert(format!("test{}", i), 150 - i as u32);
    }

    let results = trie.search("test");
    assert_eq!(results.len(), 10);
    assert_eq!(results[0], "test0");
    assert_eq!(results[9], "test9");
}

#[test]
fn test_long_words() {
    let mut trie = WeightedTrie::new();
    let long_word = "a".repeat(50);
    assert!(trie.insert(long_word.clone(), 1));

    let results = trie.search(&"a".repeat(49));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], long_word);
}

#[test]
fn test_max_word_length_default() {
    let trie = WeightedTrie::new();
    assert_eq!(trie.max_word_length(), 100);
}

#[test]
fn test_max_word_length_exceeded() {
    let mut trie = WeightedTrie::new();
    let too_long = "a".repeat(101);
    assert!(!trie.insert(too_long, 1));

    assert!(trie.search("a").is_empty());
}

#[test]
fn test_max_word_length_exactly_at_limit() {
    let mut trie = WeightedTrie::new();
    let at_limit = "a".repeat(100);
    assert!(trie.insert(at_limit.clone(), 1));

    let results = trie.search(&"a".repeat(99));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], at_limit);
}

#[test]
fn test_custom_max_word_length() {
    let mut trie = WeightedTrie::with_max_word_length(50);
    assert_eq!(trie.max_word_length(), 50);

    let ok_word = "a".repeat(50);
    assert!(trie.insert(ok_word.clone(), 1));

    let too_long = "a".repeat(51);
    assert!(!trie.insert(too_long, 1));

    let results = trie.search("a");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], ok_word);
}

#[test]
fn test_case_sensitivity() {
    let mut trie = WeightedTrie::new();
    trie.insert("Apple", 10);
    trie.insert("apple", 5);

    assert_eq!(trie.search("App"), vec!["Apple"]);
    assert_eq!(trie.search("app"), vec!["apple"]);
}

#[test]
fn test_special_characters() {
    let mut trie = WeightedTrie::new();
    trie.insert("hello-world", 10);
    trie.insert("test_case", 5);
    trie.insert("user@email.com", 3);
    trie.insert("123numbers", 2);

    assert_eq!(trie.search("hello"), vec!["hello-world"]);
    assert_eq!(trie.search("test"), vec!["test_case"]);
    assert_eq!(trie.search("user"), vec!["user@email.com"]);
    assert_eq!(trie.search("123"), vec!["123numbers"]);
}

#[test]
fn test_children_capacity_transition() {
    let mut trie = WeightedTrie::new();
    trie.insert("aa", 1);
    trie.insert("ab", 2);
    trie.insert("ac", 3);
    trie.insert("ad", 4);
    trie.insert("ae", 5);
    trie.insert("af", 6);

    assert_eq!(trie.search("a").len(), 6);
    assert_eq!(trie.search("aa"), vec!["aa"]);
    assert_eq!(trie.search("af"), vec!["af"]);
}

#[test]
fn test_overlapping_prefixes() {
    let mut trie = WeightedTrie::new();
    trie.insert("test", 1);
    trie.insert("testing", 2);
    trie.insert("tester", 3);
    trie.insert("tested", 4);

    assert_eq!(
        trie.search("test"),
        vec!["tested", "tester", "testing", "test"]
    );
    assert_eq!(trie.search("testi"), vec!["testing"]);
    assert_eq!(trie.search("teste"), vec!["tested", "tester"]);
}

#[test]
fn test_prefix_with_no_complete_words() {
    let mut trie = WeightedTrie::new();
    trie.insert("testing", 1);

    assert!(trie.search("testingmore").is_empty());
}

#[test]
fn test_multiple_weight_updates() {
    let mut trie = WeightedTrie::new();
    trie.insert("word", 5);
    trie.insert("word", 10);
    trie.insert("word", 3);
    trie.insert("word", 15);

    let results = trie.search("w");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "word");
}

#[test]
fn test_memory_stats() {
    let mut trie = WeightedTrie::new();
    for i in 0..100 {
        trie.insert(format!("word{}", i), i);
    }

    let stats: MemoryStats = trie.memory_stats();
    assert!(stats.nodes_count > 0);
    assert!(stats.words_count == 100);
    assert!(stats.total_bytes > 0);
    assert!(stats.suggestions_total > 0);
}

#[test]
fn test_exact_match_vs_prefix() {
    let mut trie = WeightedTrie::new();
    trie.insert("cat", 10);
    trie.insert("catalog", 5);

    assert_eq!(trie.search("cat"), vec!["cat", "catalog"]);
    assert_eq!(trie.search("cata"), vec!["catalog"]);
}

#[test]
fn test_whitespace_in_words() {
    let mut trie = WeightedTrie::new();
    trie.insert("hello world", 10);
    trie.insert("hello there", 5);

    assert_eq!(trie.search("hello "), vec!["hello world", "hello there"]);
    assert_eq!(trie.search("hello w"), vec!["hello world"]);
}

#[test]
fn test_build_with_duplicates() {
    let data = vec![
        WeightedString::new("test", 5),
        WeightedString::new("test", 10),
        WeightedString::new("test", 3),
    ];

    let trie = WeightedTrie::build(data);
    let results = trie.search("test");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "test");
}

#[test]
fn test_zero_weight() {
    let mut trie = WeightedTrie::new();
    trie.insert("zero", 0);
    trie.insert("one", 1);

    let results = trie.search("z");
    assert_eq!(results, vec!["zero"]);
    let results = trie.search("o");
    assert_eq!(results, vec!["one"]);
}

#[test]
fn test_large_weights() {
    let mut trie = WeightedTrie::new();
    trie.insert("max", u32::MAX);
    trie.insert("min", 0);
    trie.insert("mid", u32::MAX / 2);

    let results = trie.search("m");
    assert_eq!(results, vec!["max", "mid", "min"]);
}

#[test]
fn test_custom_max_suggestions() {
    let mut trie = WeightedTrie::with_max_suggestions(5);
    for i in 0..20 {
        trie.insert(format!("word{}", i), 20 - i as u32);
    }

    let results = trie.search("word");
    assert_eq!(results.len(), 5);
    assert_eq!(results[0], "word0");
    assert_eq!(results[4], "word4");
}

#[test]
fn test_max_suggestions_default() {
    let trie = WeightedTrie::new();
    assert_eq!(trie.max_suggestions(), 10);
}

#[test]
fn test_with_config() {
    let mut trie = WeightedTrie::with_config(50, 3);
    assert_eq!(trie.max_word_length(), 50);
    assert_eq!(trie.max_suggestions(), 3);

    for i in 0..10 {
        trie.insert(format!("test{}", i), 10 - i as u32);
    }

    let results = trie.search("test");
    assert_eq!(results.len(), 3);
    assert_eq!(results, vec!["test0", "test1", "test2"]);
}

#[test]
fn test_build_with_max_suggestions() {
    let words = (0..20)
        .map(|i| WeightedString::new(format!("item{}", i), 20 - i))
        .collect();

    let trie = WeightedTrie::build_with_max_suggestions(words, 7);
    let results = trie.search("item");
    assert_eq!(results.len(), 7);
    assert_eq!(results[0], "item0");
    assert_eq!(results[6], "item6");
}

#[test]
fn test_build_with_config() {
    let words = vec![
        WeightedString::new("a".repeat(60), 10),
        WeightedString::new("short", 5),
    ];

    let trie = WeightedTrie::build_with_config(words, 50, 3);

    // Long word should be rejected
    assert!(trie.search(&"a".repeat(60)).is_empty());

    // Short word should be found
    assert_eq!(trie.search("shor"), vec!["short"]);
}
