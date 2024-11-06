use connect_4_ai::{bencher, position, Position};
// use connect_4_ai::{NaiveTranspositionTable,Entry, Position};
use connect_4_ai::position::OpeningBook;

fn main() {
    bencher::run("datasets/Test_L1_R2", false);
}
