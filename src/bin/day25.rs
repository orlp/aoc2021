use anyhow::{Context, Result};

fn step(cucumbers: &mut Vec<u8>, width: usize) -> bool {
    let height = cucumbers.len() / width;
    let mut stuck = true;
    for (dx, dy, kind) in [(1, 0, b'>'), (0, 1, b'v')] {
        let mut new = cucumbers.clone();
        for y in 0..height {
            let ny = ((y + dy) % height) * width;
            for x in 0..width {
                let i = y * width + x;
                let j = if x + dx == width { ny } else { ny + x + dx };
                if cucumbers[i] == kind && cucumbers[j] == b'.' {
                    new[i] = b'.';
                    new[j] = kind;
                    stuck = false;
                }
            }
        }
        *cucumbers = new;
    }
    stuck
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day25.txt")?;
    let start = std::time::Instant::now();
    let width = input.split_once('\n').context("no line")?.0.trim().len();
    let mut cucumbers: Vec<u8> = input.lines().flat_map(|l| l.bytes()).collect();

    let mut n = 1;
    while !step(&mut cucumbers, width) {
        n += 1;
    }

    println!("time: {:?}", start.elapsed());
    println!("{}", n);
    Ok(())
}
