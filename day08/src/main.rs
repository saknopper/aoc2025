#[derive(Clone, Copy, Debug)]
struct Box {
    x: u32,
    y: u32,
    z: u32,
    connected: bool,
}
fn main() {
    println!("Day 8");

    let mut boxes: Vec<Box> = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            let dims: Vec<u32> = line.split(',').map(|num| num.parse::<u32>().unwrap()).collect();
            Box { x: dims[0], y: dims[1], z: dims[2], connected: false }
        })
        .collect();

    let mut box_pairs: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let dist = distance(&boxes[i], &boxes[j]);
            box_pairs.push((i, j, dist));
        }
    }

    box_pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<Vec<usize>> = Vec::new();

    //for step in 0..1000 {
    for (i, j, _) in box_pairs {
        //let (i, j, _) = box_pairs[step];
        let circuit_i = find_circuit(&circuits, i);
        let circuit_j = find_circuit(&circuits, j);

        match (circuit_i, circuit_j) {
            // Both in circuits - merge if different
            (Some(idx_i), Some(idx_j)) if idx_i != idx_j => {
                merge_circuits(&mut circuits, idx_i, idx_j);
            }
            // Neither connected - create new circuit
            (None, None) => {
                boxes[i].connected = true;
                boxes[j].connected = true;
                circuits.push(vec![i, j]);
            }
            // One connected - add the other
            (Some(idx), None) => {
                circuits[idx].push(j);
                boxes[j].connected = true;
            }
            (None, Some(idx)) => {
                circuits[idx].push(i);
                boxes[i].connected = true;
            }
            _ => {} // Both in same circuit - nothing to do
        }

        if circuits.len() == 1 && boxes.iter().all(|b| b.connected) {
            println!("Part 2: {}", boxes[i].x as u64 * boxes[j].x as u64);
            break;
        }
    }

    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    circuits.reverse();

    //println!("Part 1: {}", circuits[0..3].iter().map(|c| c.len()).product::<usize>());
}

fn find_circuit(circuits: &[Vec<usize>], box_idx: usize) -> Option<usize> {
    circuits.iter().position(|c| c.contains(&box_idx))
}

fn merge_circuits(circuits: &mut Vec<Vec<usize>>, idx_i: usize, idx_j: usize) {
    let (remove_idx, target_idx) = if idx_i < idx_j { (idx_j, idx_i) } else { (idx_i, idx_j) };
    let removed = circuits.remove(remove_idx);
    circuits[target_idx].extend(removed);
}

fn distance(a: &Box, b: &Box) -> f64 {
    let dx = a.x as f64 - b.x as f64;
    let dy = a.y as f64 - b.y as f64;
    let dz = a.z as f64 - b.z as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
