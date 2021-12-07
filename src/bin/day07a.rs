use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let mut positions: Vec<i64> = input.trim().split(',').map(str::parse).try_collect()?;
    let n = positions.len();

    // Median minimizes mean absolute deviation.
    let (_less, lower_median, greater) = positions.select_nth_unstable(n / 2);
    let median = if n % 2 == 0 {
        let upper_median = greater.iter().min().context("no positions")?;
        (*lower_median + *upper_median) / 2
    } else {
        *lower_median
    };

    println!("{}", positions.iter().map(|l| (median - *l).abs()).sum::<i64>());
    Ok(())
}
