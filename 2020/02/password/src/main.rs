use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input").unwrap();

    let valid = BufReader::new(input)
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let vals: Vec<&str> = l.split(' ').collect();

            let range: Vec<&str> = vals[0].split('-').collect();

            let small: usize = range[0].parse().unwrap();
            let large: usize = range[1].parse().unwrap();

            (
                small,
                large,
                vals[1].as_bytes()[0] as char,
                vals[2].to_owned(),
            )
        })
        .fold(0, |acc, vals| {
            let (small, large, c, password) = vals;

            /*let matches = password.matches(c).count();

            if matches >= small && matches <= large {
                acc + 1
            } else {
                acc
            }*/

            let mut chars = 0;

            if password.as_bytes()[small - 1] as char == c {
                chars += 1;
            }
            if password.as_bytes()[large - 1] as char == c {
                chars += 1;
            }

            if chars == 1 {
                acc + 1
            } else {
                acc
            }
        });

    println!("{}", valid);
}
