use anyhow::{Context, Result};
use derive_more::TryInto;
// use rayon::prelude::*;
use hashbrown::HashSet;
use itertools::Itertools;

type Word = i32;
type Register = u8;
type RegFile = [Word; 4];

#[derive(Debug, Clone, Copy, TryInto)]
enum Operand {
    Immediate(Word),
    Register(Register),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

impl Operand {
    fn value(self, regs: &RegFile) -> Word {
        match self {
            Operand::Immediate(x) => x,
            Operand::Register(r) => regs[r as usize],
        }
    }
}

const INPUTS: [[i32; 9]; 2] = [[9, 8, 7, 6, 5, 4, 3, 2, 1], [1, 2, 3, 4, 5, 6, 7, 8, 9]];

type Cache = HashSet<RegFile>;
fn find_sat_input(
    code: &[Instr],
    mut regs: RegFile,
    mut ip: u32,
    known_bad: &mut [Cache],
    min: bool,
) -> Option<u64> {
    let (init_regs, init_ip) = (regs, ip);
    if known_bad[init_ip as usize].contains(&init_regs) {
        return None;
    }

    while let Some(instr) = code.get(ip as usize) {
        match instr {
            Instr::Inp(dst) => {
                // for input in INPUTS[min as usize] {
                for input in INPUTS[min as usize] {
                    let mut new_regs = regs;
                    new_regs[*dst as usize] = input;
                    if let Some(sat) = find_sat_input(code, new_regs, ip + 1, known_bad, min) {
                        return Some(10 * sat + input as u64);
                    }
                };
                known_bad[init_ip as usize].insert(init_regs);
                return None;
            },
            Instr::Add(dst, src) => regs[*dst as usize] += src.value(&regs),
            Instr::Mul(dst, src) => regs[*dst as usize] *= src.value(&regs),
            Instr::Div(dst, src) => regs[*dst as usize] /= src.value(&regs),
            Instr::Mod(dst, src) => regs[*dst as usize] %= src.value(&regs),
            Instr::Eql(dst, src) => {
                regs[*dst as usize] = (regs[*dst as usize] == src.value(&regs)) as Word
            },
        }
        ip += 1;
    }

    let res = (regs[3] == 0).then(|| 0);
    if res.is_none() {
        known_bad[init_ip as usize].insert(init_regs);
    }
    res
}

fn parse_register(s: &str) -> Result<Register> {
    let is_reg = matches!(s, "w" | "x" | "y" | "z");
    is_reg.then(|| s.as_bytes()[0] - b'w').context("unknown register")
}

fn parse_operand(s: &str) -> Result<Operand> {
    let reg = parse_register(s).map(Operand::Register);
    reg.or_else(|_| Ok(Operand::Immediate(s.parse()?)))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day24.txt")?;
    let start = std::time::Instant::now();
    let mut instructions = Vec::new();
    for line in input.lines() {
        let components = line.split_ascii_whitespace().collect_vec();
        let instr = match &components[..] {
            &["inp", r] => Instr::Inp(parse_register(r)?),
            &["add", r, o] => Instr::Add(parse_register(r)?, parse_operand(o)?),
            &["mul", r, o] => Instr::Mul(parse_register(r)?, parse_operand(o)?),
            &["div", r, o] => Instr::Div(parse_register(r)?, parse_operand(o)?),
            &["mod", r, o] => Instr::Mod(parse_register(r)?, parse_operand(o)?),
            &["eql", r, o] => Instr::Eql(parse_register(r)?, parse_operand(o)?),
            _ => anyhow::bail!("unknown instruction"),
        };
        instructions.push(instr);
    }

    let p1start = std::time::Instant::now();
    let mut cache = vec![Cache::new(); instructions.len()];
    let part1 = find_sat_input(&instructions, [0; 4], 0, &mut cache, false);
    println!("part1 time: {:?}", p1start.elapsed());
    println!(
        "part1: {}",
        part1.context("no solution")?.to_string().chars().rev().join("")
    );
    let p2start = std::time::Instant::now();
    let mut known_bad = vec![Cache::new(); instructions.len()];
    let part2 = find_sat_input(&instructions, [0; 4], 0, &mut known_bad, true);
    println!("part2 time: {:?}", p2start.elapsed());
    println!(
        "part2: {}",
        part2.context("no solution")?.to_string().chars().rev().join("")
    );
    println!("time: {:?}", start.elapsed());
    Ok(())
}