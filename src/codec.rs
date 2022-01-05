use crate::bitset::BitSet;
use crate::trie::TrieNode;
use std::collections::HashMap; 

pub struct Codec;

impl Codec {
    pub fn encode(input: &[u8]) -> (Vec<(u8, BitSet)>, BitSet) {
        let mut nodes = count(input);

        while nodes.len() > 1 {
            nodes.sort_by(|a, b| b.freq().cmp(&a.freq()));
            let a = nodes.pop().unwrap();
            let b = nodes.pop().unwrap();

            let mut n = TreeNode::new(a.freq() + b.freq(), None);
            n.set_children(a, b);
            nodes.push(n);
        }

        let root = Box::new(nodes.pop().unwrap());
        let dict = tree_to_map(&root);

        let mut output = BitSet::new();

        for b in input {
            if let Ok(idx) = dict.binary_search_by_key(b, |(k, _)| *k) {
                let (_, v) = &dict[idx];
                output.extend(&v);
            }
        }

        (dict, output)
    }

    pub fn decode(dict: Vec<(u8, BitSet)>, input: &BitSet) -> Vec<u8> {
        let mut trie = TrieNode::new();
        for (k, v) in &dict {
            trie.insert(v.as_sequence(), *k);
        }

        let mut node = Some(&trie);
        let mut output = Vec::new();

        for i in 0..input.len() {
            match node {
                Some(n) => if n.value().is_none() {
                    node = n.get(&input[i]);
                },
                None => node = trie.get(&input[i]),
            }
            if let Some(n) = node {
                if let Some(v) = n.value() {
                    output.push(*v);
                    node = None;
                }
            }
        }    

        output
    }
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    freq: u64,
    value: Option<u8>,
    children: [Option<Box<TreeNode>>; 2],
}

impl TreeNode {
    pub fn new(freq: u64, value: Option<u8>) -> Self {
        Self {
            freq,
            value,
            children: [None, None],
        }
    }

    pub fn freq(&self) -> u64 {
        self.freq
    }

    pub fn set_children(&mut self, left: TreeNode, right: TreeNode) {
        self.children = [Some(Box::new(left)), Some(Box::new(right))];
    }

    pub fn left(&self) -> Option<&Box<TreeNode>> {
        self.children[0].as_ref()
    }

    pub fn right(&self) -> Option<&Box<TreeNode>> {
        self.children[1].as_ref()
    }

    pub fn value(&self) -> Option<&u8> {
        self.value.as_ref()
    }
}

pub fn count(bytes: &[u8]) -> Vec<TreeNode> {
    let mut data = HashMap::new();

    for byte in bytes {
        let count = data.entry(*byte).or_insert(0);
        *count += 1; 
    }

    let nodes = data
        .iter()
        .map(|(k, v)| TreeNode::new(*v, Some(*k)))
        .collect();

    nodes
}

fn tree_to_map(p: &Box<TreeNode>) -> Vec<(u8, BitSet)> {
    let mut nodes = vec![p];
    let mut paths = vec![BitSet::new()];
    let mut values = Vec::new();

    while nodes.len() > 0 {
        let n = nodes.pop().unwrap();
        let p = paths.pop().unwrap();

        if n.value().is_some() {
            values.push((*n.value().unwrap(), p.clone()));
        }

        if let Some(l) = n.left() {
            nodes.push(l);
            let mut pl = p.clone();
            pl.push(0);
            paths.push(pl);
        }

        if let Some(r) = n.right() {
            nodes.push(r);
            let mut pr = p.clone();
            pr.push(1);
            paths.push(pr);
        }
    }

    values.sort_by(|(a, _), (b, _)| a.cmp(b));
    values
}
