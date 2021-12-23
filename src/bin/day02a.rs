use anyhow::{Context, Result};


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day02.txt")?;
    let start = std::time::Instant::now();
    let mut horiz = 0i64;
    let mut depth = 0i64;
    for line in input.lines() {
        let (dir, n) = line.split_once(' ').context("split failed")?;
        let n: i64 = n.parse()?;
        match dir {
            "forward" => horiz += n,
            "down" => depth += n,
            "up" => depth -= n,
            _ => unreachable!(),
        }
    }
    println!("time: {:?}", start.elapsed());
    println!("{}", horiz * depth);
    Ok(())
}
