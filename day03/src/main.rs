fn main() {
    println!("Day 3");

    let input = include_str!("../input/input.txt");

    println!("Part 1: {}", solve(input, 2));
    println!("Part 1: {}", solve(input, 12));
}

fn solve(input: &str, digits: usize) -> u64 {
    return input.lines().map(|b| max_joltage_from_bank(b, digits)).sum();
}

fn max_joltage_from_bank(bank: &str, digits: usize) -> u64 {
    let mut batteries: Vec<u32> = Vec::new();
    let bank_length: usize = bank.len();

    for (i, b_char) in bank.chars().enumerate() {
        let b_digit = b_char.to_digit(10).unwrap();

        while !batteries.is_empty()
            && *batteries.last().unwrap() < b_digit
            && batteries.len() + bank_length - 1 - i >= digits
        {
            batteries.pop();
        }

        batteries.push(b_digit);
    }

    batteries.truncate(digits);
    let joltage_str = batteries.iter().fold("".to_string(), |wip, j| wip + &j.to_string());

    return joltage_str.parse::<u64>().unwrap();
}
