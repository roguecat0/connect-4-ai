use connect_4_ai::bencher;
// use connect_4_ai::{TranspositionTable,Entry, Position};

fn main() {
    bencher::run(
        "datasets/Test_L1_R1", 
        false,
    );
}
