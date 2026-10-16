use std::collections::HashSet;
use advent_of_code_lib::*;

struct Map {
    matrix: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>
}

impl Map {
    pub fn new() -> Self {
        let input = get_input(InputType::Input);
        let matrix = input.lines()
            .map(|line| 
                line.chars().collect::<Vec<_>>()
            ).collect::<Vec<_>>();

        Self { 
            matrix,
            visited: HashSet::new()
        }
    }

    fn dfs(&mut self, x: usize, y: usize, plant: char) -> (usize, usize) {
        let (mut area, mut perim) = (1, 0);
        self.visited.insert((x, y));

        let positions = [
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y)
        ];

        for (xn, yn) in positions {
            if let Some(letter) = self.matrix.get(yn).and_then(|row| row.get(xn)) && *letter == plant {
                if !self.visited.contains(&(xn, yn)) {
                    let res = self.dfs(xn, yn, plant);
                    area += res.0;
                    perim += res.1;
                }
            } else {
                perim += 1;
            }
        }

        (area, perim)
    } 

    pub fn calculate(&mut self) -> usize {
        let mut res = 0;
        for y in 0..self.matrix.len() { 
            for x in 0..self.matrix[y].len() {
                if self.visited.contains(&(x, y)) {
                    continue;
                }
                let plant = self.matrix[y][x];
                let (area, perim) = self.dfs(x, y, plant);
                res += area * perim;
                println!("{plant} = {:?}", (area, perim));
            }
        }

        res
    }
}
fn main() {
    let mut map = Map::new();
    let res = map.calculate();
    println!("The total pricing of all plant areas is {res}");
}
