use advent_of_code_lib::*;

#[derive(Debug, PartialEq)]
enum Cell {
    Obstacle,
    Clear,
    Visited
}

#[derive(Debug)]
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

struct Guard {
    pos: (isize, isize),
    dir: Direction
}

fn is_out_of_bounds(x: isize, y: isize, matrix: &[Vec<Cell>]) -> bool {
    x < 0 || y < 0 || x >= matrix[0].len() as isize || y >= matrix.len() as isize
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

    while !is_out_of_bounds(guard.pos.0, guard.pos.1, &matrix){
        let (i, j) = guard.dir.get();
        let (x, y) = guard.pos;

        matrix[y as usize][x as usize] = Cell::Visited;

        let (newx, newy) = (x + i, y + j);

        if let Some(cell) = matrix.get(newy as usize).and_then(|l| l.get(newx as usize)) {
            if *cell == Cell::Obstacle {
                guard.dir.turn();
                let (i, j) = guard.dir.get();
                guard.pos = (x + i, y + j);
            } else {
                guard.pos = (newx, newy);
            }
        } else {
            break;
        }
    }

    let unique_visited_count = matrix.iter()
        .flatten()
        .filter(|c| **c == Cell::Visited)
        .count();
    
    println!("The guard passed through {unique_visited_count} cells.");
}
