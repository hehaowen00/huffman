use std::fmt::{Debug, Formatter};
use std::iter::{IntoIterator, Iterator};
use std::ops::Index;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct BitSet {
    pub data: Vec<u8>,
    pub(crate) capacity: usize,
    pub(crate) len: usize,
    pub(crate) len_b: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            capacity: 0,
            len: 0,
            len_b: 0,
        }
    }

    pub fn from(bytes: &[u8], len: usize) -> Self {
        let data = bytes.to_vec();
        let len_b = len % 8;
        let len = bytes.len() - 1;

        Self {
            data,
            capacity: len,
            len,
            len_b,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity * 8 
    }

    pub fn len(&self) -> usize {
        self.len * 8 + self.len_b 
    }

    pub fn iter<'a>(&'a self) -> BitSetIterator<'a> {
        BitSetIterator::new(self)
    }

    pub fn zeros(&self) -> usize {
        let mut count = 0;
        for i in 0..self.len() {
            if self[i] == 0 {
                count += 1;
            }
        }
        count
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let b = match self.len_b {
            0 => 0,
            _ => 1,
        };

        match self.len {
            0 => vec![],
            _ => self.data[0..self.len + b].to_vec(),
        }
    }

    pub fn as_sequence(&self) -> Vec<u8> {
        let len = self.len();
        let mut output = Vec::with_capacity(len);
        for i in 0..len {
            output.push(self.get(i).unwrap());
        }
        output
    }

    pub fn get(&self, idx: usize) -> Option<u8> {
        if idx > self.len() {
            panic!("out of bounds");
        }

        match idx / 8 < self.data.len() {
            true => {
                let v = match self.data[idx / 8] & (1 << idx % 8) {
                    0 => 0,
                    _ => 1,
                };
                Some(v)
            },
            false => None
        }
    }

    pub fn set(&mut self, idx: usize, bit: u8) {
        if idx > self.len() {
            panic!("out of bounds");
        }

        match bit {
            0 => {
                self.data[idx / 8] &= !(1 << (idx % 8));
            },
            1 => {
                self.data[idx / 8] |= 1 << (idx % 8);
            },
            _ => unreachable!()
        }
    }

    pub fn extend(&mut self, rhs: &BitSet) {
        for idx in 0..rhs.len() {
            self.push(rhs[idx]);
        }
    }

    pub fn push(&mut self, bit: u8) {
        self.grow();

        let idx_a = self.len;
        match bit {
            0 => {
                self.data[idx_a] &= !(1 << (self.len_b % 8));
            },
            1 => {
                self.data[idx_a] |= 1 << (self.len_b % 8);
            },
            _ => unreachable!()
        }

        self.len_b += 1;
    }

    fn grow(&mut self) {
        if self.capacity == 0 {
            self.data.push(0u8);
            self.capacity = 1;
        } else if self.len_b == 8 {
            self.len += 1;
            self.len_b = 0;

            if self.len == self.capacity {
                for _ in 0..self.capacity {
                    self.data.push(0u8);
                }

                self.capacity *= 2;
            }
        }
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Index<usize> for BitSet {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self.data[index / 8] & (1 << index % 8) {
            0 => &0,
            _ => &1, 
        }
    }
}

impl IntoIterator for BitSet {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut entries = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            entries.push(self[i]);
        }
        entries.into_iter()
    }
}

pub struct BitSetIterator<'a> {
    data: &'a BitSet,
    pos: usize,
}

impl<'a> BitSetIterator<'a> {
    pub fn new(set: &'a BitSet) -> Self {
        Self {
            data: set,
            pos: 0,
        }
    }
}

impl<'a> Iterator for BitSetIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if self.pos > self.data.len() {
            return None;
        }
        Some(self.data[self.pos - 1])
    }
}

