use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Context, Result};


fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day02.txt")?);
    let mut horiz = 0i64;
    let mut depth = 0i64;
    for line in input.lines() {
        let line = line?;
        let (dir, n) = line.split_once(' ').context("split failed")?;
        let n: i64 = n.parse()?;
        match dir {
            "forward" => horiz += n,
            "down" => depth += n,
            "up" => depth -= n,
            _ => unreachable!(),
        }
    }
    println!("{}", horiz * depth);
    Ok(())
}