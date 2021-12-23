use anyhow::{Context, Ok, Result};
use itertools::Itertools;


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day03.txt")?;
    let start = std::time::Instant::now();
    let lines = input.lines().collect_vec();
    let w = lines.first().context("empty input")?.len();
    let n = lines.len() as u64;

    let mut ones = vec![0u64; w];
    for line in lines {
        for (i, b) in line.bytes().enumerate() {
            ones[i] += (b == b'1') as u64;
        }
    }

    let mut gamma = 0u64;
    for count in ones {
        gamma <<= 1;
        gamma += (2 * count > n) as u64;
    }
    let epsilon = (!gamma) & ((1 << w) - 1);

    println!("time: {:?}", start.elapsed());
    println!("{}", epsilon * gamma);
    Ok(())
}
