use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let input = BufReader::new(File::open("inputs/day10.txt")?);
    let illegal_points = HashMap::from([(b')', 3u64), (b']', 57), (b'}', 1197), (b'>', 25137)]);
    let closer_points = HashMap::from([(b'(', 1u64), (b'[', 2), (b'{', 3), (b'<', 4)]);
    let openers = HashMap::from([(b')', b'('), (b']', b'['), (b'}', b'{'), (b'>', b'<')]);

    let mut total_illegal_score = 0;
    let mut incomplete_scores = Vec::new();
    for line in input.lines() {
        let mut stack = Vec::new();
        for b in line?.bytes() {
            if let Some(&opener) = openers.get(&b) {
                if stack.pop() != Some(opener) {
                    total_illegal_score += illegal_points[&b];
                    stack.clear();
                    break;
                }
            } else {
                stack.push(b);
            }
        }

        if !stack.is_empty() {
            let score = stack.into_iter().rev().fold(0, |tot, b| 5 * tot + closer_points[&b]);
            incomplete_scores.push(score);
        }
    }

    let num_incomplete = incomplete_scores.len();
    println!("part1: {}", total_illegal_score);
    println!("part2: {}", incomplete_scores.select_nth_unstable(num_incomplete / 2).1);
    Ok(())
}
