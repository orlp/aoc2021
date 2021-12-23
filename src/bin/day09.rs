use anyhow::{Context, Result};
use itertools::Itertools;

/// Returns the size and lowest point of the basin at (x, y) if there is any.
/// Removes the basin in the process.
fn extract_basin(x: i64, y: i64, w: i64, h: i64, heights: &mut [u8]) -> Option<(u64, u8)> {
    let in_bounds = (0..w).contains(&x) && (0..h).contains(&y);
    (in_bounds && heights[(y * w + x) as usize] < 9).then(|| {
        let mut size = 1;
        let mut lowest = std::mem::replace(&mut heights[(y * w + x) as usize], 9);
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some((nsize, nlowest)) = extract_basin(nx, ny, w, h, heights) {
                size += nsize;
                lowest = lowest.min(nlowest);
            }
        }
        (size, lowest)
    })
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day09.txt")?;
    let start = std::time::Instant::now();
    let w = input.split_once('\n').context("no newline")?.0.trim().len() as i64;
    let h = input.trim().lines().count() as i64;
    let mut heights = input.lines().flat_map(|l| l.bytes().map(|b| b - b'0')).collect_vec();

    let mut basins = itertools::iproduct!(0..w, 0..h)
        .flat_map(|(x, y)| extract_basin(x, y, w, h, &mut heights))
        .collect_vec();
    let num_basins = basins.len();

    let part1: u64 = basins.iter().map(|(_size, lowest)| (1 + lowest) as u64).sum();
    let largest_basins = basins.select_nth_unstable(num_basins.saturating_sub(4)).2;
    let part2: Option<u64> = largest_basins.iter().map(|(size, _lowest)| *size).product1();
    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1);
    println!("part2: {}", part2.context("no basins found")?);
    Ok(())
}
