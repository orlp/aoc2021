use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day08.txt")?);
    let start = std::time::Instant::now();
    let counts = input.lines().map(|line| {
        let line = line?;
        let (_digits, display) = line.split_once(" | ").context("invalid line")?;
        Ok(display.split(' ').filter(|s| [2, 3, 4, 7].contains(&s.len())).count())
    });

    let answer = itertools::process_results(counts, |it| it.sum::<usize>())?;
    println!("time: {:?}", start.elapsed());
    println!("{}", answer);
    Ok(())
}
