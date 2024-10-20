use super::*;
use crate::position::{Position,TranspositionTable, MoveSorter};

pub struct Solver {
    pub node_count: u64,
    column_order: [usize;Position::WIDTH],
    table: TranspositionTable,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            node_count: 0,
            column_order: [3, 4, 2, 5, 1, 6, 0],
            table: TranspositionTable::new()
        }
    }
    pub fn reset(&mut self) {
        self.node_count = 0;
        self.table.reset()
    }

    fn negamax(&mut self, pos: Position, mut alpha: isize, mut beta: isize) -> isize {
        assert!(alpha < beta);
        assert!(!pos.has_winning_move());
        self.node_count += 1;
        
        let max: isize = match self.table.get(pos.key()) {
            0 => {((Position::HEIGHT*Position::WIDTH) as isize-1-(pos.moves as isize))/2},
            score => { score as isize + Position::MIN_SCORE - 1},
        };
        let min: isize = -((Position::HEIGHT*Position::WIDTH) as isize - 2 - (pos.moves as isize))/2;

        alpha = std::cmp::max(alpha,min);
        beta = std::cmp::min(beta,max);
        let next = pos.possible_non_loosing_moves();

        if next == 0 {
            -pos.calc_score()
        } else if pos.is_draw() {
            0
        } else if alpha >= beta {
                beta
        } else {
            let mut moves = MoveSorter::new();
            self.column_order
                .into_iter()
                .flat_map(|c| {
                    match next & Position::column_mask(c) {
                        0 => None,
                        n => Some(n),
                    }
                })
                .for_each(|m| moves.add(m, pos.move_score(m)));

            // for p2 in self.column_order
            //         .into_iter()
            //         .filter(|&c| next & Position::column_mask(c) != 0)
            //         .map(|c| pos.next_pos(c)) {

            while let Some(m) = moves.get_next() {
                let p2 = pos.next_pos_move(m);
                let score = -self.negamax(p2,-beta,-alpha);

                if score >= beta { return score }
                if score > alpha { alpha = score }
            }
            self.table.put(pos.key(), (alpha - Position::MIN_SCORE + 1) as u8);
            alpha
        }
    }
    fn iterative_deepening(&mut self,pos: Position, mut min: isize,mut max: isize) -> isize {
        while min < max {
            let med = match min + (max - min) / 2 {
                med if med <= 0 && min/2 < med => min/2,
                med if med >= 0 && max/2 > med => max/2,
                med => med
            };
            let r = self.negamax(pos.clone(),med,med+1);
            if r <= med { max = r } else { min = r };
        }
        min
    }

    pub fn solve(&mut self, pos: Position, weak: bool) -> isize {
        let (min, max) = if !weak {
            (
                -((Position::WIDTH*Position::HEIGHT - pos.moves) as isize)/2,
                ((Position::WIDTH*Position::HEIGHT+1 - pos.moves)/2) as isize,
            )
        } else { (-1,1) };
        if pos.has_winning_move() { 
            ((Position::WIDTH*Position::HEIGHT + 1 - pos.moves) / 2) as isize
        } else {
            self.iterative_deepening(pos,min,max)
        }
    }
}
