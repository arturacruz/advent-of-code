use std::collections::{HashMap, HashSet};

use advent_of_code_lib::*;

struct Map {
    visited: HashSet<(usize, usize)>,
    matrix: Vec<Vec<char>>,
    areas: HashMap<char, (usize, usize)>
}

impl Map {
    pub fn new(input: String) -> Self {
        let matrix = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        Self { 
            areas: HashMap::new(),
            visited: HashSet::new(),
            matrix
        }
    }

    pub fn calculate_area_and_perimeter(&mut self, x: usize, y: usize, plant: char) {

    }

    pub fn run(&mut self) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                self.calculate_area_and_perimeter(x, y, self.matrix[y][x]);
            }
        }
    }
}

fn main() {
    let input = get_input(InputType::Test);

    let mut map = Map::new(input);
    map.run();
}
