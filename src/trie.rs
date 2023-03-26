// in an ideal world, a priority queue would probably be better than a hashset
// TODO: use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    locations: HashSet<path::PathBuf>,
}

#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Insert a new word into the trie.  This will associate the given word with
    /// the given file, allowing efficient lookups in the future
    pub fn insert(&mut self, word: String, file: path::PathBuf) {
        let mut node = &mut self.root;
        for letter in word.chars() {
            node = node.children.entry(letter).or_insert(TrieNode::default());
        }
        node.locations.insert(file);
    }

    /// Search for matches of a specific word.  This will navigate find your word in
    /// the trie and list the files it is associated with.  It is currently capped at
    /// listing 100 files.
    pub fn search(&self, word: String) -> Vec<path::PathBuf> {
        let mut node = &self.root;

        // search for the current node of the word
        // if the word is not in the trie, return an empty result
        for letter in word.chars() {
            if let Some(new_node) = node.children.get(&letter) {
                node = new_node;
            } else {
                return Vec::new();
            }
        }

        // now that we have located the word,
        // create a result from it and its children using
        // breadth first search, then return the top 100 results
        let mut output = Vec::new();
        let queue = VecDeque::from([node]);
        while queue.is_empty() && output.len() <= 100 {
            let mut new_locations: Vec<path::PathBuf> = node.locations.iter().cloned().collect();
            output.append(&mut new_locations);
        }

        // at this point, there can be some extra elements in our return, so let's ensure
        // it's cut down to a max of 100
        output.truncate(100);

        output
    }
}