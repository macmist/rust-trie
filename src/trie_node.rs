use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub(crate) is_end_of_word: bool,
    pub(crate) children: HashMap<char, TrieNode>
}

impl TrieNode {

    /**
     * Calculates the length of the current node.
     * Defined as the sum of the length of all children nodes and of the current node.
     */
    pub fn len(&self) -> usize {
            let mut sum = self.children.len();
            for a in self.children.values() {
                sum += a.len();
            }
            return sum;
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_new_node() {
        let result = TrieNode::default();
        assert_eq!(result.is_end_of_word, false);
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
}