use anyhow::{Context, Ok, Result};
use arrayvec::ArrayVec;
use itertools::Itertools;

type SnailFish = ArrayVec<(u8, u8), 32>;

fn parse_fish(s: &[u8], depth: u8) -> Result<(SnailFish, usize)> {
    match s.get(0).context("unexpected eof")? {
        b'[' => {
            let (mut left, i) = parse_fish(&s[1..], depth + 1)?;
            anyhow::ensure!(s.get(1 + i) == Some(&b','));
            let (right, j) = parse_fish(&s[i + 2..], depth + 1)?;
            anyhow::ensure!(s.get(2 + i + j) == Some(&b']'));
            left.extend(right.into_iter());
            Ok((left, i + j + 3))
        },
        c @ b'0'..=b'9' => Ok(([(*c - b'0', depth)].into_iter().collect(), 1)),
        _ => anyhow::bail!("unexpected character"),
    }
}

// We reduce in two passes, the first only does explodes, then we split and
// explode as needed, in-line.
fn reduce_fish(mut v: SnailFish, first_pass: bool) -> SnailFish {
    let mut out = SnailFish::new();
    let mut i = 0;
    while i < v.len() {
        let (value, depth) = v[i];
        if depth >= 5 {
            // Explode in-place by undoing our last push to the output, allowing
            // us to split on it in the next iteration if necessary.
            let after_increment = v[i + 1].0;
            if let Some(after) = v.get_mut(i + 2) {
                after.0 += after_increment;
            }
            v[i + 1] = (0, depth - 1);
            if let Some(last_push) = out.pop() {
                v[i] = (last_push.0 + value, last_push.1);
            } else {
                i += 1;
            }
        } else if !first_pass && value >= 10 {
            // Try to split in place if possible by moving i back.
            if i > 0 {
                i -= 1;
                v[i] = (value / 2, depth + 1);
            } else {
                v.insert(0, (value / 2, depth + 1));
            }
            v[i + 1] = (value - value / 2, depth + 1);
        } else {
            out.push((value, depth));
            i += 1;
        }
    }
    out
}

fn magnitude(v: SnailFish) -> u32 {
    let mut stack: ArrayVec<(u32, u8), 32> = ArrayVec::new();
    for (value, depth) in v {
        stack.push((value as u32, depth));
        while let Some(&[l, r]) = stack.get(stack.len().saturating_sub(2)..) {
            if l.1 == r.1 {
                stack.pop();
                stack.pop();
                stack.push((3 * l.0 + 2 * r.0, l.1 - 1));
            } else {
                break;
            }
        }
    }
    stack.pop().unwrap().0
}

fn add_fish(mut l: SnailFish, r: &SnailFish) -> SnailFish {
    l.try_extend_from_slice(r.as_slice()).unwrap();
    for (_value, depth) in &mut l {
        *depth += 1;
    }
    reduce_fish(reduce_fish(l, true), false)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day18.txt")?;
    let fish: Vec<SnailFish> =
        input.lines().map(|l| Ok(parse_fish(l.as_bytes(), 0)?.0)).try_collect()?;

    let final_value = fish.iter().cloned().reduce(|l, r| add_fish(l, &r)).map(magnitude);
    let pairs = fish.iter().tuple_combinations().flat_map(|(l, r)| [(l, r), (r, l)]);
    let max_sum = pairs.map(|(l, r)| magnitude(add_fish(l.clone(), r))).max();
    println!("part 1: {}", final_value.context("no snailfish")?);
    println!("part 2: {}", max_sum.context("no snailfish")?);
    Ok(())
}
