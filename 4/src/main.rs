use std::env;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn input_v1() {
    let filepath: String = env::args().nth(1).unwrap();
    if let Ok(file) = File::open(&filepath) {

        let buf_reader: BufReader<File> = BufReader::new(file);

        let mut score: u32 = 0;

        for line in buf_reader.lines(){
            if let Ok(line) = line {
                let splitted: Vec<&str> = line.split(",").collect();

                if splitted.len() != 2 {
                    eprintln!("Vec didn't split into two parts {:?}", splitted);
                    exit(1);
                }

                let left_split: Vec<&str> = splitted[0].split("-").collect();
                if left_split.len() != 2 {
                    eprintln!("Left split didn't split into two parts {:?}", splitted);
                    exit(1);
                }

                let right_split: Vec<&str> = splitted[1].split("-").collect();
                if right_split.len() != 2 {
                    eprintln!("Right split didn't split into two parts {:?}", splitted);
                    exit(1);
                }

                let left_bot: u32 = left_split[0].parse::<u32>().unwrap();
                let left_top: u32 = left_split[1].parse::<u32>().unwrap();
                let right_bot: u32 = right_split[0].parse::<u32>().unwrap();
                let right_top: u32 = right_split[1].parse::<u32>().unwrap();

                if ((left_bot <= right_bot) && (left_top >= right_top)) || ((right_bot <= left_bot) && (right_top >= left_top)) { score += 1; }

            } else {
                eprintln!("Error: \"Error while reading lines\"");
                exit(1);
            }
        }

        println!("Score: {}", score);

    } else {
        eprintln!("Couldn't open filepath {}", filepath);
        exit(1);
    }
}

fn input_v2() {
    let filepath: String = env::args().nth(1).unwrap();
    if let Ok(file) = File::open(&filepath) {

        let buf_reader: BufReader<File> = BufReader::new(file);

        let mut score: u32 = 0;

        for line in buf_reader.lines() {
            if let Ok(line) = line {
                let splitted: Vec<&str> = line.split(",").collect();

                if splitted.len() != 2 {
                    eprintln!("Vec didn't split into two parts {:?}", splitted);
                    exit(1);
                }

                let left_split: Vec<&str> = splitted[0].split("-").collect();
                if left_split.len() != 2 {
                    eprintln!("Left split didn't split into two parts {:?}", splitted);
                    exit(1);
                }

                let right_split: Vec<&str> = splitted[1].split("-").collect();
                if right_split.len() != 2 {
                    eprintln!("Right split didn't split into two parts {:?}", splitted);
                    exit(1);
                }

                let left_bot: u32 = left_split[0].parse::<u32>().unwrap();
                let left_top: u32 = left_split[1].parse::<u32>().unwrap();
                let right_bot: u32 = right_split[0].parse::<u32>().unwrap();
                let right_top: u32 = right_split[1].parse::<u32>().unwrap();

                if !((left_bot..left_top + 1).collect::<HashSet<u32>>()).is_disjoint(&(right_bot..right_top + 1).collect::<HashSet<u32>>()) { score += 1; }
            } else {
                eprintln!("Error: \"Error while reading lines\"");
                exit(1);
            }
        }

        println!("Score: {}", score);
    } else {
        eprintln!("Couldn't open filepath {}", filepath);
        exit(1);
    }
}

fn help() {
    eprintln!("Usage: {} input_file_path", env::args().nth(0).unwrap());
    exit(1);
}

fn main() {
    if env::args().len() != 2 { help(); }
    input_v1();
    input_v2();
}
