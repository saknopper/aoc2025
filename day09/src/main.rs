#[derive(Debug)]
struct Tile {
    x: u32,
    y: u32,
}

fn main() {
    println!("Day 9");

    let red_tiles: Vec<Tile> = include_str!("../input/input.txt")
        .lines()
        .map(|l| {
            let coords: Vec<u32> = l.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            Tile { x: coords[0], y: coords[1] }
        })
        .collect();

    let red_tile_areas: Vec<(usize, usize, usize)> = (0..red_tiles.len())
        .flat_map(|i| ((i + 1)..red_tiles.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let dist = area(&red_tiles[i], &red_tiles[j]);
            (i, j, dist)
        })
        .collect();

    println!("Part 1: {}", red_tile_areas.iter().map(|(_, _, a)| a).max().unwrap());
}

fn area(a: &Tile, b: &Tile) -> usize {
    let length = if a.x > b.x { a.x - b.x } else { b.x - a.x } + 1;
    let width = if a.y > b.y { a.y - b.y } else { b.y - a.y } + 1;
    (length as usize * width as usize) as usize
}
