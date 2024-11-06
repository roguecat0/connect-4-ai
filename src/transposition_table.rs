use super::*;
use std::io::Read;
use std::{error::Error, fmt};
pub trait TranspositionTable {
    fn put(&mut self, key: u64, val: u8);
    fn get(&self, key: u64) -> u8;
    fn reset(&mut self);
}
#[derive(Debug)]
pub struct NaiveTranspositionTable {
    table: Vec<Entry>,
    accessed: u64,
}
impl TranspositionTable for NaiveTranspositionTable {
    fn get(&self, key: u64) -> u8 {
        assert!(key < (1_u64 << 56));
        let i = Self::index(key);
        if key == self.table[i].key() {
            self.table[i].value()
        } else {
            0
        }
    }
    fn put(&mut self, key: u64, val: u8) {
        assert!(key < (1_u64 << 56));
        let i = Self::index(key);
        self.table[i] = Entry::create(key, val);
        self.accessed += 1;
    }

    fn reset(&mut self) {
        self.accessed = 0;
        self.table.iter_mut().for_each(|m| *m = Entry::new());
    }
}

impl NaiveTranspositionTable {
    const SIZE: usize = 8388593; //8388593 == 64MB

    pub fn new() -> Self {
        Self {
            table: vec![Entry::empty(); Self::SIZE],
            accessed: 0,
        }
    }
    fn index(key: u64) -> usize {
        (key % Self::SIZE as u64) as usize
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Entry {
    key_val: u64,
}
impl Entry {
    pub fn create(key: u64, val: u8) -> Self {
        let key_val = (key << 8) | val as u64;
        Self { key_val }
    }
    pub fn new() -> Self {
        Self::empty()
    }
    pub fn empty() -> Self {
        Self { key_val: 0 }
    }
    pub fn key(&self) -> u64 {
        self.key_val >> 8
    }
    pub fn value(&self) -> u8 {
        (self.key_val & u8::MAX as u64) as u8
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( key={}, val={}, key_val={:b} )",
            self.key(),
            self.value(),
            self.key_val
        )
    }
}
pub struct OptimizedTranspoisitionTable {
    keys: Vec<u32>,
    values: Vec<u8>,
}

impl TranspositionTable for OptimizedTranspoisitionTable {
    fn put(&mut self, key: u64, val: u8) {
        let i = Self::index(key);
        self.keys[i] = key as u32;
        self.values[i] = val;
    }
    fn get(&self, key: u64) -> u8 {
        let i = Self::index(key);
        if key as u32 == self.keys[i] {
            self.values[i]
        } else {
            0
        }
    }
    fn reset(&mut self) {}
}
impl OptimizedTranspoisitionTable {
    const SIZE: usize = (1 << 23) + 9;
    pub fn new() -> Self {
        Self {
            keys: vec![0; Self::SIZE],
            values: vec![0; Self::SIZE],
        }
    }
    fn index(key: u64) -> usize {
        (key % Self::SIZE as u64) as usize
    }
}

pub struct BookTranspositionTable {
    pub keys: Vec<u8>,
    pub values: Vec<u8>,
}

impl TranspositionTable for BookTranspositionTable {
    fn put(&mut self, key: u64, val: u8) {
        let i = Self::index(key);
        self.keys[i] = key as u8;
        self.values[i] = val;
    }
    fn get(&self, key: u64) -> u8 {
        assert!(key < (1_u64 << 44));
        let i = Self::index(key);
        if key as u8 == self.keys[i] {
            self.values[i]
        } else {
            0
        }
    }
    fn reset(&mut self) {}
}
impl BookTranspositionTable {
    pub const SIZE: usize = 16777259;
    pub fn new() -> Self {
        Self {
            keys: vec![0; Self::SIZE],
            values: vec![0; Self::SIZE],
        }
    }
    pub fn create(keys: Vec<u8>, values: Vec<u8>) -> Self {
        Self { keys, values }
    }
    pub fn get(&self, key: u64) -> u8 {
        assert!(key < (1_u64 << 44));
        let i = Self::index(key);
        if key as u8 == self.keys[i] {
            self.values[i]
        } else {
            0
        }
    }
    fn index(key: u64) -> usize {
        (key % Self::SIZE as u64) as usize
    }
}
