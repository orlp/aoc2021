use anyhow::{Context, Ok, Result};
use itertools::Itertools;

fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let byte_pairs = s.as_bytes().chunks(2).map(|b| std::str::from_utf8(b));
    let mut bytes: Vec<u8> = byte_pairs.map(|p| Ok(u8::from_str_radix(p?, 16)?)).try_collect()?;
    if s.len() % 2 != 0 {
        *bytes.last_mut().unwrap() <<= 4;
    }
    Ok(bytes)
}

fn read_bits_be(mut n: usize, bytes: &[u8], bits_read: &mut usize) -> Result<u64> {
    let mut res = 0;
    while n > 0 {
        let be_bits = bytes.get(*bits_read / 8).context("eof reached")? << (*bits_read % 8);
        let to_read = n.min(8 - *bits_read % 8);
        res <<= to_read;
        res |= (be_bits >> (8 - to_read)) as u64;
        *bits_read += to_read;
        n -= to_read;
    }
    Ok(res)
}

fn parse_literal(reader: &[u8], bits_read: &mut usize) -> Result<u64> {
    let mut num = 0;
    let mut block = 0b10000;
    while block >> 4 != 0 {
        block = read_bits_be(5, reader, bits_read)?;
        num <<= 4;
        num += block & 0b1111;
    }
    Ok(num)
}

fn parse_packet(reader: &[u8], bits_read: &mut usize) -> Result<(u64, u64)> {
    let mut version_sum = read_bits_be(3, reader, bits_read)?;
    let type_id = read_bits_be(3, reader, bits_read)?;
    if type_id == 4 {
        return Ok((version_sum, parse_literal(reader, bits_read)?));
    }

    let is_num_bits_mode = read_bits_be(1, reader, bits_read)? == 0;
    let limit = read_bits_be(if is_num_bits_mode { 15 } else { 11 }, reader, bits_read)? as usize;
    let mut value = None;
    let mut limit_status = 0;
    let subpackets_start = *bits_read;
    while limit_status < limit {
        let (inner_version_sum, inner_value) = parse_packet(reader, bits_read)?;
        version_sum += inner_version_sum;
        value = match type_id {
            0 => Some(value.unwrap_or(0) + inner_value),
            1 => Some(value.unwrap_or(1) * inner_value),
            2 => Some(value.unwrap_or(u64::MAX).min(inner_value)),
            3 => Some(value.unwrap_or(0).max(inner_value)),
            5 => value.map(|v| (v > inner_value) as u64).or(Some(inner_value)),
            6 => value.map(|v| (v < inner_value) as u64).or(Some(inner_value)),
            7 => value.map(|v| (v == inner_value) as u64).or(Some(inner_value)),
            _ => unreachable!(),
        };

        if is_num_bits_mode {
            limit_status = *bits_read - subpackets_start;
        } else {
            limit_status += 1;
        }
    }

    Ok((version_sum, value.unwrap()))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let bytes = decode_hex(input.trim())?;
    let mut bits_read = 0;
    let (version_sum, value) = parse_packet(&bytes, &mut bits_read)?;
    println!("part1: {:?}", version_sum);
    println!("part2: {:?}", value);
    Ok(())
}
