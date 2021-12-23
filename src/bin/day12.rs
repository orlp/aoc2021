use std::collections::{HashMap, HashSet};

use anyhow::{Context, Ok, Result};


fn count_paths<'a>(
    from: &'a str,
    to: &'a str,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashSet<&'a str>,
    allow_twice: bool,
) -> u64 {
    if from == to {
        1
    } else if let Some(neighbors) = edges.get(from) {
        neighbors
            .iter()
            .map(|next| {
                let big_node = next.chars().all(|c| c.is_ascii_uppercase());
                if big_node || !seen.contains(*next) {
                    seen.insert(next);
                    let res = count_paths(next, to, edges, seen, allow_twice);
                    seen.remove(next);
                    res
                } else if allow_twice && *next != "start" {
                    count_paths(next, to, edges, seen, false)
                } else {
                    0
                }
            })
            .sum()
    } else {
        0
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let start = std::time::Instant::now();
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let (a, b) = line.trim().split_once('-').context("line contains no edge")?;
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }

    let mut seen = HashSet::from(["start"]);
    let [part1, part2] = [false, true].map(|t| count_paths("start", "end", &edges, &mut seen, t));
    println!("time: {:?}", start.elapsed());
    println!("{}", part1);
    println!("{}", part2);
    Ok(())
}
