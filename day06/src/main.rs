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
}
