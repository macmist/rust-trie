use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub(crate) is_end_of_word: bool,
    pub(crate) children: HashMap<char, TrieNode>,
}

impl TrieNode {
    /**
     * Calculates the length of the current node.
     *
     * Defined as the sum of the length of all children nodes and of the current node.
     */
    pub(crate) fn len(&self) -> usize {
        let mut sum = self.children.len();
        for a in self.children.values() {
            sum += a.len();
        }
        sum
    }

    /**
     * Get all possible words for a given node.
     */
    pub(crate) fn get_suggestions(&self, current: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        if self.is_end_of_word {
            words.push(current.to_string());
            eprintln!("Inserting {current}")
        }
        for (c, node) in self.children.iter() {
            words.append(
                node.get_suggestions(format!("{current}{c}").as_str())
                    .as_mut(),
            );
        }
        eprintln!("returning {words:#?}");
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_new_node() {
        let result = TrieNode::default();
        assert!(!result.is_end_of_word);
        assert_eq!(result.children.len(), 0);
    }

    #[test]
    fn it_should_have_correct_length() {
        let mut node = TrieNode::default();
        assert_eq!(node.len(), 0);
        let mut child = TrieNode::default();
        child.is_end_of_word = true;
        node.children.insert('a', child);
        assert_eq!(node.len(), 1);
    }

    #[test]
    fn it_should_get_suggestions() {
        let mut node = TrieNode::default();
        let mut child = TrieNode::default();
        child.is_end_of_word = true;
        node.children.insert('a', child);
        let words = node.get_suggestions("");
        assert_eq!(words, vec!["a"]);
    }

    #[test]
    fn it_should_get_suggestions_independantly_from_prefix() {
        let mut node = TrieNode::default();
        let mut child = TrieNode::default();
        child.is_end_of_word = true;
        node.children.insert('a', child);
        let prefixes = ["some", "prefix"];
        for prefix in prefixes.iter() {
            let words = node.get_suggestions(prefix);
            assert_eq!(words, vec![format!("{prefix}a")]);
        }
    }

    #[test]
    fn it_should_get_suggestions_with_multiple_children() {
        let mut node = TrieNode::default();
        let mut child = TrieNode::default();
        child.is_end_of_word = true;
        node.children.insert('a', child);
        let mut child = TrieNode::default();
        child.is_end_of_word = true;
        node.children.insert('b', child);
        let words = node.get_suggestions("");
        assert!(words.contains(&"a".to_string()));
        assert!(words.contains(&"b".to_string()));
    }
}
