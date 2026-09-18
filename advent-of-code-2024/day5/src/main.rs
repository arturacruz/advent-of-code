use std::collections::{HashMap, HashSet};

use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);
    let mut lines = input.lines();

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let (x, y) = line.split_once('|').unwrap();

        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        
        rules.entry(y).or_default().insert(x);
    }

    let sum = lines
        .map(|line| line
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
        .filter(|update| {
            let mut forbidden_nums: HashSet<usize> = HashSet::new();
            for num in update {
                if let Some(rule) = rules.get(num) {
                    forbidden_nums.extend(rule);
                }

                if forbidden_nums.contains(num) {
                    return true;
                }
            };

            false
        })
        .map(|mut update| {
            let len = update.len();
            let mut unsorted = true;
            while unsorted {
                unsorted = false;
                for i in 0..len-1 {
                    for j in i+1..len {
                        let n1 = update[i];
                        let n2 = update[j];
                        if let Some(set) = rules.get(&n1) && set.contains(&n2) {
                            update.swap(i, j);
                            unsorted = true;
                        }
                    }
                }
            }
            update
        })
        .map(|u| u[u.len() / 2])
        .sum::<usize>();

    println!("The sum of the middle number of all valid updates is {sum}!");
}
