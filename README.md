# Simple Trie

![Github Action](https://github.com/macmist/rust-trie/actions/workflows/rust.yml/badge.svg)
[![Crate](https://img.shields.io/crates/v/easy-trie.svg?style=flat-square)](https://crates.io/crates/easy-trie)
[![License](https://img.shields.io/github/license/macmist/rust-trie.svg?style=flat-square)](https://github.com/macmist/rust-trie/blob/main/LICENSE)

This is a simple rust trie library.
Essentially, I wanted to understand how to create and publish a crate so I was looking for something simple but useful.

## Installation

`cargo add easy-trie`

## Usage

```rust
use easy_trie::trie::Trie;

let mut trie = Trie::new();
trie.insert("hello");
assert_eq!(trie.len(), 5);
assert!(trie.contains("hello"));
assert!(!trie.contains("world"));
```
