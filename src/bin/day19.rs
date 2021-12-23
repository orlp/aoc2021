use std::cmp::Ordering;
use hashbrown::{HashMap, HashSet};

use anyhow::{Context, Ok, Result};
use itertools::{iproduct, Itertools};

type Point = [i32; 3];

// Total parity of permutation and negative signs must be even.
// Note: parity of these permutations is [even, odd, even, odd, even, odd].
// Note: lowest bit of permutation index equals highest bit of the sign index.
#[rustfmt::skip] const PERMUTATIONS: [[usize; 3]; 6] =
    [[0, 1, 2], [0, 2, 1], [1, 2, 0], [1, 0, 2], [2, 0, 1], [2, 1, 0]];
#[rustfmt::skip] const SIGNS: [[i32; 3]; 8] = [
    [1, 1,  1], [-1, -1, 1], [-1, 1, -1], [ 1, -1, -1], // Even #negations.
    [1, 1, -1], [ 1, -1, 1], [-1, 1,  1], [-1, -1, -1], // Odd #negations.
];
pub fn rotate(p: Point, i: usize) -> Point {
    let [ax1, ax2, ax3] = PERMUTATIONS[i >> 2];
    let [s1, s2, s3] = SIGNS[i & 7];
    [s1 * p[ax1], s2 * p[ax2], s3 * p[ax3]]
}

fn point_add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn point_sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn manhattan_dist(a: Point, b: Point) -> u32 {
    ((a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()) as u32
}

// Compares two points as per their position on the Z-order curve.
fn cmp_z_order(lhs: Point, rhs: Point) -> Ordering {
    let lhs = lhs.map(|c| c.wrapping_sub(i32::MIN) as u32);
    let rhs = rhs.map(|c| c.wrapping_sub(i32::MIN) as u32);
    let is_msb_less = |x, y| x < y && x < (x ^ y);
    let is_dim_less = |i, j| is_msb_less(lhs[i] ^ rhs[i], lhs[j] ^ rhs[j]);
    let msd = if is_dim_less(0, 1) { 1 } else { 0 };
    let msd = if is_dim_less(msd, 2) { 2 } else { msd };
    lhs[msd].cmp(&rhs[msd])
}

// Computes differences between (i, i+1), ..., (i, i+k) for all i. If the
// scanner is sorted in Z-order this gives a good chance of overlap between
// scanners if they share common beacons.
fn window_diffs(scanner: &[Point], k: usize) -> impl Iterator<Item = (usize, Point)> + '_ {
    scanner
        .windows(k + 1)
        .enumerate()
        .flat_map(|(i, w)| w.iter().skip(1).map(move |p| (i, point_sub(*p, w[0]))))
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;
    let start = std::time::Instant::now();
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.starts_with("--- scanner") {
            scanners.push(Vec::new());
        } else {
            let scanner = scanners.last_mut().context("missing scanner header")?;
            let (x, y, z) = line.split(',').map(str::parse).collect_tuple().context("bad coord")?;
            scanner.push([x?, y?, z?]);
        }
    }

    let (refscan, scanners) = scanners.split_first_mut().context("no scanners")?;
    refscan.sort_unstable_by(|l, r| cmp_z_order(*l, *r));
    let mut scanner_positions = vec![[0, 0, 0]];
    let mut unknown_scanners: HashSet<usize> = (0..scanners.len()).collect();
    let mut current_rotation = 0;
    let mut rotations_since_overlap = 0;
    let mut known_points = HashSet::new();
    let mut known_diffs = HashMap::new();
    let mut rotated = Vec::new();
    while unknown_scanners.len() > 0 {
        let window_size = 1 + rotations_since_overlap / 24;
        for ui in unknown_scanners.clone() {
            if known_points.len() < refscan.len() {
                known_points.extend(refscan.iter().copied());
                known_diffs = window_diffs(refscan, window_size).map(|(i, d)| (d, i)).collect();
            }

            rotated.clear();
            rotated.extend(scanners[ui].iter().map(|p| rotate(*p, current_rotation)));
            rotated.sort_unstable_by(|l, r| cmp_z_order(*l, *r));
            let common_diffs = window_diffs(&rotated, window_size).filter_map(|(u, d)| {
                known_diffs.get(&d).map(|k| point_sub(refscan[*k], rotated[u]))
            });

            for translation in common_diffs {
                let translated = rotated.iter().map(|p| point_add(*p, translation)).collect_vec();
                let overlaps = translated.iter().filter(|p| known_points.contains(*p));
                if overlaps.count() >= 12 {
                    scanner_positions.push(translation);
                    refscan.extend(translated);
                    refscan.sort_unstable_by(|l, r| cmp_z_order(*l, *r));
                    refscan.dedup();
                    rotations_since_overlap = 0;
                    unknown_scanners.remove(&ui);
                    break;
                }
            }
        }

        current_rotation = (current_rotation + 1) % 24;
        rotations_since_overlap += 1;
    }

    let scanner_dists =
        iproduct!(&scanner_positions, &scanner_positions).map(|(p, q)| manhattan_dist(*p, *q));
    println!("time: {:?}", start.elapsed());
    println!("part1: {}", refscan.len());
    println!("part2: {}", scanner_dists.max().context("no scanners")?);
    Ok(())
}
