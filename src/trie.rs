use compact_str::CompactString;
use hashbrown::HashMap;
use smallvec::SmallVec;
use std::cmp::Reverse;
use std::mem::size_of;

const MAX_SUGGESTIONS_PER_NODE: usize = 10;
const SMALL_CHILDREN_CAPACITY: usize = 4;
const MAX_WORD_LENGTH: usize = 100;

type NodeIndex = u32;
type WordIndex = u32;
type PackedSuggestion = u64;

#[inline(always)]
const fn pack_suggestion(weight: u32, word_idx: WordIndex) -> PackedSuggestion {
    ((weight as u64) << 32) | (word_idx as u64)
}

#[inline(always)]
const fn get_weight(packed: PackedSuggestion) -> u32 {
    (packed >> 32) as u32
}

#[inline(always)]
const fn get_word_idx(packed: PackedSuggestion) -> WordIndex {
    packed as u32
}

#[derive(Clone)]
enum Children {
    Small(SmallVec<[(char, NodeIndex); SMALL_CHILDREN_CAPACITY]>),
    Large(HashMap<char, NodeIndex>),
}

impl Children {
    #[inline]
    fn new() -> Self {
        Self::Small(SmallVec::new())
    }

    #[inline]
    fn get(&self, c: char) -> Option<NodeIndex> {
        match self {
            Self::Small(vec) => vec.iter().find_map(|&(ch, idx)| (ch == c).then_some(idx)),
            Self::Large(map) => map.get(&c).copied(),
        }
    }

    #[inline]
    fn insert(&mut self, c: char, idx: NodeIndex) {
        match self {
            Self::Small(vec) if vec.len() < SMALL_CHILDREN_CAPACITY => {
                if let Some(entry) = vec.iter_mut().find(|(ch, _)| *ch == c) {
                    entry.1 = idx;
                } else {
                    vec.push((c, idx));
                }
            }
            Self::Small(vec) => {
                #[cold]
                fn transition_to_large(vec: &mut SmallVec<[(char, NodeIndex); SMALL_CHILDREN_CAPACITY]>, c: char, idx: NodeIndex) -> Children {
                    let mut map: HashMap<_, _> = vec.drain(..).collect();
                    map.insert(c, idx);
                    Children::Large(map)
                }
                *self = transition_to_large(vec, c, idx);
            }
            Self::Large(map) => {
                map.insert(c, idx);
            }
        }
    }
}

#[derive(Default)]
pub struct TrieNode {
    children: Children,
    suggestions: SmallVec<[PackedSuggestion; 2]>,
}

impl Default for Children {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WeightedTrie {
    nodes: Vec<TrieNode>,
    root: NodeIndex,
    words: Vec<CompactString>,
    word_map: HashMap<CompactString, WordIndex>,
    max_word_length: usize,
    max_suggestions: usize,
}

#[derive(Clone)]
pub struct WeightedString {
    pub word: String,
    pub weight: u32,
}

impl WeightedString {
    pub fn new(word: impl Into<String>, weight: u32) -> Self {
        Self {
            word: word.into(),
            weight,
        }
    }
}

pub struct MemoryStats {
    pub nodes_count: usize,
    pub nodes_vec_capacity: usize,
    pub nodes_struct_size: usize,
    pub words_count: usize,
    pub words_storage_bytes: usize,
    pub words_capacity_bytes: usize,
    pub word_map_capacity: usize,
    pub suggestions_total: usize,
    pub suggestions_heap_bytes: usize,
    pub children_small_count: usize,
    pub children_large_count: usize,
    pub children_heap_bytes: usize,
    pub total_bytes: usize,
}

impl WeightedTrie {
    pub fn new() -> Self {
        Self::with_config(MAX_WORD_LENGTH, MAX_SUGGESTIONS_PER_NODE)
    }

    pub fn with_max_word_length(max_word_length: usize) -> Self {
        Self::with_config(max_word_length, MAX_SUGGESTIONS_PER_NODE)
    }

    pub fn with_max_suggestions(max_suggestions: usize) -> Self {
        Self::with_config(MAX_WORD_LENGTH, max_suggestions)
    }

    pub fn with_config(max_word_length: usize, max_suggestions: usize) -> Self {
        Self {
            nodes: vec![TrieNode::default()],
            root: 0,
            words: Vec::new(),
            word_map: HashMap::new(),
            max_word_length,
            max_suggestions,
        }
    }

