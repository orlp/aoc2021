use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use itertools::partition;

fn partition_recursive(v: &mut [String], i: usize, keep_most_common: bool) -> Option<String> {
    if v.len() <= 1 {
        return v.get(0).cloned();
    }

    let split = partition(&mut v[..], |s| s.bytes().nth(i).unwrap() == b'0');
    let zero_vs_one = (2 * split).cmp(&v.len());
    match (keep_most_common, zero_vs_one) {
        (true, Ordering::Less) | (true, Ordering::Equal) | (false, Ordering::Greater) => {
            partition_recursive(&mut v[split..], i + 1, keep_most_common)
        },
        (false, Ordering::Less) | (false, Ordering::Equal) | (true, Ordering::Greater) => {
            partition_recursive(&mut v[..split], i + 1, keep_most_common)
        },
    }
}

fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day03.txt")?);
    let mut lines = input.lines().map(|l| Ok(l?)).collect::<Result<Vec<_>>>()?;
    let oxygen = partition_recursive(&mut lines[..], 0, true).context("could not find oxygen")?;
    let co2_scrubber = partition_recursive(&mut lines[..], 0, false).context("could not find co2 scrubber")?;
    println!("{}", u64::from_str_radix(&oxygen, 2)? * u64::from_str_radix(&co2_scrubber, 2)?);
    Ok(())
}
