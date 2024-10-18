use std::fmt;

pub mod solver;
pub mod bencher;

#[derive(Clone,Debug)]
pub struct Position {
    moves: usize,
    current_position: u64,
    mask: u64,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"player: {}\nboard:\n{}",self.get_current_player(),self.mask)
    }
}

impl Position {
    pub const HEIGHT: usize = 6;
    pub const WIDTH: usize = 7;
    pub const MIN_SCORE: isize = -((Self::WIDTH*Self::HEIGHT) as isize) / 2 + 3;
    pub const MAX_SCORE: isize = ((Self::WIDTH*Self::HEIGHT) as isize + 1) / 2 - 3;

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
            .fold(Self::new(), |acc, d|  acc.next_pos(d as usize))
    }
    pub fn parse_safe(code: &str) -> Option<Self> {
        code.chars().try_fold(Self::new(), |acc, c| {
            match c.to_digit(10) {
                Some(n) => Some(acc.next_pos(n as usize)),
                None => None,
            }
        })
    }
    fn can_play(&self, col: usize) -> bool {
        self.mask & Self::top_mask(col) == 0
    }

    pub fn key(&self) -> u64 {
        self.current_position + self.mask
    }

    fn next_pos(&self, col: usize) -> Self {
        let moves = self.moves + 1;
        let current_position = self.mask ^ self.current_position;
        let mask = self.mask | (self.mask + Self::bottom_mask(col));

        Self {
            moves,
            current_position,
            mask,
        }
    }
    fn get_current_player(&self) -> usize {
        1 + self.moves % 2
    }

    fn alignment(pos: u64) -> bool {
        let horizontal = |pos| {
            let m = pos & (pos >> (Self::HEIGHT + 1));
            m & ( m >> ( 2 * (Self::HEIGHT+ 1 ))) != 0
        };
        let diagonal1 = |pos| {
            let m = pos & (pos >> Self::HEIGHT);
            m & ( m >> ( 2 * Self::HEIGHT)) != 0
        };
        let diagonal2 = |pos| {
            let m = pos & (pos >> (Self::HEIGHT + 2));
            m & ( m >> ( 2 * (Self::HEIGHT+ 2 ))) != 0
        };
        let vertical = |pos| {
            let m: u64 = pos & (pos >> 1);
            m & ( m >> 2) != 0
        };
        horizontal(pos) || diagonal1(pos) || diagonal2(pos) || vertical(pos)
    }
    fn is_winning_move(&self, col: usize) -> bool {
        let pos: u64 = self.current_position 
            | (self.mask + Self::bottom_mask(col)) 
            & Self::column_mask(col);
        Self::alignment(pos)
    }
    fn has_winning_move(&self) -> bool {
        (0..Self::WIDTH)
            .filter(|&c| self.can_play(c))
            .fold(false, |acc, c| {self.is_winning_move(c) || acc })
    }
    fn all_moves_played(&self) -> bool {
        self.moves == Self::WIDTH*Self::HEIGHT
    }
    fn calc_score(&self) -> isize {
        ((Self::WIDTH*Self::HEIGHT + 1) as isize - self.moves as isize)/2
    }
    // mask functions
    fn top_mask(col: usize) -> u64 {
        (1_u64 << (Self::HEIGHT - 1)) << col * (Self::HEIGHT + 1)
    }
    fn bottom_mask(col: usize) -> u64 {
        1_u64 << col * (Self::HEIGHT + 1)
    }
    fn column_mask(col: usize) -> u64 {
        ((1_u64 << Self::HEIGHT) - 1) << col * (Self::HEIGHT + 1)
    }
}

#[derive(Debug)]
pub struct TranspositionTable {
    table: Vec<Entry>,
    accessed: u64,
}

impl TranspositionTable {
    const SIZE: usize = 8388593; //8388593 == 64MB

    pub fn new() -> Self {
        Self {
            table: vec![Entry::empty();Self::SIZE],
            accessed: 0
        }
    }
    fn index(key: u64) -> usize {
        (key % Self::SIZE as u64) as usize
    }

    pub fn put(&mut self, key: u64, val: u8) {
        assert!(key < (1_u64 << 56));
        let i = Self::index(key);
        self.table[i] = Entry::create(key,val);
        self.accessed += 1;
    }

    pub fn get(&self, key: u64) -> u8 {
        assert!(key < (1_u64 << 56));
        let i = Self::index(key);
        if key == self.table[i].key() {
            self.table[i].value()
        } else {
            0
        }
    }
    pub fn reset(&mut self) {
        self.accessed = 0;
        self.table.iter_mut().for_each(|m| *m = Entry::new());
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Entry {
    key_val: u64
}
impl Entry {
    pub fn create(key: u64, val: u8) -> Self {
        let key_val = (key << 8) | val as u64;
        Self { key_val }
    }
    pub fn new() -> Self {Self::empty()}
    pub fn empty() -> Self {
        Self {
            key_val: 0
        }
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
        write!(f,"( key={}, val={}, key_val={:b} )",self.key(),self.value(), self.key_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solver::Solver;

    #[test]
    fn test_parsing() {
        let pos2 = Position::parse("4455454513231");
        println!("{:}", pos2);
        assert_eq!(true, true);
    }
    #[test]
    fn test_winning_move() {
        let pos2 = Position::parse("121212");
        assert_eq!(true, pos2.is_winning_move(0));
    }
    #[test]
    fn test_has_winning_move() {
        let pos2 = Position::parse("112233");
        assert_eq!(true, pos2.has_winning_move());
    }
    fn test_solver(mut solver: Solver) {
        let pos = Position::parse("52753311433677442422121");
        println!("{pos}");
        // println!("winning move: {}, moves: {}, score: {}"
            // ,pos.has_winning_move(), pos.moves, pos.calc_score());
        assert_eq!(8 ,solver.solve(pos,false));

        let pos = Position::parse("1233722555341451114725221333");
        println!("{pos}");
        assert_eq!(-1 ,solver.solve(pos,false));

        let pos = Position::parse("2737772244262123677516643354");
        println!("{pos}");
        assert_eq!(0 ,solver.solve(pos,false));
    }
    #[test]
    fn test_solver_iterative_deepening() {
        let solver = Solver::new();
        test_solver(solver);
    }
}
