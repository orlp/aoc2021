use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Ok, Result};
use itertools::Itertools;


/*
   8888    Regardless of how the ten unique signal patterns are ordered, the
  6    8   total number of times a segment is used remains unchanged. On the
  6    8   left is visualized how often each segment is used across the 10
   7777    digits. If we sum the counts of the active segments for a particular
  4    9   digit, we get a unique signature. For example, the signature of two
  4    9   is 8+8+7+4+7 = 34. Thus we only need to count how often each segment
   7777    is used, sum the relevant counts for each unknown digit and look up.
*/

fn main() -> Result<()> {
    let signatures = HashMap::from([
        (42, 0), (17, 1), (34, 2), (39, 3), (30, 4),
        (37, 5), (41, 6), (25, 7), (49, 8), (45, 9),
    ]);

    let input = BufReader::new(File::open("inputs/day08.txt")?);
    let displays = input.lines().map(|line| {
        let line = line?;
        let (unique, display) = line.split_once(" | ").context("invalid line")?;
        let unique_counts = unique.bytes().counts();
        Ok(display
            .split(' ')
            .map(|digit| signatures[&digit.bytes().map(|b| unique_counts[&b]).sum::<usize>()])
            .fold(0, |sum, digit| 10 * sum + digit))
    });

    println!("{}", itertools::process_results(displays, |it| it.sum::<usize>())?);
    Ok(())
}
