use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl From<&str> for Op {
    fn from(op: &str) -> Self {
        match op {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => panic!("Invalid op"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Inst {
    op: Op,
    val: i32,
}

impl From<String> for Inst {
    fn from(inst: String) -> Self {
        let inst: Vec<&str> = inst.split(' ').collect();
        Inst {
            op: inst[0].into(),
            val: inst[1].parse().unwrap(),
        }
    }
}

fn main() {
    let input = File::open("input").unwrap();

    let lines: Vec<Inst> = BufReader::new(input)
        .lines()
        .filter_map(|x| x.ok())
        .map(Inst::from)
        .collect();

    let mut index: i32 = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();

    loop {
        let inst = lines[index as usize];

        match inst.op {
            Op::Acc => {
                acc += inst.val;
                index += 1;
            }
            Op::Jmp => {
                index += inst.val;
            }
            Op::Nop => {
                index += 1;
            }
        };

        if visited.contains(&index) {
            break;
        } else {
            visited.insert(index);
        }
    }

    println!("{}", acc);
}
