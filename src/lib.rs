pub use position::{OpeningBook, Position};
pub use solver::Solver;
pub use transposition_table::TranspositionTable;

pub mod bencher;
pub mod position;
pub mod solver;
pub mod transposition_table;

#[cfg(test)]
mod tests {
    use super::*;
    use position::OpeningBook;
    use solver::Solver;

    #[test]
    fn test_parsing() {
        let pos2 = Position::parse("4455454513231");
        println!("{:}", pos2);
        assert_eq!(true, true);
    }
    #[test]
    fn test_safe_parsing() {
        let pos = Position::parse_safe("000");
        assert!(pos.is_some());
        let pos = Position::parse_safe("000000");
        assert!(pos.is_some());
        let pos = Position::parse_safe("0000000");
        assert!(pos.is_none());
        let pos = Position::parse_safe("0123456");
        assert!(pos.is_some());
        let pos = Position::parse_safe("01234567");
        assert!(pos.is_none());
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
        assert_eq!(8, solver.solve(&pos, false));

        let pos = Position::parse("1233722555341451114725221333");
        println!("{pos}");
        assert_eq!(-1, solver.solve(&pos, false));

        let pos = Position::parse("2737772244262123677516643354");
        println!("{pos}");
        assert_eq!(0, solver.solve(&pos, false));
    }
    #[test]
    fn test_solver_iterative_deepening() {
        let solver = Solver::new();
        test_solver(solver);
    }
    fn opening_cmp(pos: &Position, book: &OpeningBook, solver: &mut Solver) {
        if let Some(n) = book.get(&pos) {
            let val = solver.solve(&pos, false);
            let val2 = (n as isize) + Position::MIN_SCORE - 1;
            println!("in book {val2}, solver: {val}");
            assert_eq!(val, val2);
        }
    }

    #[test]
    fn test_pop_count() {
        let count = Position::pop_count(4, 0);
        assert_eq!(1, count);
        let count = Position::pop_count(3, 0);
        assert_eq!(2, count);
    }
}
