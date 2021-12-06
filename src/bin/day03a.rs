use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};


fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day03.txt")?);
    let lines = input.lines().map(|l| Ok(l?)).collect::<Result<Vec<_>>>()?;
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

    println!("{}", epsilon * gamma);
    Ok(())
}
