use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut calories: Vec<usize> = Vec::new();
    let mut current_calories: usize = 0;

    for line in reader.lines(){
        let line = line.unwrap();
        if line == ""{
            calories.push(current_calories);
            current_calories = 0;
        }
        else {
            current_calories += line.parse::<usize>().unwrap();
        }
    }

    calories.sort();
    current_calories = calories.pop().unwrap();
    println!("calories max: {}", current_calories.to_string());

    current_calories += calories.pop().unwrap() + calories.pop().unwrap();
    println!("top 3: {}", current_calories.to_string());
}
