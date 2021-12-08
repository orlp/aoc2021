use anyhow::{Context, Ok, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day08.txt")?);
    let counts = input.lines().map(|line| {
        let line = line?;
        let (_digits, display) = line.split_once(" | ").context("invalid line")?;
        Ok(display.split(' ').filter(|s| [2, 3, 4, 7].contains(&s.len())).count())
    });

    println!("{}", itertools::process_results(counts, |it| it.sum::<usize>())?);

    Ok(())
}
