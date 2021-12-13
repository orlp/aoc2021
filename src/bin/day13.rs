use std::collections::HashSet;

use anyhow::Result;


fn apply_fold(fold: i64, coord: i64) -> i64 {
    if coord < fold {
        coord
    } else {
        fold - (coord - fold)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day13.txt")?;
    let mut first_fold = true;
    let mut coords: HashSet<(i64, i64)> = HashSet::new();
    for line in input.trim().split('\n').filter(|l| l.trim().len() > 0) {
        if let Some((x, y)) = line.split_once(",") {
            coords.insert((x.parse()?, y.parse()?));
        } else {
            if let Some(fx) = line.strip_prefix("fold along x=") {
                let fx = fx.parse()?;
                coords = coords.into_iter().map(|(x, y)| (apply_fold(fx, x), y)).collect();
            } else if let Some(fy) = line.strip_prefix("fold along y=") {
                let fy = fy.parse()?;
                coords = coords.into_iter().map(|(x, y)| (x, apply_fold(fy, y))).collect();
            }
            if first_fold {
                println!("part 1: {}", coords.len());
                first_fold = false;
            }
        }
    }

    println!("part 2:");
    let width = coords.iter().map(|(x, _y)| *x).max().unwrap_or(0);
    let height = coords.iter().map(|(_x, y)| *y).max().unwrap_or(0);
    for y in 0..=height {
        let line = (0..=width).map(|x| if coords.contains(&(x, y)) { "#" } else { " " });
        println!("{}", String::from_iter(line));
    }
    Ok(())
}
