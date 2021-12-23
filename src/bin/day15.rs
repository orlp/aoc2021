use std::cmp::Reverse;
use std::collections::BinaryHeap;

use anyhow::{Context, Ok, Result};
use itertools::Itertools;


fn dijkstra_cross_grid(grid: &[u8], width: i64, height: i64, repeats: i64) -> Option<u64> {
    let mut to_visit = BinaryHeap::from([(Reverse(0), (0, 0))]);
    let mut min_cost = vec![u64::MAX; (width * repeats * height * repeats) as usize];
    min_cost[0] = 0;

    while let Some((cost, (x, y))) = to_visit.pop() {
        if cost.0 > min_cost[(y * width + x) as usize] {
            continue; // This means we already visited this node earlier.
        } else if (x, y) == (width * repeats - 1, height * repeats - 1) {
            return Some(cost.0); // First time we visit a node is optimal - return.
        }

        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if nx >= 0 && nx < repeats * width && ny >= 0 && ny < repeats * height {
                let ni = (ny * width + nx) as usize;
                let repeat_ni = ((ny % height) * width + (nx % width)) as usize;
                let repeat_factor = (nx / width + ny / height) as u64;
                let edge_cost = u64::from(grid[repeat_ni]);
                let n_cost = cost.0 + 1 + (edge_cost + repeat_factor - 1) % 9;
                if n_cost < min_cost[ni] {
                    min_cost[ni] = n_cost;
                    to_visit.push((Reverse(n_cost), (nx, ny)));
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day15.txt")?;
    let start = std::time::Instant::now();
    let grid = input.lines().flat_map(|l| l.bytes().map(|b| b - b'0')).collect_vec();
    let width = input.split_once('\n').context("no newline")?.0.trim().len() as i64;
    let height = (grid.len() / width as usize) as i64;

    let [part1, part2] = [1, 5].map(|repeats| dijkstra_cross_grid(&grid, width, height, repeats));
    println!("time: {:?}", start.elapsed());
    println!("part 1: {}", part1.context("no path")?);
    println!("part 2: {}", part2.context("no path")?);
    Ok(())
}
