use super::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::time::{Instant, Duration};

fn read_file_to_string(file_path: &str) -> Result<String,IOError> {
    let mut s: String = String::new();
    let mut file: File = File::open(file_path)?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run(file_path: &str) {
    let mut solver = solver::Solver::new();

    if let Ok(s) = read_file_to_string(file_path) {
        let res = s.lines()
            .flat_map(|line| {
                let vars = line.split_whitespace().collect::<Vec<_>>();
                vars
                    .get(1)
                    .and_then(|numstr| numstr.parse().ok())
                    .map(|num: isize| (vars[0],num))
            })
            .map(|(s,num)| {
                let pos = Position::parse(s);
                let before = Instant::now();
                let sol = solver.solve(pos);
                let elapsed: Duration = before.elapsed();
                println!("{s}, sol: {sol}, score: {num} nb: {}, time: {:.2?}",solver.node_count,elapsed);
                assert_eq!(sol,num);
                (solver.node_count,elapsed)
            })
            .fold((0,0,Duration::ZERO),|acc, (nb,dur)| (acc.0 + 1, acc.1 + nb,acc.2 + dur));
            // .collect::<Vec<_>>();
        let res = (
            res.0,
            res.1 as f64 / res.0 as f64,
            res.2 / res.0,
            res.1 as u128/ res.2.as_millis(),
        );
        println!("\ncount: {}, mean nb pos: {}, mean time: {:.2?}, K pos / s: {}",
            res.0, res.1, res.2, res.3);
    } else {
        println!("couldn't read file");
    }
}
