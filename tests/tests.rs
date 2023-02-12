extern crate weighted_trie;

#[cfg(test)]
mod tests {
    use weighted_trie::WeightedTrie;

    #[test]
    fn test_weighted_trie() {
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
}
