use std::collections::HashSet;

fn main() {
    println!("Day 7");

    let lines : Vec<&str> = include_str!("../input/input.txt").lines().collect();

    let mut beam_locations: HashSet<u64> = HashSet::new();
    beam_locations.insert(lines.first().unwrap().chars().position(|c| c == 'S').unwrap() as u64);

    let mut split_counter : u64 = 0;

    for line in lines[1..].iter() {
        let mut beams_to_add: HashSet<u64> = HashSet::new();
        let mut beams_to_remove: HashSet<u64> = HashSet::new();

        for beam_ref in beam_locations.iter() {
            let beam = *beam_ref;
            if let Some(pos) = line.chars().nth(beam as usize) {
                match pos {
                    '^' => {
                        beams_to_add.insert(beam - 1);
                        beams_to_add.insert(beam + 1);
                        beams_to_remove.insert(beam);
                        split_counter += 1;
                    }
                    _ => {}
                }
            }
        }
        for beam in beams_to_add {
            beam_locations.insert(beam);
        }
        for beam in beams_to_remove {
            beam_locations.remove(&beam);
        }
    }

    println!("Part 1: {}", split_counter);
}
