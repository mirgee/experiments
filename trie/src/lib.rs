use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    
}

struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn create() -> Self {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
