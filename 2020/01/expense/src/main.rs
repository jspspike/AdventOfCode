use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input").unwrap();
    let nums: Vec<u32> = BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    let mut nums_set: HashSet<u32> = HashSet::new();
    for num in nums.iter() {
        nums_set.insert(*num);
    }

    /*for num in nums.iter() {
        let val = 2020 - num;
        if nums_set.contains(&val) {
            println!("{}", val * num);
            return;
        }
    }*/

    for i in 0..nums.len() {
        for j in i..nums.len() {
            let sum: u32 = 2020;
            let sub = nums[i] + nums[j];
            let val = sum.checked_sub(sub);

            if let Some(val) = val {
                if nums_set.contains(&val) {
                    println!("{}", val * nums[i] * nums[j]);
                    return;
                }
            }
        }
    }
}
