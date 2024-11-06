use connect_4_ai::solver::Solver;
use connect_4_ai::{bencher, position, Position};
// use connect_4_ai::{NaiveTranspositionTable,Entry, Position};
use connect_4_ai::position::OpeningBook;
use std::rc::Rc;

fn main() {
    // bencher::run("datasets/Test_L1_R2", false);
    let book = Rc::new(OpeningBook::load("7x6.book").expect("loaded"));
    let mut solver = Solver::with_opening_book(book);
    let moves = "57";
    let weak = false;
    let pos = Position::parse(moves);
    let scores = solver.analyse(&pos, weak);
    let sol = solver.solve(&pos, weak);
    println!("pos: {moves}, scores: {scores:?}, solution: {sol}");
}
