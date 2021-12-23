use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use anyhow::{Context, Result};
use itertools::Itertools;

/*
    State: for logic simplicity we use 0, 1, 3, 5, 7, 9, 10 as the valid hallway positions.
    Indices 2, 4, 6, 8 are unused and always Empty, 11+ are the rooms, with each room contiguous.
    Value encoding: 0 = Empty, 1 = A, 2 = B, 3 = C, 4 = D.

        ###################################
        #00 01 02 03 04 05 06 07 08 09 10 #
        ###### 11 ## 13 ## 15 ## 17 #######
             # 12 ## 14 ## 16 ## 18 #
             ########################
*/

fn gen_moves<const N: usize>(state: &[u8; N], moves: &mut Vec<(usize, usize, usize)>) {
    let symm_range = |a: usize, b: usize| a.min(b)..=a.max(b);
    let room_size = (N - 11) / 4;
    let room_start = |r| 11 + r * room_size;
    let room_states = [0, 1, 2, 3].map(|r| &state[room_start(r)..room_start(r + 1)]);
    let room_first_occupied = room_states.map(|rs| rs.iter().position(|s| *s > 0));

    let try_gen_move = |moves: &mut Vec<_>, hallway: usize, room: usize, to_room: bool| {
        let path = symm_range(hallway, 2 + 2 * room);
        let path_len = path.clone().count();
        let unobstructed = || path.clone().all(|i| i == hallway || state[i] == 0);

        if to_room {
            let fill_ready = room_states[room].iter().all(|s| *s == 0 || *s as usize == room + 1);
            if fill_ready && unobstructed() {
                let vacant_idx = room_first_occupied[room].unwrap_or(room_size) - 1;
                moves.push((hallway, room_start(room) + vacant_idx, path_len + vacant_idx));
            }
        } else {
            if let Some(occupied_idx) = room_first_occupied[room] {
                let target_room = room_states[room][occupied_idx] as usize - 1;
                let between = symm_range(2 + 2 * room, 2 + 2 * target_room).contains(&hallway);
                if !(between && path_len > 2) && unobstructed() {
                    moves.push((room_start(room) + occupied_idx, hallway, path_len + occupied_idx));
                }
            }
        }
    };

    for hallway in [0, 1, 3, 5, 7, 9, 10] {
        if state[hallway] > 0 {
            try_gen_move(moves, hallway, state[hallway] as usize - 1, true);
        } else {
            for room in 0..4 {
                try_gen_move(moves, hallway, room, false);
            }
        }
    }
}

fn dijkstra_fuel_cost<const N: usize>(state: [u8; N]) -> Option<u64> {
    let mut to_visit = BinaryHeap::from([(Reverse(0), state)]);
    let mut min_cost = HashMap::new();
    let mut moves = Vec::new();
    min_cost.insert(state, 0);

    while let Some((cost, state)) = to_visit.pop() {
        if cost.0 > *min_cost.get(&state).unwrap_or(&u64::MAX) {
            continue; // This means we already visited this node earlier.
        } else if state.windows(2).all(|w| w[0] <= w[1]) {
            return Some(cost.0); // First time we visit a node is optimal - return.
        }

        gen_moves(&state, &mut moves);
        for (from, to, dist) in moves.drain(..) {
            let mut new_state = state;
            let amphi = new_state[from];
            new_state[to] = amphi;
            new_state[from] = 0;
            let new_cost = cost.0 + dist as u64 * 10u64.pow(amphi as u32 - 1);
            if new_cost < *min_cost.get(&new_state).unwrap_or(&u64::MAX) {
                min_cost.insert(new_state, new_cost);
                to_visit.push((Reverse(new_cost), new_state));
            }
        }
    }

    None
}

fn parse_state<const N: usize>(s: &str) -> [u8; N] {
    let mut state = [0u8; N];
    let room_size = (N - 11) / 4;
    for (i, b) in s.bytes().filter(|b| b'A' <= *b && *b <= b'D').enumerate() {
        state[11 + room_size * (i % 4) + i / 4] = b - b'A' + 1;
    }
    state
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day23.txt")?;
    let start = std::time::Instant::now();
    let mut part2_input = input.lines().collect_vec();
    part2_input.splice(3..3, ["#D#C#B#A#", "#D#B#A#C#"]);
    let part1 = dijkstra_fuel_cost(parse_state::<{ 11 + 2 * 4 }>(&input));
    let part2 = dijkstra_fuel_cost(parse_state::<{ 11 + 4 * 4 }>(&part2_input.join("")));

    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1.context("no part 1 solution")?);
    println!("part2: {}", part2.context("no part 2 solution")?);
    Ok(())
}
