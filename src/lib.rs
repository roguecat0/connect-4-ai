use std::fmt;

mod solver;
pub mod bencher;

#[derive(Clone,Debug)]
struct Position {
    moves: usize,
    current_position: u64,
    mask: u64,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let str: String = (0..Position::HEIGHT)
        //     .rev().map(|r| {
        //     (0..Position::WIDTH)
        //         .map(|c| format!("{:?}, ",self.board[c][r]))
        //         .collect::<String>() + "\n"
        // })
        // .collect();
        write!(f,"player: {}\nboard:\n{}",self.get_current_player(),self.mask)
    }
}

impl Position {
    pub const HEIGHT: usize = 6;
    pub const WIDTH: usize = 7;

    fn new() -> Self {
        Self {
            moves: 0,
            current_position: 0,
            mask: 0,
        }
    }
    fn parse(code: &str) -> Self {
        code.chars()
            .flat_map(|c| c.to_digit(10).map(|d| d - 1))
            .fold(Position::new(), |acc, d|  acc.next_pos(d as usize))
    }
    fn parse_safe(code: &str) -> Option<Self> {
        code.chars().try_fold(Position::new(), |acc, c| {
            match c.to_digit(10) {
                Some(n) => Some(acc.next_pos(n as usize)),
                None => None,
            }
        })
    }
    fn can_play(&self, col: usize) -> bool {
        self.mask & Position::top_mask(col) == 0
    }

    fn next_pos(&self, col: usize) -> Self {
        let moves = self.moves + 1;
        let current_position = self.mask ^ self.current_position;
        let mask = self.mask | (self.mask + Position::bottom_mask(col));

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
            let m = pos & (pos >> (Position::HEIGHT + 1));
            m & ( m >> ( 2 * (Position::HEIGHT+ 1 ))) != 0
        };
        let diagonal1 = |pos| {
            let m = pos & (pos >> Position::HEIGHT);
            m & ( m >> ( 2 * Position::HEIGHT)) != 0
        };
        let diagonal2 = |pos| {
            let m = pos & (pos >> (Position::HEIGHT + 2));
            m & ( m >> ( 2 * (Position::HEIGHT+ 2 ))) != 0
        };
        let vertical = |pos| {
            let m: u64 = pos & (pos >> 1);
            m & ( m >> 2) != 0
        };
        horizontal(pos) || diagonal1(pos) || diagonal2(pos) || vertical(pos)

    }
    fn is_winning_move(&self, col: usize) -> bool {
        let pos: u64 = self.current_position 
            | (self.mask + Position::bottom_mask(col)) 
            & Position::column_mask(col);
        Position::alignment(pos)
    }
    fn has_winning_move(&self) -> bool {
        (0..Position::WIDTH)
            .filter(|&c| self.can_play(c))
            .fold(false, |acc, c| {self.is_winning_move(c) || acc })
    }
    fn all_moves_played(&self) -> bool {
        self.moves == Position::WIDTH*Position::HEIGHT
    }
    fn calc_score(&self) -> isize {
        ((Position::WIDTH*Position::HEIGHT + 1) as isize - self.moves as isize)/2
    }
    // mask functions
    fn top_mask(col: usize) -> u64 {
        (1_u64 << (Position::HEIGHT - 1)) << col * (Position::HEIGHT + 1)
    }
    fn bottom_mask(col: usize) -> u64 {
        1_u64 << col * (Position::HEIGHT + 1)
    }
    fn column_mask(col: usize) -> u64 {
        ((1_u64 << Position::HEIGHT) - 1) << col * (Position::HEIGHT + 1)
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
    #[test]
    fn test_solver_mod() {
        let mut solver = Solver::new();

        let pos = Position::parse("23163416124767223154467471272416755633");
        println!("test1");
        println!("{pos}");
        println!("winning move: {}, moves: {}, score: {}"
            ,pos.has_winning_move(), pos.moves, pos.calc_score());
        assert_eq!(0 ,solver.solve(pos));

        println!("test1");
        let pos = Position::parse("65214673556155731566316327373221417");
        println!("{pos}");
        assert_eq!(-1 ,solver.solve(pos));

        let pos = Position::parse("3642756176227637211322113551637574556");
        println!("{pos}");
        assert_eq!(2 ,solver.solve(pos));
    }
}
