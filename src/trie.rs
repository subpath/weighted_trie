use std::cmp::Reverse;
use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub suggestions: Vec<(i32, String)>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            children: HashMap::new(),
            suggestions: Vec::new(),
        }
    }
}

pub struct WeightedTrie {
    root: TrieNode,
}

pub struct WeightedString {
    pub word: String,
    pub weight: i32,
}

impl WeightedTrie {
    pub fn new() -> WeightedTrie {
        WeightedTrie {
            root: TrieNode::new(),
        }
    }

    pub fn build(weighted_strings: Vec<WeightedString>) -> WeightedTrie {
        let mut trie = WeightedTrie::new();
        weighted_strings
            .into_iter()
            .for_each(|ws| trie.insert(ws.word, ws.weight));
        trie
    }

    pub fn insert(&mut self, word: String, weight: i32) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node
                .children
                .entry(c)
                .or_insert_with(|| Box::new(TrieNode::new()));
            let pos = node
                .suggestions
                .binary_search_by_key(&Reverse(weight), |&(w, _)| Reverse(w))
                .unwrap_or_else(|x| x);
            node.suggestions.insert(pos, (weight, word.clone()));
        }
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return vec![];
            }
        }

        node.suggestions
            .iter()
            .map(|(_, word)| word.clone())
            .collect()
    }
}
