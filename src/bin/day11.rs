use std::collections::HashSet;

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let width = input.lines().next().context("no lines")?.len() as isize;
    let mut energy_levels = input
        .lines()
        .flat_map(|line| line.trim().bytes().map(|b| b - b'0'))
        .collect_vec();
    let height = energy_levels.len() as isize / width;

    let mut total100 = 0;
    let mut flashed = HashSet::new();
    let mut step = 0usize;
    while flashed.len() as isize != width * height {
        for i in flashed.drain() {
            energy_levels[i] = 0;
        }

        let mut to_increase = itertools::iproduct!(0..width, 0..height).collect_vec();
        while let Some((x, y)) = to_increase.pop() {
            let i = (y * width + x) as usize;
            if !(x < 0 || x >= width || y < 0 || y >= height || flashed.contains(&i)) {
                energy_levels[i] += 1;
                if energy_levels[i] > 9 {
                    #[rustfmt::skip]
                    to_increase.extend([(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                                        (x - 1, y),                 (x + 1, y),
                                        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]);
                    flashed.insert(i);
                }
            }
        }

        step += 1;
        if step <= 100 {
            total100 += flashed.len();
        }
    }

    println!("part1: {}", total100 + step.saturating_sub(100) / 9);
    println!("part2: {}", step);
    Ok(())
}
