#![feature(linked_list_cursors)]
use std::collections::LinkedList;

use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);

    let mut stones = LinkedList::new();
    input
        .split_whitespace()
        .map(|i|
            i.parse::<usize>().unwrap()
        )
        .for_each(|i| stones.push_back(i));

    for i in 0..25 {
        let mut cursor = stones.cursor_front_mut();

        while let Some(val) = cursor.current() {
            if *val == 0 {
                *val = 1;
                cursor.move_next();
                continue
            }

            let val_str = val.to_string();
            if val_str.len() % 2 == 0 {
                let (a, b) = val_str.split_at(val_str.len() / 2);
                *val = a.parse().unwrap();
                cursor.insert_after(b.parse().unwrap());
                cursor.move_next();
                cursor.move_next();
                continue;
            }

            *val *= 2024;
            cursor.move_next();
        }

        println!("{i}");
    }

    println!("The amount of stones after 25 blinks is {}", stones.len());
}
