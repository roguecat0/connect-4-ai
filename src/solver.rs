use super::*;

pub fn print_something() {
    println!("something")
}

pub struct Solver {
    pub node_count: u64,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            node_count: 0
        }
    }
    fn negamax(&mut self, pos: Position) -> isize {
        self.node_count += 1;
        if pos.all_moves_played() {
            0
        } else if pos.has_winning_move() {
                pos.calc_score()
        } else {
            (0..Position::WIDTH)
                .filter(|&c| pos.can_play(c))
                .map(|c| -self.negamax(pos.next_pos(c)))
                .max()
                .expect("max of static mapped array")
        }
    }

    pub fn solve(&mut self, pos: Position) -> isize {
        self.node_count = 0;
        self.negamax(pos)
    }
}
