const POSITIONS_TO_CHECK: &'static [(isize, isize)] =
    &[(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

fn main() {
    println!("Day 4");

    let input = include_str!("../input/input.txt");
    let mut grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|r| match r {
                    '@' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    //println!("Part 1: {}", process_accessible_rolls(&mut grid, 1));
    println!("Part 2: {}", process_accessible_rolls(&mut grid, 999));
}

fn process_accessible_rolls(grid: &mut Vec<Vec<bool>>, iterations: usize) -> u64 {
    let mut rolls_processed_total: u64 = 0;
    for _ in 0..iterations {
        let mut rolls_processed: u64 = 0;
        let mut locations_processed: Vec<(usize, usize)> = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, is_roll) in row.iter().enumerate() {
                if *is_roll {
                    if count_adjacent_rolls(y, x, grid) < 4 {
                        rolls_processed += 1;
                        locations_processed.push((y, x));
                    }
                }
            }
        }

        for (y, x) in locations_processed {
            grid[y][x] = false;
        }

        if rolls_processed == 0 {
            break;
        }

        rolls_processed_total += rolls_processed;
    }

    return rolls_processed_total;
}

fn count_adjacent_rolls(y: usize, x: usize, grid: &Vec<Vec<bool>>) -> u64 {
    let mut counter: u64 = 0;
    for pos in POSITIONS_TO_CHECK {
        match grid.get((y.cast_signed() + pos.0).cast_unsigned()) {
            Some(row) => match row.get((x.cast_signed() + pos.1).cast_unsigned()) {
                Some(position) => {
                    if *position {
                        counter += 1
                    }
                }
                None => continue,
            },
            None => continue,
        }
    }

    return counter;
}
