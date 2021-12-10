use std::cmp::Reverse;

use anyhow::Result;
use itertools::Itertools;


#[derive(Clone, Copy, Debug)]
struct UnionFindNode {
    parent: usize,
    size: usize,
}

fn make_set(size: usize, nodes: &mut Vec<UnionFindNode>) -> usize {
    let idx = nodes.len();
    nodes.push(UnionFindNode { parent: idx, size });
    idx
}

fn find(mut cur: usize, nodes: &mut [UnionFindNode]) -> usize {
    let mut parent = nodes[cur].parent;
    while cur != parent {
        let grandparent = nodes[parent].parent;
        nodes[cur].parent = grandparent;
        cur = parent;
        parent = grandparent;
    }
    cur
}

fn union(mut a: usize, mut b: usize, nodes: &mut [UnionFindNode]) -> usize {
    a = find(a, nodes);
    b = find(b, nodes);
    if a != b {
        if nodes[a].size < nodes[b].size {
            core::mem::swap(&mut a, &mut b);
        }
        nodes[b].parent = a;
        nodes[a].size += nodes[b].size;
    }
    a
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day09.txt")?;

    let mut union_find = Vec::new();
    let mut prev_row = Vec::new();
    let mut cur_row = Vec::new();
    for line in input.split('\n') {
        for (i, c) in line.trim().bytes().enumerate() {
            if c == b'9' {
                cur_row.push(None);
                continue;
            }
            let up = prev_row.get(i).copied().flatten();
            let left = cur_row.last().copied().flatten();
            let comp = match (up, left) {
                (Some(u), Some(l)) => union(u, l, &mut union_find),
                (Some(u), None) => find(u, &mut union_find),
                (None, Some(l)) => l, // We just found the left neighbor.
                (None, None) => make_set(0, &mut union_find),
            };
            union_find[comp].size += 1;
            cur_row.push(Some(comp));
        }
        core::mem::swap(&mut cur_row, &mut prev_row);
        cur_row.clear();
    }

    let basins = union_find
        .into_iter()
        .enumerate()
        .filter(|(i, c)| c.parent == *i)
        .map(|c| c.1.size);
    let result: usize = basins.map(|x| Reverse(x)).k_smallest(3).map(|r| r.0).product();
    println!("{}", result);
    Ok(())
}
