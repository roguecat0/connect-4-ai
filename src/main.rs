use connect_4_ai::bencher;

fn main() {
    bencher::run("datasets/Test_L2_R1");
    // println!("output: {:?}",(0..7).map(|i| 7/2 + (1-2*(i%2))*(i+1)/2).collect::<Vec<_>>());
}
