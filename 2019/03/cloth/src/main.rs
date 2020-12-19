extern crate regex;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    let mut grid = [[0; 1000]; 1000];

    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
    	.expect("something went wrong reading the file");

	let lines = contents.lines();

	let regex = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

	let mut overlap = 0;


	for line in lines {
		let vals = regex.captures_iter(line).next().unwrap();
		let start:(usize, usize) = (vals[2].parse().unwrap(), vals[3].parse().unwrap());
		let size:(usize, usize) = (vals[4].parse().unwrap(), vals[5].parse().unwrap());
		for i in start.0 .. start.0 + size.0 {
			for j in start.1 .. start.1 + size.1 {
				grid[i][j] = if grid[i][j] == 0 {
					1
				} else if grid[i][j] == 1 {
					overlap += 1;
					2
				} else {
					2
				}
			}
		}		
	}

	println!("{}", overlap);

}
