use connect_4_ai::bencher;
// use connect_4_ai::{TranspositionTable,Entry, Position};

fn main() {
    bencher::run(
        "datasets/Test_L2_R1", 
        false,
    );
}
