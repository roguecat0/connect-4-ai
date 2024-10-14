use std::fmt;

mod solver;
pub mod bencher;

#[derive(Clone,Debug)]
struct Position {
    moves: usize,
    board: [[usize;Position::HEIGHT];Position::WIDTH],
    height: [usize;Position::WIDTH],
    current_player: usize,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String = (0..Position::HEIGHT)
            .rev().map(|r| {
            (0..Position::WIDTH)
                .map(|c| format!("{:?}, ",self.board[c][r]))
                .collect::<String>() + "\n"
        })
        .collect();
        write!(f,"player: {}\nboard:\n{}",self.current_player,str)
    }
}

impl Position {
    pub const HEIGHT: usize = 6;
    pub const WIDTH: usize = 7;

    fn new() -> Self {
        Self {
            moves: 0,
            height:[0;Position::WIDTH],
            board:[[0;Position::HEIGHT];Position::WIDTH],
            current_player: 1
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
        col < Self::WIDTH && self.height[col] < Self::HEIGHT
    }

    fn next_pos(&self, col: usize) -> Self {
        let mut height: [usize;Position::WIDTH] = self.height.clone();
        let mut board = self.board.clone();
        let moves = self.moves + 1;
        board[col][height[col]] = 1 + self.moves % 2;
        height[col] += 1;

        Self {
            moves,
            height,
            board,
            current_player: 1 + moves % 2
        }
    }

    fn is_winning_move(&self, col: usize) -> bool {
        let current_player = 1 + self.moves % 2;
        let vertical = || self.board[col][..self.height[col]].iter().rev().take_while(|&&num| {num == self.current_player}).count() == 3;
        // println!("player: {current_player}, column: {col}");
        let front: Vec<(usize,&[usize;Position::HEIGHT])> = self.board[..col].iter().rev().enumerate().collect();
        let back: Vec<(usize,&[usize;Position::HEIGHT])>  = self.board[col..].iter().skip(1).enumerate().collect();
        let horizontal = || front.iter().map_while(|(i,c)| self.is_pos_curr_player(*i,c,col,0)).count()
        + back.iter().map_while(|(i,c)| self.is_pos_curr_player(*i,c,col,0)).count();
        let diagonal1 = || front.iter().map_while(|(i,c)| self.is_pos_curr_player(*i,c,col,1)).count()
        + back.iter().map_while(|(i,c)| self.is_pos_curr_player(*i,c,col,-1)).count();
        let diagonal2 = || front.iter().map_while(|(i,c)| self.is_pos_curr_player(*i,c,col,-1)).count()
        + back.iter().map_while(|(i,c)| self.is_pos_curr_player(*i,c,col,1)).count();
        // println!("{self}");
        // println!("hor: {horizontal:?}, dia1 {diagonal1:?}, dia2 {diagonal2:?}");
        vertical() || horizontal() > 2 || diagonal1() > 2 || diagonal2() > 2
    }
    fn has_winning_move(&self) -> bool {
        (0..Position::WIDTH)
            .filter(|&c| self.can_play(c))
            .fold(false, |acc, c| {self.is_winning_move(c) || acc })
    }

    fn is_pos_curr_player(&self, i: usize, column: &[usize; Position::HEIGHT], col: usize, dy: isize) -> Option<usize> {
        // println!("{column:?} col: {col}, i: {i}, dy: {dy}, formula: {}", self.height[col] as isize + (i as isize+1) * dy);
        match self.height[col] as isize + (i as isize+1) * dy {
            n if n < 0 => None,
            n if n as usize >= Position::HEIGHT => None,
            n => (column[n as usize] == self.current_player).then_some(self.current_player)
        }
    }

    fn all_moves_played(&self) -> bool {
        self.moves == Position::WIDTH*Position::HEIGHT
    }
    fn calc_score(&self) -> isize {
        ((Position::WIDTH*Position::HEIGHT+1) as isize - self.moves as isize)/2
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
        println!("{pos}");
        assert_eq!(0 ,solver.solve(pos));

        let pos = Position::parse("65214673556155731566316327373221417");
        println!("{pos}");
        assert_eq!(-1 ,solver.solve(pos));

        let pos = Position::parse("3642756176227637211322113551637574556");
        println!("{pos}");
        assert_eq!(2 ,solver.solve(pos));
    }
}
