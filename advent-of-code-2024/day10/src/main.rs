use std::collections::HashSet;

use advent_of_code_lib::*;

const RADIX: u32 = 10;

fn is_in_bounds(x: usize, y: usize, matrix: &[Vec<u8>]) -> bool {
    x < matrix[0].len() && y < matrix.len()
}

fn dfs(x: usize, y: usize, num: u8, matrix: &Vec<Vec<u8>>) -> usize {
    if !is_in_bounds(x, y, matrix) || matrix[y][x] != num + 1 {
        return 0;
    }

    if matrix[y][x] == 9 {
        return 1;
    }

    let num = matrix[y][x];

    dfs(x.wrapping_sub(1), y, num, matrix) +
        dfs(x.wrapping_add(1), y, num, matrix) +
        dfs(x, y.wrapping_add(1), num, matrix) +
        dfs(x, y.wrapping_sub(1), num, matrix)
}

fn main() {
    let input = get_input(InputType::Input);

    let map = input.lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap() as u8)
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let mut trailheads = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != 0 {
                continue
            }

            let val = dfs(x.wrapping_sub(1), y, 0, &map) +
                dfs(x.wrapping_add(1), y, 0, &map) +
                dfs(x, y.wrapping_add(1), 0, &map) +
                dfs(x, y.wrapping_sub(1), 0, &map);
            trailheads.push(val);
        }
    }

    let sum = trailheads
        .iter()
        .sum::<usize>();

    println!("The sum of all trailheads is {sum}");
}
