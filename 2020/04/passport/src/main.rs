use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input").unwrap();
    let lines = BufReader::new(input).lines().filter_map(|x| x.ok());

    let mut fields = HashSet::new();
    let mut valid = 0;

    for line in lines {
        if line == "" {
            if fields.len() == 7 {
                valid += 1;
            }

            //dbg!(&fields);
            fields.clear();
            continue;
        }

        let entries: Vec<&str> = line.split(" ").collect();
        for entry in entries {
            let data: Vec<&str> = entry.split(":").collect();
            let tag = data[0];
            let value = data[1];

            match tag {
                "byr" => {
                    let date: u16 = value.parse().unwrap();
                    if date >= 1920 && date <= 2002 {
                        fields.insert(tag.to_owned());
                    }
                }
                "iyr" => {
                    let date: u16 = value.parse().unwrap();
                    if date >= 2010 && date <= 2020 {
                        fields.insert(tag.to_owned());
                    }
                }
                "eyr" => {
                    let date: u16 = value.parse().unwrap();
                    if date >= 2020 && date <= 2030 {
                        fields.insert(tag.to_owned());
                    }
                }
                "hgt" => {
                    let unit = &value[value.len() - 2..];
                    let height: Result<u16, _> = value[..value.len() - 2].parse();

                    match unit {
                        "in" => {
                            if let Ok(h) = height {
                                if h >= 59 && h <= 76 {
                                    fields.insert(tag.to_owned());
                                }
                            }
                        }
                        "cm" => {
                            if let Ok(h) = height {
                                if h >= 150 && h <= 193 {
                                    fields.insert(tag.to_owned());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                "hcl" => {
                    if value.starts_with('#')
                        && value.len() == 7
                        && value[1..].chars().all(|x| x.is_alphanumeric())
                    {
                        fields.insert(tag.to_owned());
                    }
                }
                "ecl" => match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                        fields.insert(tag.to_owned());
                    }
                    _ => {}
                },
                "pid" => {
                    if value.len() == 9 && value.parse::<u32>().is_ok() {
                        fields.insert(tag.to_owned());
                    }
                }
                _ => {}
            }

            /*match tag {
                "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => {fields.insert(tag.to_owned());},
                _ => {}
            };*/
        }
    }

    if fields.len() == 7 {
        valid += 1;
    }

    println!("{}", valid);
}
