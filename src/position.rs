use std::borrow::BorrowMut;
use std::fmt;
use std::io::Read;
#[derive(Clone, Debug)]
pub struct Position {
    pub moves: usize,
    current_position: u64,
    mask: u64,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "player: {}\nboard:\n{}",
            self.get_current_player(),
            self.mask
        )
    }
}

impl Position {
    pub const HEIGHT: usize = 6;
    pub const WIDTH: usize = 7;
    pub const MIN_SCORE: isize = -((Self::WIDTH * Self::HEIGHT) as isize) / 2 + 3;
    pub const MAX_SCORE: isize = ((Self::WIDTH * Self::HEIGHT) as isize + 1) / 2 - 3;

    pub fn new() -> Self {
        Self {
            moves: 0,
            current_position: 0,
            mask: 0,
        }
    }
    pub fn parse(code: &str) -> Self {
        code.chars()
            .flat_map(|c| c.to_digit(10).map(|d| d - 1))
            .fold(Self::new(), |acc, d| acc.next_pos(d as usize))
    }
    pub fn parse_safe(code: &str) -> Option<Self> {
        code.chars()
            .try_fold(Self::new(), |acc, c| match c.to_digit(10) {
                Some(n) if n < 7 => None,
                Some(n) => Some(acc.next_pos(n as usize)),
                None => None,
            })
    }
    pub fn key(&self) -> u64 {
        self.current_position + self.mask
    }
    pub fn key3(&self) -> u64 {
        let mut key_forward = 0;
        for i in 0..Self::WIDTH {
            self.partial_key3(&mut key_forward, i)
        }
        let mut key_reverse = 0;
        for i in (0..Self::WIDTH).rev() {
            self.partial_key3(&mut key_reverse, i)
        }
        // println!(
        //     "mask: {}, curr: {}, forward: {key_forward}, reverse: {key_reverse}",
        //     self.mask, self.current_position
        // );
        std::cmp::min(key_forward, key_reverse) / 3
    }
    pub fn partial_key3(&self, key: &mut u64, col: usize) {
        let mut p = 1_u64 << (col * (Self::HEIGHT + 1));
        while p & self.mask != 0 {
            *key *= 3;
            if p & self.current_position != 0 {
                *key += 1
            } else {
                *key += 2
            }
            p <<= 1;
        }
        *key *= 3;
    }
    pub fn possible_non_loosing_moves(&self) -> u64 {
        assert!(!self.has_winning_move());
        let possible_mask = self.possible();
        let opponent_win = self.opponent_winning_position();
        let forced_moves = possible_mask & opponent_win;

        match forced_moves {
            n if n == 0 => possible_mask & !(opponent_win >> 1),
            n if n & (n - 1) > 0 => 0,
            _ => forced_moves & !(opponent_win >> 1),
        }
    }

    fn can_play(&self, col: usize) -> bool {
        self.mask & Self::top_mask(col) == 0
    }

    pub fn next_pos(&self, col: usize) -> Self {
        let m = self.mask + Self::bottom_mask(col) & Self::column_mask(col);
        self.next_pos_move(m)
    }

    pub fn next_pos_move(&self, m: u64) -> Self {
        let moves = self.moves + 1;
        let current_position = self.mask ^ self.current_position;
        let mask = self.mask | m;
        Self {
            moves,
            current_position,
            mask,
        }
    }
    fn get_current_player(&self) -> usize {
        1 + self.moves % 2
    }

    pub fn has_winning_move(&self) -> bool {
        (self.winning_position() & self.possible()) != 0
    }
    // check only after winning move is checked
    pub fn is_draw(&self) -> bool {
        self.moves == Self::WIDTH * Self::HEIGHT - 2
    }
    pub fn calc_score(&self) -> isize {
        ((Self::WIDTH * Self::HEIGHT) as isize - self.moves as isize) / 2
    }
    pub fn is_winning_move(&self, col: usize) -> bool {
        self.winning_position() & self.possible() & Self::column_mask(col) != 0
    }
    fn possible(&self) -> u64 {
        (self.mask + Self::BOTTOM_MASK) & Self::BOARD_MASK
    }
    fn winning_position(&self) -> u64 {
        Self::compute_winning_position(self.current_position, self.mask)
    }
    fn opponent_winning_position(&self) -> u64 {
        Self::compute_winning_position(self.current_position ^ self.mask, self.mask)
    }
    pub fn move_score(&self, m: u64) -> usize {
        Self::pop_count(
            Self::compute_winning_position(self.current_position | m, self.mask),
            0,
        )
    }
    pub fn pop_count(m: u64, c: usize) -> usize {
        match m {
            0 => c,
            _ => Self::pop_count(m & (m - 1), c + 1),
        }
    }

