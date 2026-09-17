use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != 'A' || x == 0 || y == 0 || y == matrix.len() - 1 || x == line.len() - 1 {
                continue;
            }

            let top_left = matrix[y - 1][x - 1];
            let top_right = matrix[y - 1][x + 1];
            let bot_left = matrix[y + 1][x - 1];
            let bot_right = matrix[y + 1][x + 1];

            if matches!((top_left, bot_right), ('M', 'S') | ('S', 'M'))
                && matches!((top_right, bot_left), ('M', 'S') | ('S', 'M'))
            {
                count += 1;
            }
        }
    }

    println!("The number of XMAS in the input was {count}!");
}
