use crate::trie_node::TrieNode;

/// Trie stucture
///
/// The Trie struct is the main structure of the crate.
/// It contains a root node and has methods to insert a new word and check if a word is in the trie.
///
/// # Examples
/// ```
/// use easy_trie::trie::Trie;
///
/// let mut trie = Trie::new();
/// trie.insert("hello");
/// assert_eq!(trie.len(), 5);
/// assert!(trie.contains("hello"));
/// assert!(!trie.contains("world"));
/// ```
#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /**
     * Initializes a new Trie.
     *
     * It will be an empty trie with default values
     *
     * # Examples
     * ```
     * use easy_trie::trie::Trie;
     *
     * let mut trie = Trie::new();
     * assert_eq!(trie.len(), 0);
     * ```
     */
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    /**
     * Inserts a new word into the Trie.
     *
     * # Examples
     * ```
     * use easy_trie::trie::Trie;
     *
     * let mut trie = Trie::new();
     * trie.insert("hello");
     * assert_eq!(trie.len(), 5);
     * ```
     */
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    /**
     * Calculates the length of the trie.
     *
     * Defined as the sum of the length of all children nodes and of the root node.
     *
     * # Examples
     * ```
     * use easy_trie::trie::Trie;
     *
     * let mut trie = Trie::new();
     * trie.insert("hello");
     * assert_eq!(trie.len(), 5);
     * ```
     */
    pub fn len(&self) -> usize {
        self.root.len()
    }

    /**
     * Checks if the trie contains a word.
     *
     * # Examples
     * ```
     * use easy_trie::trie::Trie;
     *
     * let mut trie = Trie::new();
     * trie.insert("hello");
     * assert!(trie.contains("hello"));
     * assert!(!trie.contains("world"));
     * ```
     */
    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_end_of_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_new_trie() {
        let result = Trie::new();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn it_should_insert_a_new_word() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert_eq!(trie.len(), 5);
    }

    #[test]
    fn it_should_insert_multiple_words() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        assert_eq!(trie.len(), 10);
    }

    #[test]
    fn it_should_correctly_insert_with_same_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        assert_eq!(trie.len(), 6);
    }

    #[test]
    fn it_should_correctly_insert_with_same_prefixes() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("hel");
        assert_eq!(trie.len(), 6);
    }

    #[test]
    fn it_should_contain_word() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(trie.contains("hello"));
    }

    #[test]
    fn it_should_not_contain_word() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(!trie.contains("world"));
    }

    #[test]
    fn it_should_not_contain_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(!trie.contains("hel"));
    }
}
