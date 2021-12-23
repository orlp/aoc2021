use std::cmp::Ordering;

use anyhow::{Context, Result};
use itertools::{partition, Itertools};


fn partition_recursive<'a>(v: &mut [&'a str], i: usize, keep_most_common: bool) -> Option<&'a str> {
    if v.len() <= 1 {
        return v.get(0).cloned();
    }

    let split = partition(&mut v[..], |s| *s.as_bytes().get(i).unwrap() == b'0');
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
    let input = std::fs::read_to_string("inputs/day03.txt")?;
    let start = std::time::Instant::now();
    let mut lines = input.lines().collect_vec();
    let oxygen = partition_recursive(&mut lines[..], 0, true).context("could not find oxygen")?;
    let co2_scrubber =
        partition_recursive(&mut lines[..], 0, false).context("could not find co2 scrubber")?;
    let answer = u64::from_str_radix(&oxygen, 2)? * u64::from_str_radix(&co2_scrubber, 2)?;
    println!("time: {:?}", start.elapsed());
    println!("{}", answer);
    Ok(())
}
