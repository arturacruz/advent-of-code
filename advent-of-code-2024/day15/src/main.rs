use advent_of_code_lib::*;

const MULT: usize = 100;

type Vec2 = (isize, isize);

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cell {
    Wall,
    Box,
    Robot,
    None
}

fn in_bounds((x, y): Vec2, (width, height): Vec2) -> bool {
    x > 0 && y > 0 && x < width - 1 && y < height - 1
}

fn attempt_box_movement((x, y): Vec2, (dx, dy): Vec2, map_size: Vec2, map: &mut [Vec<Cell>]) -> bool {
    let (nx, ny) = (x + dx, y + dy);
    if !in_bounds((nx, ny), map_size) {
        return false;
    }

    let cell = &map[ny as usize][nx as usize];
    if *cell == Cell::None {
        return true;
    } else if *cell == Cell::Wall {
        return false;
    }

    let res = attempt_box_movement((nx, ny), (dx, dy), map_size, map);
    if res {
        map[(ny + dy) as usize][(nx + dx) as usize] = map[ny as usize][nx as usize];
        map[ny as usize][nx as usize] = Cell::None;
    }
    res
}

#[allow(dead_code)]
fn render_map(map: &[Vec<Cell>]) {
    for row in map {
        for c in row {
            let c = match c {
                Cell::Box => 'O',
                Cell::Wall => '#',
                Cell::None => '.',
                Cell::Robot => '@'
            };
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    let input = get_input(InputType::Input);
    let mut lines = input.lines();

    let mut robot: Vec2 = (0, 0);
    let mut map = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| line.chars()
            .enumerate()
            .map(|(x, c)| match c {
                '.' => Cell::None,
                'O' => Cell::Box,
                '#' => Cell::Wall,
                '@' => {
                    robot = (x as isize, y as isize);
                    Cell::Robot
                },
                o => panic!("Invalid character {o}")
            }).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let size = (
        map[0].len() as isize,
        map.len() as isize
    );

    let movements = lines.collect::<String>()
        .chars()
        .map(|c| match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            o => panic!("Invalid character {o}")
        }).collect::<Vec<Vec2>>();

    for (dx, dy) in movements {
        let (x, y) = robot;

        if attempt_box_movement((x, y), (dx, dy), size, &mut map) {
            let (nx, ny) = (x + dx, y + dy);
            map[ny as usize][nx as usize] = map[y as usize][x as usize];
            map[y as usize][x as usize] = Cell::None;
            robot = (nx, ny);
        }
    }

    let mut gps = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Cell::Box {
                gps += y * MULT + x;
            }
        }
    }

    println!("The sum of all GPS score is {gps}");

}
