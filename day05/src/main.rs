use core::cmp::Ordering;
use std::ops::RangeInclusive;

fn main() {
    println!("Day 5");

    let input: Vec<&str> = include_str!("../input/input.txt").split("\n\n").collect();
    let (fresh_ranges, ingredients) = (input[0], input[1]);

    let mut expanded_fresh_ranges: Vec<RangeInclusive<_>> = fresh_ranges
        .lines()
        .map(|r| {
            let splitted: Vec<&str> = r.split("-").collect();
            let (start, end) = (splitted[0].parse::<u64>().unwrap(), splitted[1].parse::<u64>().unwrap());

            start..=end
        })
        .collect();

    let part_a = ingredients
        .lines()
        .map(|i| i.parse::<u64>().unwrap())
        .filter(|i| expanded_fresh_ranges.iter().any(|range| range.contains(i)))
        .count();

    expanded_fresh_ranges.sort_by(|a, b| match a.start().cmp(b.start()) {
        Ordering::Equal => a.end().cmp(b.end()),
        o => o,
    });
    let mut merged_fresh_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    for range in expanded_fresh_ranges {
        if let Some(last) = merged_fresh_ranges.last_mut()
            && range.start() <= last.end()
        {
            let updated = RangeInclusive::new(*(last).start(), *(last).end().max(range.end()));
            *last = updated;
            continue;
        }
        merged_fresh_ranges.push(range);
    }

    let part_b: u64 = merged_fresh_ranges.iter().map(|r| r.end() - r.start() + 1).sum();

    println!("Part 1: {}", part_a);
    println!("Part 2: {}", part_b);
}