    fn compute_winning_position(position: u64, mask: u64) -> u64 {
        // vertical
        let mut r = (position << 1) & (position << 2) & (position << 3);

        // horizontal
        let mut p = (position << (Self::HEIGHT + 1)) & (position << 2 * (Self::HEIGHT + 1));
        r |= p & (position << 3 * (Self::HEIGHT + 1));
        r |= p & (position >> (Self::HEIGHT + 1));
        p = (position >> (Self::HEIGHT + 1)) & (position >> 2 * (Self::HEIGHT + 1));
        r |= p & (position << (Self::HEIGHT + 1));
        r |= p & (position >> 3 * (Self::HEIGHT + 1));

        //diagonal 1
        p = (position << Self::HEIGHT) & (position << 2 * Self::HEIGHT);
        r |= p & (position << 3 * Self::HEIGHT);
        r |= p & (position >> Self::HEIGHT);
        p = (position >> Self::HEIGHT) & (position >> 2 * Self::HEIGHT);
        r |= p & (position << Self::HEIGHT);
        r |= p & (position >> 3 * Self::HEIGHT);

        //diagonal 2
        p = (position << (Self::HEIGHT + 2)) & (position << 2 * (Self::HEIGHT + 2));
        r |= p & (position << 3 * (Self::HEIGHT + 2));
        r |= p & (position >> (Self::HEIGHT + 2));
        p = (position >> (Self::HEIGHT + 2)) & (position >> 2 * (Self::HEIGHT + 2));
        r |= p & (position << (Self::HEIGHT + 2));
        r |= p & (position >> 3 * (Self::HEIGHT + 2));

        r & (Self::BOARD_MASK ^ mask)
    }
    // static bitmaps
    const BOTTOM_MASK: u64 = 4432676798593; //Self::bottom(Self::WIDTH as u64, Self::HEIGHT as u64);
    const BOARD_MASK: u64 = Self::BOTTOM_MASK * ((1_u64 << Self::HEIGHT) - 1);

    // mask functions
    fn top_mask(col: usize) -> u64 {
        1_u64 << ((Self::HEIGHT - 1) + col * (Self::HEIGHT + 1))
    }
    fn bottom_mask(col: usize) -> u64 {
        1_u64 << col * (Self::HEIGHT + 1)
    }

    pub fn column_mask(col: usize) -> u64 {
        ((1_u64 << Self::HEIGHT) - 1) << col * (Self::HEIGHT + 1)
    }

    // broken function /*TODO*/
    const fn bottom(height: u64, width: u64) -> u64 {
        match width {
            0 => 0,
            _ => Self::bottom(width - 1, height) | 1_u64 << (width - 1) * (height + 1),
        }
    }
}

pub struct MoveSorter {
    size: usize,
    entries: [SortEntry; Position::WIDTH],
}

impl MoveSorter {
    pub fn new() -> Self {
        Self {
            size: 0,
            entries: [SortEntry::new(); Position::WIDTH],
        }
    }
    pub fn add(&mut self, m: u64, score: usize) {
        let mut p = self.size;
        // (1..=self.size).rev()
        //     .map(|pos| (pos,self.entries))
        //     .take_while(|(pos,entries)| entries[pos-1].score > score)
        //     .for_each(|(pos,mut entries)| {
        //         entries[pos] = entries[pos-1];
        //         p -=1;
        //     });
        while p != 0 && self.entries[p - 1].score > score {
            self.entries[p] = self.entries[p - 1];
            p -= 1;
        }
        self.entries[p].m = m;
        self.entries[p].score = score;
        self.size += 1;
    }
    pub fn get_next(&mut self) -> Option<u64> {
        match self.size {
            0 => None,
            _ => {
                self.size -= 1;
                Some(self.entries[self.size].m)
            }
        }
    }
    pub fn reset(&mut self) {
        self.size = 0;
    }
}

#[derive(Copy, Clone, Debug)]
struct SortEntry {
    m: u64,
    score: usize,
}
impl SortEntry {
    fn new() -> Self {
        Self { m: 0, score: 0 }
    }
}

use crate::transposition_table::BookTranspositionTable;
use std::error::Error;
use std::fs::File;
pub struct OpeningBook {
    pub table: BookTranspositionTable,
    depth: usize,
}
impl OpeningBook {
    pub fn new() -> Self {
        Self {
            table: BookTranspositionTable::new(),
            depth: 0,
        }
    }
    pub fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file: File = File::open(file_path)?;
        let depth = file
            .borrow_mut()
            .take(6)
            .bytes()
            .flatten()
            .inspect(|b| println!("{b:?}"))
            .collect::<Vec<u8>>()[2];
        let mut keys = vec![0; BookTranspositionTable::SIZE];
        file.read_exact(&mut keys)?;
        let mut values = vec![0; BookTranspositionTable::SIZE];
        file.read_exact(&mut values)?;
        let bytes = file.borrow_mut().take(4).bytes().count();
        println!("num bytes: {bytes}, depth: {depth}");
        Ok(Self {
            table: BookTranspositionTable::create(keys, values),
            depth: depth as usize,
        })
    }
    pub fn get(&self, pos: &Position) -> Option<u8> {
        if pos.moves > self.depth {
            None
        } else {
            match self.table.get(pos.key3()) {
                0 => None,
                n => Some(n),
            }
        }
    }
}
