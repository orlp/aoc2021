use std::collections::HashMap;

use anyhow::{Context, Ok, Result};
use itertools::Itertools;


fn parse_rule(rule: &str) -> Result<((u8, u8), u8)> {
    let (from, to) = rule.trim().split_once(" -> ").context("could not split rule")?;
    if let (&[a, b], &[c]) = (from.as_bytes(), to.as_bytes()) {
        Ok(((a, b), c))
    } else {
        anyhow::bail!("invalid rule")
    }
}

fn solve(polymer: &[u8], rules: &HashMap<(u8, u8), u8>, steps: usize) -> usize {
    // If the number of steps n were very large, we could solve in O(p^3 log(n)) steps using matrix
    // exponentiation instead, where p is the number of potential pairs.
    let mut state: HashMap<(u8, u8), usize> = polymer.iter().copied().tuple_windows().counts();
    for _ in 0..steps {
        let mut new_state = HashMap::new();
        for ((a, b), n) in state {
            if let Some(&c) = rules.get(&(a, b)) {
                *new_state.entry((a, c)).or_default() += n;
                *new_state.entry((c, b)).or_default() += n;
            } else {
                *new_state.entry((a, b)).or_default() += n;
            }
        }
        state = new_state;
    }

    let mut counts: HashMap<u8, usize> = HashMap::from([(polymer[0], 1)]);
    state.into_iter().for_each(|((_a, b), n)| *counts.entry(b).or_default() += n);
    let minmax = counts.values().minmax().into_option().unwrap();
    minmax.1 - minmax.0
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day14.txt")?;
    let (polymer, rest) = input.split_once('\n').context("could not get polymer")?;
    let rules = rest.trim().split('\n').map(parse_rule).try_collect()?;
    println!("{}", solve(polymer.trim().as_bytes(), &rules, 10));
    println!("{}", solve(polymer.trim().as_bytes(), &rules, 40));
    Ok(())
}
