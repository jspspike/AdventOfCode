use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {

	let mut f = File::open("input.txt").expect("file not found");

	let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut split = contents.split("\n").collect::<Vec<&str>>();
    let len = split.len();
    split.truncate(len - 1);
    
    let mut sum : i32 = 0;

    let mut nums = HashSet::new();
    nums.insert(sum);

    let mut index = 0;

    loop {
    	let value : i32 = split[index % 1036].parse().unwrap();
    	sum += value;
    	if !nums.insert(sum) {
    		println!("{}", sum);	
    		break;
    	}
    	index += 1;
    }
}
