use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input").unwrap();

    let mut sum = 0;
    let mut questions = HashSet::new();
    let mut next_questions = HashSet::new();
    let mut first = true;

    for l in BufReader::new(input).lines() {
        let line = l.unwrap();

        if first {
            for c in line.chars() {
                questions.insert(c);
            }
            first = false;
            continue;
        }

        if line == "" {
            sum += questions.len();
            questions.clear();
            first = true;
            continue;
        }

        for c in line.chars() {
            if questions.contains(&c) {
                next_questions.insert(c);
            }
        }

        questions = next_questions.to_owned();
        next_questions.clear();
    }

    sum += questions.len();

    println!("{}", sum);
}
