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

    fn negamax_alpha_beta(&mut self, pos: Position, mut alpha: isize, mut beta: isize) -> isize {
        self.node_count += 1;
        let max: isize = ((Position::HEIGHT*Position::WIDTH) as isize-1-(pos.moves as isize))/2;
        if pos.all_moves_played() {
            0
        } else if pos.has_winning_move() {
                pos.calc_score()
        } else {
            beta = if beta > max {max} else {beta};
            if alpha >= beta {
                beta
            } else {
                for c in (0..Position::WIDTH).filter(|&c| pos.can_play(c)) {
                    let score = -self.negamax_alpha_beta(pos.next_pos(c),-beta,-alpha);
                    if score >= beta {return score}
                    if score > alpha {alpha = score}
                }
                alpha
            }
        }
    }

    pub fn solve(&mut self, pos: Position) -> isize {
        self.node_count = 0;
        self.negamax_alpha_beta(pos, -((Position::HEIGHT*Position::WIDTH) as isize), (Position::HEIGHT*Position::WIDTH) as isize)
    }
}
