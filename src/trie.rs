use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub suggestions: BinaryHeap<Reverse<(i32, String)>>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            children: HashMap::new(),
            suggestions: BinaryHeap::new(),
        }
    }
}

pub struct WeightedTrie {
    root: TrieNode,
}

impl WeightedTrie {
    pub fn new() -> WeightedTrie {
        WeightedTrie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String, weight: i32) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node
                .children
                .entry(c)
                .or_insert_with(|| Box::new(TrieNode::new()));
            node.suggestions.push(Reverse((weight, word.clone())));
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

        let mut suggestions: Vec<(i32, String)> = node
            .suggestions
            .iter()
            .map(|Reverse((weight, word))| (weight.to_owned(), word.to_owned()))
            .collect();

        suggestions.sort_by_key(|k| Reverse(k.0));
        suggestions.iter().map(|k| k.1.clone()).collect()
    }
}
