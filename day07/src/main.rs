use std::collections::{HashMap, HashSet};

fn main() {
    println!("Day 7");

    let lines: Vec<&str> = include_str!("../input/input.txt").lines().collect();

    let mut beams: HashMap<u64, u64> = HashMap::new();
    beams.insert(lines.first().unwrap().chars().position(|c| c == 'S').unwrap() as u64, 1);

    let mut split_counter: u64 = 0;

    for line in lines[1..].iter() {
        let mut beams_to_add: HashMap<u64, u64> = HashMap::new();
        let mut beams_to_remove: HashSet<u64> = HashSet::new();

        for (beam_pos, beam_cnt) in beams.iter() {
            let beam = *beam_pos;
            if let Some(pos) = line.chars().nth(beam as usize)
                && pos == '^'
            {
                beams_to_add.entry(beam - 1).and_modify(|cnt| *cnt += *beam_cnt).or_insert(*beam_cnt);
                beams_to_add.entry(beam + 1).and_modify(|cnt| *cnt += *beam_cnt).or_insert(*beam_cnt);
                beams_to_remove.insert(beam);
                split_counter += 1;
            }
        }

        for beam in beams_to_add {
            beams.entry(beam.0).and_modify(|cnt| *cnt += beam.1).or_insert(beam.1);
        }
        for beam in beams_to_remove {
            beams.remove(&beam);
        }
    }

    println!("Part 1: {}", split_counter);
    println!("Part 2: {}", beams.values().sum::<u64>());
}
