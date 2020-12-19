use std::fs::File;
use std::io::{BufRead, BufReader};

fn id(ticket: String) -> u32 {
    let mut small = 0;
    let mut large = 127;

    for letter in ticket.chars().take(7) {
        match letter {
            'F' => large -= (large + 1 - small) / 2,
            'B' => small += (large + 1 - small) / 2,
            _ => panic!("{} is not valid", letter),
        }
    }

    let row = small;

    small = 0;
    large = 7;

    for letter in ticket.chars().skip(7) {
        match letter {
            'L' => large -= (large + 1 - small) / 2,
            'R' => small += (large + 1 - small) / 2,
            _ => panic!("{} is not valid", letter),
        }
    }

    row * 8 + small
}

fn main() {
    let input = File::open("input").unwrap();

    /*let biggest = BufReader::new(input).lines().fold(0, |acc, line| {
        let ticket = line.unwrap();

        let ticket_id = id(ticket);

        //dbg!(row * 8 + small);

        if ticket_id > acc {
            ticket_id
        } else {
            acc
        }
    });

    println!("{}", biggest);*/

    let mut seats: [bool; 971] = [false; 971];

    for line in BufReader::new(input).lines() {
        let ticket = line.unwrap();

        seats[id(ticket) as usize] = true;
    }

    for seat in seats.iter().enumerate() {
        let (val, seat) = seat;
        if !seat {
            println!("{}", val);
            //return;
        }
    }
}
