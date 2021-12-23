use anyhow::{Ok, Result};
use itertools::Itertools;


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day01.txt")?;
    let start = std::time::Instant::now();
    let depths: Vec<i32> = input.lines().map(|l| Ok(l.parse()?)).try_collect()?;
    let part1 = depths.windows(2).filter(|w| w[1] > w[0]).count();
    let part2 = depths.windows(4).filter(|w| w[3] > w[0]).count();
    // Logical equivalent for part 2: w[3] + w[2] + w[1] > w[2] + w[1] + w[0]

    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
