use std::collections::HashSet;

use advent_of_code_lib::*;

#[derive(Debug, PartialEq)]
enum Cell {
    Obstacle,
    Clear,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn turn(&mut self) {
        use Direction as D;
        *self = match self {
            D::Up => D::Right,
            D::Right => D::Down,
            D::Down => D::Left,
            D::Left => D::Up
        }
    }

    fn get(&self) -> (isize, isize) {
        use Direction as D;
        match self {
            D::Up => (0, -1),
            D::Down => (0, 1),
            D::Right => (1, 0),
            D::Left => (-1, 0)
        }
    }
}

#[derive(Clone)]
struct Guard {
    pos: (isize, isize),
    dir: Direction
}

fn is_out_of_bounds(x: isize, y: isize, matrix: &[Vec<Cell>]) -> bool {
    x < 0 || y < 0 || x >= matrix[0].len() as isize || y >= matrix.len() as isize
}

fn is_guard_in_loop(mut guard: Guard, matrix: &[Vec<Cell>]) -> bool {
    let mut visited = HashSet::new();

    while !is_out_of_bounds(guard.pos.0, guard.pos.1, matrix){
        if !visited.insert((guard.pos, guard.dir.clone())) {
            return true;
        }

        let (i, j) = guard.dir.get();
        let (x, y) = guard.pos;

        let (newx, newy) = (x + i, y + j);

        if let Some(cell) = matrix.get(newy as usize).and_then(|l| l.get(newx as usize)) {
            if *cell == Cell::Obstacle {
                guard.dir.turn();
            } else {
                guard.pos = (newx, newy);
            }
        } else {
            return false;
        }
    }

    unreachable!();
}
fn main() {
    let input = get_input(InputType::Input);
    let mut guard = Guard { pos: (0, 0), dir: Direction::Up };

    let mut matrix = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '#' => Cell::Obstacle,
                '.' => Cell::Clear,
                o => {
                    let dir = match o {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '>' => Direction::Right,
                        '<' => Direction::Left,
                        _ => panic!("Unexpected character found."),
                    };

                    guard.dir = dir;
                    guard.pos = (x as isize, y as isize);
                    Cell::Clear
                }
            };
            row.push(cell);
        }
        matrix.push(row);
    }

    let mut loop_counts = 0;
    let mut tested_obstacles = HashSet::new();

    while !is_out_of_bounds(guard.pos.0, guard.pos.1, &matrix){
        let (i, j) = guard.dir.get();
        let (x, y) = guard.pos;

        let (newx, newy) = (x + i, y + j);

        if let Some(cell) = matrix.get(newy as usize).and_then(|l| l.get(newx as usize)) {
            if *cell == Cell::Obstacle {
                guard.dir.turn();
            } else {
                // Test loop
                matrix[newy as usize][newx as usize] = Cell::Obstacle;
                if tested_obstacles.insert((newx, newy)) && is_guard_in_loop(guard.clone(), &matrix) {
                    loop_counts += 1;
                }
                matrix[newy as usize][newx as usize] = Cell::Clear;
                guard.pos = (newx, newy);
            }
        } else {
            break;
        }
    }

    println!("The amount of obstacles that could be placed and place the guard in a loop are {loop_counts}");
}
