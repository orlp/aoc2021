use std::cmp::Reverse;
use std::collections::BinaryHeap;

use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use regex::Regex;

/*
    v + (v - 1) + (v - 2) + ... = v*t - v*(t-1)/2
    \-------  t steps  -------/
*/

fn bound_triangular_steps(v: i64, min: i64, max: i64) -> (i64, i64) {
    // Find bounds for t when min <= v*t - t*(t-1)/2 <= max.
    // WolframAlpha gives us the solution - we take the first solution
    // ('upwards') for negative bounds, and the second ('downwards') for
    // positive bounds, when viewed on the parabola.
    let t = |bound| {
        let sqrt = ((4 * v * v + 4 * v - 8 * bound + 1) as f64).sqrt();
        (-sqrt.copysign(bound as f64) + 2.0 * v as f64 + 1.0) / 2.0
    };
    let (mut lower, mut upper) = (t(min), t(max));
    if upper < lower {
        core::mem::swap(&mut lower, &mut upper);
    }
    (lower.ceil() as i64, upper.floor() as i64)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day17.txt")?;
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;
    let caps = re.captures(&input).context("area not found")?;
    let bounds = caps.iter().skip(1).flatten().map(|c| c.as_str().parse::<i64>());
    let (xmin, xmax, ymin, ymax) =
        itertools::process_results(bounds, |it| it.collect_tuple())?.context("bad match count")?;

    let mut global_y_peak = 0;
    let mut valid_step_ranges = Vec::new();
    for yv in ymin..=-ymin {
        let (lo, hi) = bound_triangular_steps(yv, ymin, ymax);
        if lo <= hi {
            valid_step_ranges.push((lo, hi, false));
            global_y_peak = global_y_peak.max(yv * yv - yv * (yv - 1) / 2);
        }
    }
    let max_y_steps = valid_step_ranges.iter().map(|(_lo, hi, _is_x)| *hi).max().unwrap();

    for xv in 1..=xmax {
        let (lo, hi) = bound_triangular_steps(xv, xmin, xmax);
        let stopping_point = xv * xv - xv * (xv - 1) / 2;
        if xmin <= stopping_point && stopping_point <= xmax {
            // We stop inside the range - unbounded max steps.
            valid_step_ranges.push((lo, max_y_steps, true));
        } else if lo <= hi {
            valid_step_ranges.push((lo, hi, true));
        }
    }

    valid_step_ranges.sort_by_key(|(lo, _hi, _is_x)| *lo);
    let mut open_ranges: [BinaryHeap<Reverse<i64>>; 2] = Default::default();
    let mut intersections = 0;
    for (lo, hi, is_x) in valid_step_ranges {
        let other = &mut open_ranges[!is_x as usize];
        while other.peek().map(|e| e.0 < lo).unwrap_or(false) {
            other.pop();
        }
        intersections += other.len();
        open_ranges[is_x as usize].push(Reverse(hi));
    }

    println!("part1: {}", global_y_peak);
    println!("part2: {}", intersections);
    Ok(())
}
