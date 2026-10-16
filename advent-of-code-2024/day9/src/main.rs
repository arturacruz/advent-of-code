use std::collections::HashSet;

use advent_of_code_lib::*;

const RADIX: u32 = 10;

#[allow(dead_code)]
fn debug_disk(disk: &[Option<usize>]) {
    disk.iter()
        .for_each(|c| match c {
            Some(o) => print!("{o}"),
            None => print!(".")
        });
    println!();
}

fn main() {
    let input = get_input(InputType::Test);
    let input = input.trim();
    
    let mut disk = vec![];
    let mut file_sizes = vec![];

    // Convert input into disk visualization
    let mut file_id = 0;
    for (i, c) in input.chars().enumerate() {
        let c = c.to_digit(RADIX).unwrap() as usize;
        if i % 2 == 0 {
            for _ in 0..c {
                disk.push(Some(file_id));
            }
            file_sizes.push((disk.len() - c + 1, c));
            file_id += 1;
        } else {
            for _ in 0..c {
                disk.push(None);
            }
        }
    }

    // Move disk partitions
    let mut i = 0;

    while i < disk.len() {

    }

    // Calculate checksum
    let checksum = disk.iter()
        .enumerate()
        .filter(|(_i, c)| c.is_some())
        .map(|(i, c)| i * c.unwrap())
        .sum::<usize>();

    println!("The checksum of this disk is {checksum}")
}
