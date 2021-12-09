use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;


fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day01.txt")?);
    let depths = input.lines().map(|l| Ok(l?.parse()?)).collect::<Result<Vec<i32>>>()?;
    let part1 = depths.windows(2).filter(|w| w[1] > w[0]).count();
    let part2 = depths.windows(4).filter(|w| w[3] > w[0]).count();
    // Logical equivalent for part 2: w[3] + w[2] + w[1] > w[2] + w[1] + w[0]
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
