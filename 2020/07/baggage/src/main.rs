use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Entry {
    bag: String,
    amount: u32,
}

fn count_bags(bag: &str, bags: &HashMap<String, Vec<Entry>>) -> u32 {
    if let Some(inners) = bags.get(bag) {
        inners.iter().fold(0, |acc, inner| {
            acc + inner.amount * count_bags(&inner.bag, bags)
        }) + 1
    } else {
        1
    }
}

fn main() {
    let input = File::open("input").unwrap();

    let mut bags = HashMap::new();

    for l in BufReader::new(input).lines() {
        let l = l.unwrap();
        let line: Vec<&str> = l.split(' ').collect();

        let dest = line[0].to_owned() + " " + line[1];

        for i in 2..line.len() {
            if let Ok(amount) = line[i].parse::<u32>() {
                let src = line[i + 1].to_owned() + " " + line[i + 2];

                let containers: &mut Vec<Entry> = bags.entry(dest.to_owned()).or_default();

                containers.push(Entry { bag: src, amount });
            }
        }
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert("shiny gold");
    queue.push_back("shiny gold");

    /*while !queue.is_empty() {
        let bag = queue.pop_front().unwrap();

        if let Some(containers) = bags.get(bag) {
            for container in containers {
                if !visited.contains(container.as_str()) {
                    visited.insert(container);
                    queue.push_back(container);
                }
            }
        }
    }

    println!("{}", visited.len() - 1);*/

    let bags = count_bags(&"shiny gold".to_string(), &bags);
    println!("{}", bags - 1);
}
