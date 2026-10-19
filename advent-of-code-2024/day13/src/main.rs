use advent_of_code_lib::*;

const OFFSET: usize = 10000000000000;

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    target: (usize, usize),
}

fn gcd(a: usize, b: usize) -> usize {
    let mut n1 = a.max(b);
    let mut n2 = a.min(b);
    
    while n2 != 0 {
        let temp = n2;
        n2 = n1 % n2;
        n1 = temp;
    }
    n1
}

impl Machine {
    pub fn new(lines: (&str, &str, &str)) -> Self {
        let (a, b, target) = lines;

        let (x, y) = a.split_once(',').unwrap();
        let a = (
            x[x.find('+').unwrap() + 1..].parse().unwrap(),
            y[y.find('+').unwrap() + 1..].parse().unwrap(),
        );

        let (x, y) = b.split_once(',').unwrap();
        let b = (
            x[x.find('+').unwrap() + 1..].parse().unwrap(),
            y[y.find('+').unwrap() + 1..].parse().unwrap(),
        );

        let (x, y) = target.split_once(',').unwrap();
        let target = (
            x[x.find('=').unwrap() + 1..].parse::<usize>().unwrap() + OFFSET,
            y[y.find('=').unwrap() + 1..].parse::<usize>().unwrap() + OFFSET,
        );

        Self { 
            a,
            b,
            target,
        }
    }

    pub fn find_fewest_token_solution(&mut self) -> Option<usize> {
        let gcd_x = gcd(self.a.0, self.b.0);
        let gcd_y = gcd(self.a.1, self.b.1);

        if !self.target.0.is_multiple_of(gcd_x) || !self.target.1.is_multiple_of(gcd_y) {
            println!("{:?} has no solution", self);
            return None;
        }

        Some(0)
    }
}
fn main() {
    let input = get_input(InputType::Test);

    let tokens = input.lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|m| Machine::new((m[0], m[1], m[2])))
        .filter_map(|mut m| m.find_fewest_token_solution())
        .sum::<usize>();

    println!("The minimum amount of tokens to win all prizes is {tokens}");
}
