pub use position::MoveSorter;
pub use position::Position;
pub use transposition_table::{NaiveTranspositionTable, TranspositionTable};

pub mod bencher;
pub mod position;
pub mod solver;
pub mod transposition_table;

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
        // let pos2 = Position::parse("121212");
        println!("\n##IGNORE##");
        assert_eq!(true, pos2.has_winning_move());
    }
    fn test_solver(mut solver: Solver) {
        let pos = Position::parse("52753311433677442422121");
        println!("{pos}");
        // println!("winning move: {}, moves: {}, score: {}"
        // ,pos.has_winning_move(), pos.moves, pos.calc_score());
        assert_eq!(8, solver.solve(pos, false));

        let pos = Position::parse("1233722555341451114725221333");
        println!("{pos}");
        assert_eq!(-1, solver.solve(pos, false));

        let pos = Position::parse("2737772244262123677516643354");
        println!("{pos}");
        assert_eq!(0, solver.solve(pos, false));
    }
    #[test]
    fn test_solver_iterative_deepening() {
        let solver = Solver::new();
        test_solver(solver);
    }

    #[test]
    fn test_pop_count() {
        let count = Position::pop_count(4, 0);
        assert_eq!(1, count);
        let count = Position::pop_count(3, 0);
        assert_eq!(2, count);
    }
}
