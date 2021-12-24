use std::borrow::Cow;

use anyhow::{Context, Ok, Result};
use hashbrown::HashMap;
use itertools::Either;

const START: usize = 0;
const END: usize = 1;

fn count_paths<'a>(
    from: usize,
    edges: &[Vec<usize>],
    seen: &mut [bool],
    allow_twice: bool,
    cache: &mut HashMap<(usize, Cow<'a, [bool]>, bool), u64>,
) -> u64 {
    if from == END {
        return 1;
    } else if let Some(ret) = cache.get(&(from, Cow::Borrowed(seen), allow_twice)) {
        return *ret;
    }

    let mut total = 0;
    for next in edges[from].iter().copied() {
        if !seen[next] {
            seen[next] = true;
            total += count_paths(next, edges, seen, allow_twice, cache);
            seen[next] = false;
        } else if allow_twice && next != START {
            total += count_paths(next, edges, seen, false, cache);
        }
    }
    cache.insert((from, seen.iter().copied().collect(), allow_twice), total);
    total
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let start_time = std::time::Instant::now();
    let mut node_ids: HashMap<&str, usize> = [("start", START), ("end", END)].into_iter().collect();
    let mut edgelist: Vec<Vec<usize>> = vec![Vec::new(), Vec::new()];
    let mut big_node: Vec<bool> = vec![false, false];
    for line in input.lines() {
        let (a, b) = line.split_once('-').context("line contains no edge")?;
        let [an, bn] = [a, b].map(|name| {
            *node_ids.entry(name).or_insert_with(|| {
                let ret = edgelist.len();
                edgelist.push(Vec::new());
                big_node.push(name.chars().all(|c| c.is_ascii_uppercase()));
                ret
            })
        });
        edgelist[an].push(bn);
        edgelist[bn].push(an);
    }

    // Two big nodes can't be connected, it would make the answer unbounded.
    // Replace each edge to a big node with all nodes it connects to.
    for node in 0..edgelist.len() {
        edgelist[node] = core::mem::take(&mut edgelist[node])
            .into_iter()
            .flat_map(|next| {
                if big_node[next] {
                    Either::Left(edgelist[next].iter().copied())
                } else {
                    Either::Right([next].into_iter())
                }
            })
            .collect();
    }

    let mut seen = vec![false; node_ids.len()];
    seen[START] = true;
    let part1 = count_paths(START, &edgelist, &mut seen, false, &mut HashMap::new());
    let part2 = count_paths(START, &edgelist, &mut seen, true, &mut HashMap::new());
    println!("time: {:?}", start_time.elapsed());
    println!("{}", part1);
    println!("{}", part2);
    Ok(())
}
