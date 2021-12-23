use anyhow::Result;
use itertools::Itertools;


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let start = std::time::Instant::now();
    let mut positions: Vec<i64> = input.trim().split(',').map(str::parse).try_collect()?;
    let n = positions.len();

    // True median minimizes mean absolute deviation.
    // In the case of an even number of elements we don't need the true median,
    // any integer in the range [sorted[n/2], sorted[n/2+1]] works.
    let lower_median = *positions.select_nth_unstable(n / 2).1;
    let answer = positions.iter().map(|l| (lower_median - *l).abs()).sum::<i64>();
    println!("time: {:?}", start.elapsed());
    println!("{}", answer);
    Ok(())
}
