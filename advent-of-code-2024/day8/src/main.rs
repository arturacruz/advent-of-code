use std::collections::{HashMap, HashSet};

use advent_of_code_lib::*;

type Vec2 = (isize, isize);
 
pub fn is_in_bounds(p: Vec2, size: Vec2) -> bool {
    p.0 >= 0 && p.0 < size.0 && p.1 >= 0 && p.1 < size.1
}

pub fn get_antinodes(p1: Vec2, p2: Vec2, size: Vec2) -> HashSet<Vec2> {
    let mut nodes = HashSet::new();

    let dist = (p2.0 - p1.0, p2.1 - p1.1);

    let mut i = 1;
    loop {
        let ant = (p1.0 - dist.0 * i, p1.1 - dist.1 * i);
        if !is_in_bounds(ant, size) {
            break;
        }
        nodes.insert(ant);
        i += 1;
    }

    i = 1;
    loop {
        let ant = (p2.0 + dist.0 * i, p2.1 + dist.1 * i);
        if !is_in_bounds(ant, size) {
            break;
        }
        nodes.insert(ant);
        i += 1;
    }

    nodes
}

fn main() {
    let input = get_input(InputType::Input);

    let mut frequencies: HashMap<char, Vec<Vec2>> = HashMap::new();

    let mut size = (0, 0);

    for (y, line) in input.lines().enumerate() {
        if y > size.1 as usize { size.1 = y as isize }
        for (x, c) in line.chars().enumerate() {
            let c = match c {
                '.' => None,
                o => Some(o)
            };

            if let Some(f) = c {
                frequencies.entry(f).or_default().push((x as isize, y as isize));
            }

            if x > size.0 as usize { size.0 = x as isize }
        }
    }

    let size = (size.0 + 1, size.1 + 1);

    let mut antinodes = HashSet::new();

    for freqs in frequencies.values() {
        let n = freqs.len();
        for i in 0..n - 1 {
            for j in i + 1..n {
                let nodes = get_antinodes(
                    freqs[i],
                    freqs[j],
                    size
                );

                antinodes.insert(freqs[i]);
                antinodes.insert(freqs[j]);
                antinodes.extend(nodes);
            }
        }
    } 

    let count = antinodes.len();

    println!("The amount of antinodes inside the map is {count}");
}
