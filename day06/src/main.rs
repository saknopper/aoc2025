fn main() {
    println!("Day 6");

    let input = include_str!("../input/input.txt");

    let lines: Vec<&str> = input.lines().collect();

    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<char>().unwrap())
        .collect();

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    for line in &lines[0..lines.len() - 1] {
        let line_numbers: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        numbers.push(line_numbers);
    }

    let mut part_a: u64 = 0;
    for (i, operator) in operators.iter().enumerate() {
        match operator {
            '+' => {
                let sum: u64 = numbers.iter().map(|row| row[i]).sum();
                part_a += sum;
            }
            '*' => {
                let product: u64 = numbers.iter().map(|row| row[i]).product();
                part_a += product;
            }
            _ => panic!("Unknown operator"),
        }
    }

    println!("Part A: {}", part_a);

    let line_length = lines.first().unwrap().len();
    let mut part_b: u64 = 0;
    let mut values_for_problem: Vec<u64> = Vec::new();

    for i in (0..line_length).rev() {
        let mut current_value: String = String::new();

        for line in lines[0..lines.len() - 1].iter() {
            let ch = line.chars().nth(i).unwrap();
            if ch.is_ascii_digit() {
                current_value.push(ch);
            }
        }
        if !current_value.is_empty() {
            let value: u64 = current_value.parse().unwrap();
            values_for_problem.push(value);
        }

        let operator = lines.last().unwrap().chars().nth(i).unwrap();
        match operator {
            '+' => {
                let sum: u64 = values_for_problem.iter().sum();
                part_b += sum;
                values_for_problem.clear();
            }
            '*' => {
                let product: u64 = values_for_problem.iter().product();
                part_b += product;
                values_for_problem.clear();
            }
            _ => {}
        }
    }

    println!("Part B: {}", part_b);
}
