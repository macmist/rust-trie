//! Easy implementation of tries in rust.
//!
//! A simple crate to use tries in rust.
//! At the moment, supports only insertion and checking if a word is in the trie.
//! Only works with `str` types.
//!
//! # Examples
//! ```
//! use easy_trie::trie::Trie;
//!
//! let mut trie = Trie::new();
//! trie.insert("hello");
//! assert_eq!(trie.len(), 5);
//! assert!(trie.contains("hello"));
//! assert!(!trie.contains("world"));
//! ```

pub mod trie;
pub mod trie_node;
