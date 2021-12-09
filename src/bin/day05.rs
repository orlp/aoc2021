use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Ok, Result};
use itertools::Either::{Left, Right};
use itertools::Itertools;


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(i64, i64);

fn interpolate(p: Point, q: Point) -> impl Iterator<Item = Point> {
    let symmetric_range = |a, b| {
        if a <= b {
            Left(a..=b)
        } else {
            Right((b..=a).rev())
        }
    };
    let xs = symmetric_range(p.0, q.0).cycle();
    let ys = symmetric_range(p.1, q.1).cycle();
    let len = 1 + cmp::max((p.0 - q.0).abs(), (p.1 - q.1).abs());
    xs.zip(ys).map(|(x, y)| Point(x, y)).take(len as usize)
}

fn num_intersections(lines: impl IntoIterator<Item = (Point, Point)>) -> usize {
    let grid = lines.into_iter().flat_map(|(p, q)| interpolate(p, q)).counts();
    grid.into_values().filter(|n| *n >= 2).count()
}

fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day05.txt")?);
    let lines: Vec<(Point, Point)> = input
        .lines()
        .map(|line| {
            let line = line?;
            let (from, to) = line.split_once(" -> ").context("invalid line")?;
            let (x1, y1) = from.split_once(",").context("invalid point")?;
            let (x2, y2) = to.split_once(",").context("invalid point")?;
            Ok((Point(x1.parse()?, y1.parse()?), Point(x2.parse()?, y2.parse()?)))
        })
        .try_collect()?;

    let axis_aligned = lines.iter().copied().filter(|(p, q)| p.0 == q.0 || p.1 == q.1);
    println!("part1: {}", num_intersections(axis_aligned));
    println!("part2: {}", num_intersections(lines));
    Ok(())
}
