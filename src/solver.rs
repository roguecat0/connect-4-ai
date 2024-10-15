use super::*;

pub fn print_something() {
    println!("something")
}

pub struct Solver {
    pub node_count: u64,
    column_order: [usize;Position::WIDTH],
    strategy: SolveStrat
}
pub enum SolveStrat {
    Naive,
    AlphaBeta,
    Weak,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            strategy: SolveStrat::AlphaBeta,
        }
    }
    pub fn with_strategy(strategy: SolveStrat) -> Self {
        Self {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            strategy,
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
                for c in self.column_order.into_iter().filter(|&c| pos.can_play(c)) {
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
        match self.strategy {
            SolveStrat::Weak => self.negamax_alpha_beta(pos, -1, 1),
            SolveStrat::Naive => self.negamax(pos),
            SolveStrat::AlphaBeta => self.negamax_alpha_beta(pos, 
                -((Position::HEIGHT*Position::WIDTH) as isize), 
                (Position::HEIGHT*Position::WIDTH) as isize)
        }

        
            
    }
}
