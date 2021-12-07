use anyhow::Result;
use itertools::Itertools;
use std::cmp;

/*
    Let fuel(a) = total fuel cost to align to position a.
    Note: 1 + 2 + 3 + ... + k = k*(k+1)/2.
    fuel(a) = sum(abs(x_i - a)(1 + abs(x_i - a))) / 2
    fuel_left(a) = sum((a - x_i)(1 + a - x_i) for x_i < a) / 2    partition
    fuel_right(a) = sum((x_i - a)(1 + x_i - a) for x_i >= a) / 2
    fuel_left'(a) = sum(2(a - x_i) + 1 for x_i < a) / 2           differentiate
    fuel_right'(a) = sum(2(a - x_i) - 1 for x_i >= a) / 2
    fuel'(a) = sum(a - x_i) + (count(x_i < a) - count(x_i >= a)) / 2
    count(x_i >= a) = n - count(x_i < a)                          observation
    fuel'(a) = sum(a - x_i) + (2*count(x_i < a) - n) / 2
    fuel'(a) = n*a - sum(x_i) + count(x_i < a) - n/2

    n*a - sum(x_i) + count(x_i < a) - n/2 = 0                     fuel'(a) = 0
    a = mean(x_i) + 1/2 - count(x_i < a)/n

    Finally since count(x_i < a)/n lies in [0, 1] we have a in mean(x_i) +/- 1/2.
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
