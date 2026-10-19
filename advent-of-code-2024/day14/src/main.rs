use std::ops::{Add, AddAssign, Mul};

use advent_of_code_lib::*;

const INPUT_TYPE: InputType = InputType::Input;

const SIZE: (usize, usize) = match INPUT_TYPE {
    InputType::Test => (11, 7),
    InputType::Input => (101, 103)
};

const SECONDS: usize = 80000;

struct Position {
    x: usize,
    y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl AddAssign<&mut Velocity> for Position {
    fn add_assign(&mut self, rhs: &mut Velocity) {
        let (width, height) = (SIZE.0 as isize, SIZE.1 as isize);
        let x = self.x as isize + rhs.x;
        let y = self.y as isize + rhs.y;

        let min_x = 0;
        let max_x = width - 1;

        let min_y = 0;
        let max_y = height - 1;
        
        let range_x = max_x - min_x + 1;
        let x = (min_x + (x - min_x).rem_euclid(range_x)) as usize;

        let range_y = max_y - min_y + 1;
        let y = (min_y + (y - min_y).rem_euclid(range_y)) as usize;

        *self = Position::new(x, y);
    }
}

struct Velocity {
    x: isize,
    y: isize
}
 
impl Velocity {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Mul<usize> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.x * rhs as isize, self.y * rhs as isize)
    }

}


fn main() {

    let input = get_input(INPUT_TYPE);
    let mut map = [[0_usize; SIZE.0]; SIZE.1];

    let mut robots = input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| {
            let (_, pos) = a.split_once('=').unwrap();
            let (_, vel) = b.split_once('=').unwrap();

            let (xpos, ypos) = pos.split_once(',').unwrap();
            let (xpos, ypos) = (xpos.parse().unwrap(), ypos.parse().unwrap());

            let (xvel, yvel) = vel.split_once(',').unwrap();
            let (xvel, yvel) = (xvel.parse().unwrap(), yvel.parse().unwrap());

            (Position::new(xpos, ypos), Velocity::new(xvel, yvel))
        }).collect::<Vec<_>>();

    for (pos, _vel) in &robots {
        map[pos.y][pos.x] += 1;
        
    }

    for i in 0..SECONDS {
        for (pos, vel) in robots.iter_mut() {
            map[pos.y][pos.x] -= 1;
            *pos += vel;
            map[pos.y][pos.x] += 1;
        }

        let mut found = false;
        'outer: for (pos, _vel) in robots.iter() {
            for i in 1..5 {
                if pos.y + i >= SIZE.1 || map[pos.y + i][pos.x] == 0 {
                    continue 'outer;
                }
            }
            found = true; 
            break;
        }

        if !found {
            continue;
        }
    
        println!("Render {i}");
        map.iter().for_each(|row| {
            row.iter().for_each(|c| 
                if *c > 0 {
                    print!("O")
                } else {
                    print!(".")
                });
            println!();
        });
    }
}
