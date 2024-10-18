use super::*;

pub fn print_something() {
    println!("something")
}

pub struct Solver {
    pub node_count: u64,
    column_order: [usize;Position::WIDTH],
    strategy: SolveStrat,
    table: TranspositionTable,
}
pub enum SolveStrat {
    Naive,
    AlphaBeta,
    Weak,
    Transposition,
    IterativeDeepening,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            strategy: SolveStrat::Transposition,
            table: TranspositionTable::new()
        }
    }
    pub fn with_strategy(strategy: SolveStrat) -> Self {
        Self {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            strategy,
            table: TranspositionTable::new(),
        }
    }
    pub fn reset(&mut self) {
        self.node_count = 0;
        if let SolveStrat::Transposition = self.strategy {
            self.table.reset()
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
        beta = std::cmp::min(beta,max);
        if pos.all_moves_played() {
            0
        } else if pos.has_winning_move() {
                pos.calc_score()
        } else if alpha >= beta {
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

    fn negamax_transposition(&mut self, pos: Position, mut alpha: isize, mut beta: isize) -> isize {
        assert!(alpha < beta);
        self.node_count += 1;
        
        let max: isize = match self.table.get(pos.key()) {
            0 => {((Position::HEIGHT*Position::WIDTH) as isize-1-(pos.moves as isize))/2},
            score => { score as isize + Position::MIN_SCORE - 1},
        };
        beta = std::cmp::min(beta,max);

        if pos.all_moves_played() {
            0
        } else if pos.has_winning_move() {
                pos.calc_score()
        } else if alpha >= beta {
                beta
        } else {
            for p2 in self.column_order
                    .into_iter()
                    .filter(|&c| pos.can_play(c))
                    .map(|c| pos.next_pos(c)) {

                let score = -self.negamax_transposition(p2,-beta,-alpha);

                if score >= beta { return score }
                if score > alpha { alpha = score }
            }
            self.table.put(pos.key(), (alpha - Position::MIN_SCORE + 1) as u8);
            alpha
        }
    }

    fn iterative_deepening(&mut self,pos: Position, weak: bool) -> isize {
        let (mut min, mut max) = if !weak {
            (
                -((Position::WIDTH*Position::HEIGHT - pos.moves) as isize)/2,
                ((Position::WIDTH*Position::HEIGHT+1 - pos.moves)/2) as isize,
            )
        } else { (-1,1) };
        while min < max {
            let med = match min + (max - min) / 2 {
                med if med <= 0 && min/2 < med => min/2,
                med if med >= 0 && max/2 > med => max/2,
                med => med
            };
            let r = self.negamax_transposition(pos.clone(),med,med+1);
            if r <= med { max = r } else { min = r };
        }
        min
    }

    pub fn solve(&mut self, pos: Position, weak: bool) -> isize {
        match self.strategy {
            SolveStrat::Weak => self.negamax_alpha_beta(pos, -1, 1),
            SolveStrat::Naive => self.negamax(pos),
            SolveStrat::AlphaBeta => self.negamax_alpha_beta(pos, 
                -((Position::HEIGHT*Position::WIDTH) as isize), 
                (Position::HEIGHT*Position::WIDTH) as isize),
            SolveStrat::Transposition => self.negamax_transposition(pos, 
                -((Position::HEIGHT*Position::WIDTH) as isize), 
                (Position::HEIGHT*Position::WIDTH) as isize),
            SolveStrat::IterativeDeepening => self.iterative_deepening(pos,weak),
        }
    }
}
