use std::fs::File;
use std::io::{BufRead, BufReader};

fn trees(map: &[String], x_diff: usize, y_diff: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    let x_size = map[0].len();
    let y_size = map.len();

    while y < y_size {
        if map[y].as_bytes()[x % x_size] as char == '#' {
            trees += 1;
        }

        x += x_diff;
        y += y_diff;
    }

    trees
}

fn main() {
    let input = File::open("input").unwrap();
    let map: Vec<String> = BufReader::new(input)
        .lines()
        .filter_map(|x| x.ok())
        .collect();

    let sum = trees(&map, 1, 1)
        * trees(&map, 3, 1)
        * trees(&map, 5, 1)
        * trees(&map, 7, 1)
        * trees(&map, 1, 2);

    println!("{}", sum);
}
