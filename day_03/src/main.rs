use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut item_priorities = HashMap::new();

    {
        // building the values hashmap
        let mut temp_chars = b'a'; // +1 a 24 min; -7 a -32 maj A L'ENVERS

        for priority in 1..53 {
            item_priorities.insert(temp_chars as char, priority);

            temp_chars += 1;
            if !((temp_chars as char).is_alphabetic()) {
                temp_chars = b'A';
            }
        }
    }

    let mut sum = 0;

    let mut i_group = 0; // indexing line to make groups
    let mut working_group: [String; 3] = Default::default();
    let mut group_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        working_group[i_group] = line.clone();
        i_group += 1;

        if i_group == 3 {
            for item in working_group[0].chars() {
                if working_group[1].contains(item) && working_group[2].contains(item) {
                    group_sum += item_priorities.get(&item).unwrap();
                    i_group = 0;
                    working_group = Default::default();
                    break;
                }
            }
        }

        let (first, second) = line.split_at(line.len() / 2);
        let mut item_already_checked = String::from("");
        for item in first.chars() {
            if second.contains(item) && !item_already_checked.contains(item) {
                item_already_checked.push(item);
                sum += item_priorities.get(&item).unwrap();
            }
        }
    }

    println!("priority of every item: {}", sum.to_string());
    println!("priority of badges: {}", group_sum.to_string());
}
