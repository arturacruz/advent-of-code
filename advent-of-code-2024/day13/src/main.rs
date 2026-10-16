use advent_of_code_lib::*;

struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    target: (usize, usize),
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
            x[x.find('=').unwrap() + 1..].parse().unwrap(),
            y[y.find('=').unwrap() + 1..].parse().unwrap(),
        );

        Self { 
            a,
            b,
            target,
        }
    }

    pub fn find_fewest_token_solution(&mut self) -> Option<usize> {
        let mut min_val: Option<usize> = None;
        for i in 0..=100 {
            for j in 0..=100 {
                let x = i * self.a.0 + j * self.b.0;
                let y = i * self.a.1 + j * self.b.1;

                let tokens = i * 3 + j;
                if (x, y) == self.target {
                    min_val = Some(match min_val {
                        Some(m) => m.min(tokens),
                        None => tokens
                    });
                }
            }
        }
        min_val
    }
}
fn main() {
    let input = get_input(InputType::Input);
    
    let tokens = input.lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|m| Machine::new((m[0], m[1], m[2])))
        .filter_map(|mut m| m.find_fewest_token_solution())
        .sum::<usize>();

    println!("The minimum amount of tokens to win all prizes is {tokens}");
}
