use anyhow::{Context, Result};
use itertools::{iproduct, Itertools};
use regex::Regex;


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day21.txt")?;
    let re =
        Regex::new(r"Player 1 starting position:\s*(\d)\s*Player 2 starting position:\s*(\d)\s*")?;
    let parsed = re.captures(&input).context("could not parse input")?;
    let positions = parsed.iter().skip(1).map(|c| c.unwrap().as_str().parse::<u64>().unwrap());
    let (p1, p2) = positions.collect_tuple().unwrap();

    let mut state = [p1 - 1, p2 - 1];
    let mut scores = [0u64; 2];
    let mut turn = 0u64;
    for roll in (1..=100).cycle().chunks(3).into_iter().map(|c| c.sum::<u64>()) {
        let player = (turn % 2) as usize;
        state[player] = (state[player] + roll) % 10;
        scores[player] += 1 + state[player];
        turn += 1;
        if scores[player] >= 1000 {
            break;
        }
    }
    println!("part1: {}", turn * 3 * scores[(turn % 2) as usize]);


    let throws = iproduct!(1..=3, 1..=3, 1..=3).map(|t| t.0 + t.1 + t.2);
    let distinct_throws = throws.counts().into_iter().collect_vec();
    let chronological_unfinished_scores =
        iproduct!(0..=20u64, 0..=20).sorted_by_key(|(p1, p2)| (*p1.max(p2), *p1.min(p2)));

    // Bottom-up dynamic programming on the occurrence count.
    // State space: scores: [0, 21)^2, positions: [0, 10)^2, turn: [0, 1].
    let idx = |p1_score, p2_score, turn, p1_pos, p2_pos| {
        (p1_score + 21 * (p2_score + 21 * (turn as u64 + 2 * (p1_pos + 10 * p2_pos)))) as usize
    };
    let mut partial_counts = vec![0u64; 21 * 21 * 10 * 10 * 2];
    partial_counts[idx(0, 0, 0, p1 - 1, p2 - 1)] = 1;

    let mut wins = [0u64; 2];
    for scores in chronological_unfinished_scores.map(|(a, b)| [a, b]) {
        for (turn, pos) in iproduct!([0, 1], 0..10, 0..10).map(|(t, a, b)| (t, [a, b])) {
            let count = partial_counts[idx(scores[0], scores[1], turn, pos[0], pos[1])];
            if count == 0 {
                continue;
            }

            for (throw, multiplicity) in distinct_throws.iter() {
                let mut next_pos = pos;
                let mut next_scores = scores;
                next_pos[turn] = (pos[turn] + 10 + throw) % 10;
                next_scores[turn] = scores[turn] + 1 + next_pos[turn];
                let ni = idx(next_scores[0], next_scores[1], 1 - turn, next_pos[0], next_pos[1]);

                if next_scores[turn] >= 21 {
                    wins[turn] += count * *multiplicity as u64;
                } else {
                    partial_counts[ni] += count * *multiplicity as u64;
                }
            }
        }
    }
    println!("part2: {}", wins[0].max(wins[1]));
    Ok(())
}
