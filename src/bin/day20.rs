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

fn simulate(algo: &[bool], img: Vec<Vec<bool>>, infinity: bool) -> (Vec<Vec<bool>>, bool) {
    let (w, h) = (img[0].len() as i32, img.len() as i32);
    let neighborhood_bitvec = |x, y| {
        let mut idx = 0;
        for (dy, dx) in itertools::iproduct!([-1, 0, 1], [-1, 0, 1]) {
            idx <<= 1;
            if 0 <= x + dx && x + dx < w && 0 <= y + dy && y + dy < h {
                idx |= img[(y + dy) as usize][(x + dx) as usize] as usize;
            } else {
                idx |= infinity as usize;
            }
        }
        idx
    };

    let new_img = (0..h + 2)
        .map(|y| (0..w + 2).map(|x| algo[neighborhood_bitvec(x - 1, y - 1)]).collect_vec())
        .collect_vec();
    (new_img, algo[511 * infinity as usize])
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let (algo, image) = input.split_once('\n').context("no newline")?;
    let algo: Vec<bool> = parse_pixels(algo).try_collect()?;
    let image = image.trim().lines().map(|l| Ok(parse_pixels(l).try_collect()?)).try_collect()?;

    let mut state = (image, false);
    state = simulate(&algo, state.0, state.1);
    state = simulate(&algo, state.0, state.1);
    println!("part1: {}", state.0.iter().flatten().map(|x| *x as u64).sum::<u64>());
    for _ in 2..50 {
        state = simulate(&algo, state.0, state.1);
    }
    println!("part2: {}", state.0.iter().flatten().map(|x| *x as u64).sum::<u64>());
    Ok(())
}
