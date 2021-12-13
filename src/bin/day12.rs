use std::collections::{HashMap, HashSet};

use anyhow::{Context, Ok, Result};


fn count_paths<'a>(
    from: &'a str,
    to: &'a str,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    mut seen: &mut HashSet<&'a str>,
    allow_twice: bool,
) -> u64 {
    if from == to {
        1
    } else {
        edges
            .get(from)
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter_map(|next| {
                let big_node = next.chars().all(|c| c.is_ascii_uppercase());
                let unseen = !seen.contains(*next);
                let twice_exception = !big_node && !unseen && allow_twice && !(*next == "start");
                (big_node || unseen || twice_exception).then(|| {
                    seen.insert(next);
                    let res =
                        count_paths(next, to, edges, &mut seen, allow_twice & !twice_exception);
                    if !twice_exception {
                        seen.remove(next);
                    }
                    res
                })
            })
            .sum()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().split('\n') {
        let (a, b) = line.trim().split_once('-').context("line contains no edge")?;
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }

    let mut seen = HashSet::from(["start"]);
    println!("{}", count_paths("start", "end", &edges, &mut seen, false));
    println!("{}", count_paths("start", "end", &edges, &mut seen, true));

    Ok(())
}
