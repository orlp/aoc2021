use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;


fn winning_time(board: &[u64], width: usize, draw_time: &HashMap<u64, usize>) -> Option<usize> {
    let (mut maxcols, mut maxrows) = (vec![0; width], vec![0; width]);
    for (i, x) in board.iter().enumerate() {
        let t = *draw_time.get(x).unwrap_or(&usize::MAX);
        let (c, r) = (i % width, i / width);
        maxcols[c] = maxcols[c].max(t);
        maxrows[r] = maxrows[r].max(t);
    }

    let win_t = maxcols.into_iter().chain(maxrows).min().unwrap();
    (win_t < usize::MAX).then(|| win_t)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day04.txt")?;
    let (drawline, rest) = input.split_once('\n').context("no newline")?;

    let draws: Vec<u64> = drawline.split(',').map(|s| s.parse()).try_collect()?;
    let mut draw_t: HashMap<u64, usize> = HashMap::new();
    for (i, draw) in draws.iter().enumerate() {
        draw_t.entry(*draw).or_insert(i);
    }

    let boardnums: Vec<u64> = rest.split_ascii_whitespace().map(|n| n.parse()).try_collect()?;
    let winning_times =
        boardnums.chunks_exact(25).flat_map(|b| Some((winning_time(b, 5, &draw_t)?, b)));
    let (min, max) = winning_times.minmax().into_option().context("no winning board")?;

    let score = |(win_t, board): (usize, &[u64])| {
        draws[win_t] * board.iter().filter(|x| draw_t[x] > win_t).sum::<u64>()
    };
    println!("part1: {}", score(min));
    println!("part2: {}", score(max));
    Ok(())
}