    pub fn memory_stats(&self) -> MemoryStats {
        let nodes_count = self.nodes.len();
        let nodes_vec_capacity = self.nodes.capacity();
        let nodes_struct_size = nodes_count * size_of::<TrieNode>();

        let words_count = self.words.len();
        let words_storage_bytes: usize = self.words.iter().map(|s| s.len()).sum();
        let words_capacity_bytes: usize = self.words.iter().map(|s| s.capacity()).sum();
        let word_map_capacity = self.word_map.capacity();

        let (
            suggestions_total,
            suggestions_heap_bytes,
            children_small_count,
            children_large_count,
            children_heap_bytes,
        ) = self.nodes.iter().fold(
            (0, 0, 0, 0, 0),
            |(sugg_total, sugg_heap, small, large, child_heap), node| {
                let sugg_heap_add = if node.suggestions.spilled() {
                    node.suggestions.capacity() * size_of::<PackedSuggestion>()
                } else {
                    0
                };

                let (small_add, large_add, child_heap_add) = match &node.children {
                    Children::Small(_) => (1, 0, 0),
                    Children::Large(map) => (
                        0,
                        1,
                        map.capacity() * (size_of::<char>() + size_of::<u32>() + 8),
                    ),
                };

                (
                    sugg_total + node.suggestions.len(),
                    sugg_heap + sugg_heap_add,
                    small + small_add,
                    large + large_add,
                    child_heap + child_heap_add,
                )
            },
        );

        let total_bytes = nodes_struct_size
            + nodes_vec_capacity * size_of::<TrieNode>()
            + words_capacity_bytes
            + word_map_capacity * (size_of::<CompactString>() + size_of::<u32>() + 8)
            + suggestions_heap_bytes
            + children_heap_bytes;

        MemoryStats {
            nodes_count,
            nodes_vec_capacity,
            nodes_struct_size,
            words_count,
            words_storage_bytes,
            words_capacity_bytes,
            word_map_capacity,
            suggestions_total,
            suggestions_heap_bytes,
            children_small_count,
            children_large_count,
            children_heap_bytes,
            total_bytes,
        }
    }

    pub fn build(weighted_strings: Vec<WeightedString>) -> Self {
        Self::build_with_config(weighted_strings, MAX_WORD_LENGTH, MAX_SUGGESTIONS_PER_NODE)
    }

    pub fn build_with_max_word_length(weighted_strings: Vec<WeightedString>, max_word_length: usize) -> Self {
        Self::build_with_config(weighted_strings, max_word_length, MAX_SUGGESTIONS_PER_NODE)
    }

    pub fn build_with_max_suggestions(weighted_strings: Vec<WeightedString>, max_suggestions: usize) -> Self {
        Self::build_with_config(weighted_strings, MAX_WORD_LENGTH, max_suggestions)
    }

    pub fn build_with_config(weighted_strings: Vec<WeightedString>, max_word_length: usize, max_suggestions: usize) -> Self {
        let count = weighted_strings.len();
        let mut trie = Self {
            nodes: Vec::with_capacity((count * 2).max(1000)),
            root: 0,
            words: Vec::with_capacity(count),
            word_map: HashMap::with_capacity(count),
            max_word_length,
            max_suggestions,
        };
        trie.nodes.push(TrieNode::default());

        for WeightedString { word, weight } in weighted_strings {
            trie.insert(word, weight);
        }

        trie.words.shrink_to_fit();
        trie.word_map.shrink_to_fit();
        trie.nodes.shrink_to_fit();

        trie
    }

    pub fn insert(&mut self, word: impl Into<String>, weight: u32) -> bool {
        let word = word.into();

        if word.len() > self.max_word_length {
            return false;
        }

        let word_compact = CompactString::from(&word);
        let word_idx = *self
            .word_map
            .entry(word_compact.clone())
            .or_insert_with(|| {
                self.words.push(word_compact);
                (self.words.len() - 1) as WordIndex
            });

        let packed = pack_suggestion(weight, word_idx);
        let mut node_idx = self.root;

        for c in word.chars() {
            node_idx = self.get_or_create_child(node_idx, c);
            self.insert_suggestion(node_idx, word_idx, packed, weight);
        }

        true
    }

    #[inline]
    fn get_or_create_child(&mut self, node_idx: NodeIndex, c: char) -> NodeIndex {
        if let Some(idx) = self.nodes[node_idx as usize].children.get(c) {
            return idx;
        }

        let new_idx = self.nodes.len() as NodeIndex;
        self.nodes.push(TrieNode::default());
        self.nodes[node_idx as usize].children.insert(c, new_idx);
        new_idx
    }

    #[inline]
    fn insert_suggestion(
        &mut self,
        node_idx: NodeIndex,
        word_idx: WordIndex,
        packed: PackedSuggestion,
        weight: u32,
    ) {
        let node = &mut self.nodes[node_idx as usize];

        if let Some(pos) = node
            .suggestions
            .iter()
            .position(|&p| get_word_idx(p) == word_idx)
        {
            if weight > get_weight(node.suggestions[pos]) {
                node.suggestions.remove(pos);
            } else {
                return;
            }
        }

        let pos = node
            .suggestions
            .binary_search_by_key(&Reverse(weight), |&p| Reverse(get_weight(p)))
            .unwrap_or_else(|x| x);

        node.suggestions.insert(pos, packed);

        if node.suggestions.len() > self.max_suggestions {
            node.suggestions.truncate(self.max_suggestions);
        }
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut node_idx = self.root;

        for c in prefix.chars() {
            node_idx = match self.nodes[node_idx as usize].children.get(c) {
                Some(idx) => idx,
                None => return Vec::new(),
            };
        }

        self.nodes[node_idx as usize]
            .suggestions
            .iter()
            .map(|&packed| self.words[get_word_idx(packed) as usize].to_string())
            .collect()
    }

    pub fn max_word_length(&self) -> usize {
        self.max_word_length
    }

    pub fn max_suggestions(&self) -> usize {
        self.max_suggestions
    }
}

impl Default for WeightedTrie {
    fn default() -> Self {
        Self::new()
    }
}
