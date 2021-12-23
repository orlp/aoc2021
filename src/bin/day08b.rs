use anyhow::{Context, Ok, Result};

fn parse_digit_segments(s: &str) -> u8 {
    // Parse seven segments into bitset.
    s.bytes().map(|b| 1 << (b - b'a')).sum()
}

fn decode_display<I: IntoIterator<Item = u8> + Clone>(digits: I, display: I) -> usize {
    // Decode in two passes, we can directly identify the 1 and 4 using just the
    // number of segments and the rest using the overlap with 1 and 4.
    let mut decoded = [0; 10];
    for digit in digits.clone() {
        match digit.count_ones() {
            2 => decoded[1] = digit,
            4 => decoded[4] = digit,
            3 => decoded[7] = digit,
            7 => decoded[8] = digit,
            _ => (),
        }
    }

    for digit in digits {
        let one_overlap = (digit & decoded[1]).count_ones();
        let four_overlap = (digit & decoded[4]).count_ones();
        match (digit.count_ones(), one_overlap, four_overlap) {
            (6, 2, 3) => decoded[0] = digit,
            (5, 1, 2) => decoded[2] = digit,
            (5, 2, 3) => decoded[3] = digit,
            (5, 1, 3) => decoded[5] = digit,
            (6, 1, 3) => decoded[6] = digit,
            (6, 2, 4) => decoded[9] = digit,
            _ => (),
        }
    }

    display
        .into_iter()
        .map(|digit| decoded.iter().position(|d| *d == digit).unwrap())
        .fold(0, |sum, digit| 10 * sum + digit)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day08.txt")?;
    let start = std::time::Instant::now();
    let displays = input.lines().map(|line| {
        let (digits, display) = line.split_once(" | ").context("invalid line")?;
        let [digits, display] = [digits, display].map(|s| s.split(' ').map(parse_digit_segments));
        Ok(decode_display(digits, display))
    });

    let answer = itertools::process_results(displays, |it| it.sum::<usize>())?;
    println!("time: {:?}", start.elapsed());
    println!("{}", answer);
    Ok(())
}
