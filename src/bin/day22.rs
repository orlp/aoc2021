use std::cmp::{max, min};

use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use regex::Regex;

fn on_volume(cuboids: impl Iterator<Item = ([i64; 6], bool)>) -> i64 {
    let mut signed_volumes: Vec<([i64; 6], i64)> = Vec::new();
    let mut removed = 0;
    for new_cuboid in cuboids {
        for i in removed..signed_volumes.len() {
            let (other, other_sign) = signed_volumes[i];
            let [x1, x2, y1, y2, z1, z2] = new_cuboid.0;
            let [ox1, ox2, oy1, oy2, oz1, oz2] = other;
            let [ix1, ix2] = [max(x1, ox1), min(x2, ox2)];
            let [iy1, iy2] = [max(y1, oy1), min(y2, oy2)];
            let [iz1, iz2] = [max(z1, oz1), min(z2, oz2)];
            let intersection = [ix1, ix2, iy1, iy2, iz1, iz2];

            if ix1 <= ix2 && iy1 <= iy2 && iz1 <= iz2 {
                if other == intersection {
                    signed_volumes.swap(removed, i);
                    removed += 1;
                } else {
                    signed_volumes.push((intersection, -other_sign));
                }
            }
        }

        if new_cuboid.1 {
            signed_volumes.push((new_cuboid.0, 1));
        }
    }

    signed_volumes[removed..]
        .into_iter()
        .map(|(cub, sgn)| {
            let [x1, x2, y1, y2, z1, z2] = cub;
            (x2 + 1 - x1) * (y2 + 1 - y1) * (z2 + 1 - z1) * sgn
        })
        .sum()
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day22.txt")?;
    let start = std::time::Instant::now();
    let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")?;
    let cuboids: Vec<([i64; 6], bool)> = input
        .lines()
        .map(|line| {
            let cap = re.captures(line).context("could not parse cuboid")?;
            let mut groups = cap.iter().skip(1).map(|c| c.unwrap().as_str());
            let status = groups.next().unwrap();
            let (x1, x2, y1, y2, z1, z2) = groups.map(str::parse).collect_tuple().unwrap();
            Ok(([x1?, x2?, y1?, y2?, z1?, z2?], status == "on"))
        })
        .try_collect()?;

    let part1_cuboids = cuboids
        .iter()
        .filter(|(coords, _)| coords.iter().all(|x| -50 <= *x && *x <= 50))
        .copied();
    let (part1, part2) = (on_volume(part1_cuboids), on_volume(cuboids.into_iter()));
    println!("time: {:?}", start.elapsed());
    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
    Ok(())
}
