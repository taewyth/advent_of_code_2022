use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut contained_pairs = 0;
    let mut overlapping_pairs = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let ranges: Vec<&str> = line.split(&[',', '-'][..]).collect();

// first part
        if ((ranges[0].parse::<u8>().unwrap()..ranges[1].parse::<u8>().unwrap()+1).contains(&ranges[2].parse::<u8>().unwrap())
            && (ranges[0].parse::<u8>().unwrap()..ranges[1].parse::<u8>().unwrap()+1).contains(&ranges[3].parse::<u8>().unwrap()))
        || ((ranges[2].parse::<u8>().unwrap()..ranges[3].parse::<u8>().unwrap()+1).contains(&ranges[0].parse::<u8>().unwrap())
            && (ranges[2].parse::<u8>().unwrap()..ranges[3].parse::<u8>().unwrap()+1).contains(&ranges[1].parse::<u8>().unwrap())){
                contained_pairs += 1;
            }
        
// second part
            if (ranges[0].parse::<u8>().unwrap()..ranges[1].parse::<u8>().unwrap()+1).contains(&ranges[2].parse::<u8>().unwrap())
            || (ranges[0].parse::<u8>().unwrap()..ranges[1].parse::<u8>().unwrap()+1).contains(&ranges[3].parse::<u8>().unwrap())
            || (ranges[2].parse::<u8>().unwrap()..ranges[3].parse::<u8>().unwrap()+1).contains(&ranges[0].parse::<u8>().unwrap())
            || (ranges[2].parse::<u8>().unwrap()..ranges[3].parse::<u8>().unwrap()+1).contains(&ranges[1].parse::<u8>().unwrap()){
                overlapping_pairs += 1;
            }
    }

    println!("pairs containing their counterpart: {}", contained_pairs.to_string());
    println!("pairs overlapping with their counterpart: {}", overlapping_pairs.to_string());
}
