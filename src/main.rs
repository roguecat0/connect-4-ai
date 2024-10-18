use connect_4_ai::bencher;
use connect_4_ai::solver::SolveStrat;
// use connect_4_ai::{TranspositionTable,Entry, Position};

fn main() {
    bencher::run(
        "datasets/Test_L3_R1", 
        SolveStrat::Transposition,
        false,
    );
}
