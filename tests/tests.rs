extern crate weighted_trie;

#[cfg(test)]
mod tests {
    use weighted_trie::{trie::WeightedString, WeightedTrie};

    #[test]
    fn test_weighted_trie_insert() {
        let mut trie = WeightedTrie::new();
        trie.insert("pie".to_owned(), 5);
        trie.insert("pita".to_owned(), 2);
        trie.insert("pi".to_owned(), 1);
        trie.insert("pizza".to_owned(), 10);

        // get prefix based suggestions sorted by weight
        let suggestions = trie.search("pi");
        assert_eq!(suggestions, vec!["pizza", "pie", "pita", "pi"]);

        let suggestions = trie.search("piz");
        assert_eq!(suggestions, vec!["pizza"]);

        // out of vocabulary
        let suggestions = trie.search("apple");
        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_build_weighted_trie() {
        let weighted_strings = vec![
            WeightedString {
                word: "pie".to_owned(),
                weight: 5,
            },
            WeightedString {
                word: "pita".to_owned(),
                weight: 2,
            },
            WeightedString {
                word: "pi".to_owned(),
                weight: 1,
            },
            WeightedString {
                word: "pizza".to_owned(),
                weight: 10,
            },
        ];

        let trie = WeightedTrie::build(weighted_strings);

        // get prefix based suggestions sorted by weight
        let suggestions = trie.search("pi");
        assert_eq!(suggestions, vec!["pizza", "pie", "pita", "pi"]);

        let suggestions = trie.search("piz");
        assert_eq!(suggestions, vec!["pizza"]);

        // out of vocabulary
        let suggestions = trie.search("apple");
        assert_eq!(suggestions.len(), 0);
    }
}
