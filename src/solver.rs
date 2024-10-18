use super::*;

pub fn print_something() {
    println!("something")
}

pub struct Solver {
    pub node_count: u64,
    column_order: [usize;Position::WIDTH],
    table: TranspositionTable,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            table: TranspositionTable::new()
        }
    }
    pub fn reset(&mut self) {
        self.node_count = 0;
        self.table.reset()
    }

    fn negamax(&mut self, pos: Position, mut alpha: isize, mut beta: isize) -> isize {
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

                let score = -self.negamax(p2,-beta,-alpha);

                if score >= beta { return score }
                if score > alpha { alpha = score }
            }
            self.table.put(pos.key(), (alpha - Position::MIN_SCORE + 1) as u8);
            alpha
        }
    }

    pub fn solve(&mut self, pos: Position, weak: bool) -> isize {
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
            let r = self.negamax(pos.clone(),med,med+1);
            if r <= med { max = r } else { min = r };
        }
        min
    }
}
