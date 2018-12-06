extern crate regex;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    let mut grid = [[false; 1000]; 1000];

    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
    	.expect("something went wrong reading the file");

	let split = contents.lines();

	let regex = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

	for line in lines {
		let vals = regex.caputres_iter(line).next().unwrap();

		
	}

}
