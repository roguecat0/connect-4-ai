use connect_4_ai::bencher;
// use connect_4_ai::{NaiveTranspositionTable,Entry, Position};
use connect_4_ai::MoveSorter;

fn main() {
    // let mut moves = MoveSorter::new();
    // moves.add(0,1);
    // moves.add(3,3);
    // moves.add(1,2);
    // while let Some(n) = moves.get_next() {
    //     println!("move: {n}");
    // }

    bencher::run("datasets/Test_L1_R2", false);
}
