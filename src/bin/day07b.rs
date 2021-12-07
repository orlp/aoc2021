use anyhow::Result;
use itertools::Itertools;
use std::cmp;

/*
    Let f(a) = total fuel cost to align to position a.
    Note: 1 + 2 + 3 + ... + k = k*(k+1)/2.
    Minimize: f(a) = sum(abs(x_i - a)*(1 + abs(x_i - a))/2) for all x_i in positions.
    Decompose: f(a) = sum((x_i - a)^2) + sum(abs(x_i - a)) / 2
    Opt f'(a) = 0 : sum(x_i - a) + sum(-sgn(x_i - a)) / 2 = 0
    Rearrange:      sum(x_i) - sum(sgn(x_i - a)) / 2 = n*a
    Rearrange:      mean(x_i) - sum(sgn(x_i - a)) / (2*n) = a
    Bound:          mean(x_i) - 1/2 <= a <= mean(x_i) + 1/2
*/

fn cost(positions: &[i64], a: i64) -> i64 {
    positions.iter().map(|p| {
        let d = (p - a).abs();
        d * (d+1) / 2
    }).sum::<i64>()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let positions: Vec<i64> = input.trim().split(',').map(str::parse).try_collect()?;
    let mean_floor = positions.iter().sum::<i64>() / positions.len() as i64;
    let fuel_cost = cmp::min(cost(&positions, mean_floor), cost(&positions, mean_floor + 1));
    println!("{}", fuel_cost);
    Ok(())
}
