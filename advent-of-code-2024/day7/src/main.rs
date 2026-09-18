use advent_of_code_lib::*;

fn backtrack(target_value: &usize, nums: &[usize], current_val: usize) -> bool {
    if nums.is_empty() {
        return current_val == *target_value;
    }

    let new_slice = &nums[1..];

    // Addition
    let val = current_val + nums[0];
    if backtrack(target_value, new_slice, val) {
        return true;
    }

    let val = current_val * nums[0];
    if backtrack(target_value, new_slice, val) {
        return true;
    }

    let val = (current_val.to_string() + &nums[0].to_string()).parse::<usize>().unwrap();
    backtrack(target_value, new_slice, val)
}

fn main() {
    let input = get_input(InputType::Input);
    let equations = input.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(value, nums)| (
            value.parse::<usize>().unwrap(),
            nums.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        ));

    let count = equations
        .filter(|(val, nums)| backtrack(val, nums, 0))
        .map(|(val, _)| val)
        .sum::<usize>();

    println!("The sum of all valid calibration results is {count}");
}
