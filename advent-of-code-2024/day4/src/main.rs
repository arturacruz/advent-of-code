use advent_of_code_lib::*;

const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];

fn main() {
    let input = get_input(InputType::Input);
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != TARGET[0] { continue }

            'dirs: for (i, j) in DIRS {
                for dist in 1..=3 {
                    let Some(newy) = y.checked_add_signed(j * dist) else {
                        continue 'dirs;
                    };

                    let Some(newx) = x.checked_add_signed(i * dist) else {
                        continue 'dirs;
                    };

                    let Some(t) = matrix.get(newy).and_then(|l| l.get(newx)) else {
                        continue 'dirs;
                    };

                    if *t != TARGET[dist as usize] {
                        continue 'dirs
                    }
                }
                count += 1;
            }
        }
    }

    println!("The number of XMAS in the input was {count}!");
}
