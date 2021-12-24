use anyhow::{Context, Ok, Result};
use itertools::Itertools;

fn parse_pixels(line: &str) -> impl Iterator<Item = Result<bool>> + '_ {
    let bytes = line.trim().bytes();
    bytes.map(|c| match c {
        b'#' => Ok(true),
        b'.' => Ok(false),
        _ => anyhow::bail!("unknown char: {}", c),
    })
}

fn simulate(algo: &[bool], img: Vec<bool>, width: usize, infty: bool) -> (Vec<bool>, usize, bool) {
    let (w, h) = (width as i64, (img.len() / width) as i64);
    let mut new_img = vec![false; ((w + 2) * (h + 2)) as usize];

    let mut prev_row = vec![511 * infty as usize; width + 2];
    for y in 0..h + 2 {
        for x in 0..w + 2 {
            let mut idx = prev_row[x as usize];
            for dx in [-2, -1, 0] {
                idx <<= 1;
                if 0 <= x + dx && x + dx < w && 0 <= y && y < h {
                    idx |= img[(y * w + x + dx) as usize] as usize;
                } else {
                    idx |= infty as usize;
                }
            }
            idx &= 511;

            new_img[(y * (w + 2) + x) as usize] = algo[idx];
            prev_row[x as usize] = idx;
        }
    }

    (new_img, width + 2, algo[511 * infty as usize])
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let start = std::time::Instant::now();
    let (algo, image) = input.split_once('\n').context("no newline")?;
    let algo: Vec<bool> = parse_pixels(algo).try_collect()?;
    let width = image.trim().split_once('\n').context("no image")?.0.len();
    let image: Vec<bool> = image.lines().flat_map(|l| parse_pixels(l)).try_collect()?;

    let mut state = (image, width, false);
    state = simulate(&algo, state.0, state.1, state.2);
    state = simulate(&algo, state.0, state.1, state.2);
    let part1 = state.0.iter().map(|x| *x as u64).sum::<u64>();
    for _ in 2..50 {
        state = simulate(&algo, state.0, state.1, state.2);
    }
    let part2 = state.0.iter().map(|x| *x as u64).sum::<u64>();

    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
