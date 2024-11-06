use super::*;
use position::OpeningBook;
use solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::rc::Rc;
use std::time::{Duration, Instant};

fn read_file_to_string(file_path: &str) -> Result<String, IOError> {
    let mut s: String = String::new();
    let mut file: File = File::open(file_path)?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run(file_path: &str, weak: bool) {
    let book = Rc::new(OpeningBook::load("7x6.book").expect("book to load"));
    let mut solver = Solver::with_opening_book(Rc::clone(&book));

    if let Ok(s) = read_file_to_string(file_path) {
        let num_lines = s.lines().count();
        let res = s
            .lines()
            .flat_map(|line| {
                let vars = line.split_whitespace().collect::<Vec<_>>();
                vars.get(1)
                    .and_then(|numstr| numstr.parse().ok())
                    .map(|num: isize| (vars[0], num))
            })
            .enumerate()
            .map(|(i, (s, num))| {
                let pos = Position::parse(s);
                solver.reset();
                let before = Instant::now();
                let sol = solver.solve(&pos, weak);
                let elapsed: Duration = before.elapsed();
                println!(
                    "progress ... {:.2}%",
                    (i as f64 / num_lines as f64) * 100_f64
                );
                assert_eq!(sol, num);
                (solver.node_count, elapsed)
            })
            .fold((0, 0, Duration::ZERO), |acc, (nb, dur)| {
                (acc.0 + 1, acc.1 + nb, acc.2 + dur)
            });
        let res = (
            res.0,
            res.1 as f64 / res.0 as f64,
            res.2 / res.0,
            res.1 as u128 / res.2.as_millis(),
        );
        println!(
            "\ncount: {}, mean nb pos: {}, mean time: {:.2?}, K pos / s: {}",
            res.0, res.1, res.2, res.3,
        );
    } else {
        println!("couldn't read file");
    }
}
