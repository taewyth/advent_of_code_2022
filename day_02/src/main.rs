use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current_score: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<char> = line.chars().collect();
        /*
        // part_1
                current_score += match values[0] {
                    'A' => match values[2] {
                        'X' => 4,

                        'Y' => 8,

                        'Z' => 3,

                        _ => {
                            println!("erreur.");
                            0
                        }
                    },

                    'B' => match values[2] {
                        'X' => 1,

                        'Y' => 5,

                        'Z' => 9,

                        _ => {
                            println!("erreur.");
                            0
                        }
                    },

                    'C' => match values[2] {
                        'X' => 7,

                        'Y' => 2,

                        'Z' => 6,

                        _ => {
                            println!("erreur.");
                            0
                        }
                    },

                    _ => {
                        println!("erreur.");
                        0
                    }
                }
        */

        // part_2
        current_score += match values[0] {
            'A' => match values[2] {
                'X' => 3,

                'Y' => 4,

                'Z' => 8,

                _ => {
                    println!("erreur.");
                    0
                }
            },

            'B' => match values[2] {
                'X' => 1,

                'Y' => 5,

                'Z' => 9,

                _ => {
                    println!("erreur.");
                    0
                }
            },

            'C' => match values[2] {
                'X' => 2,

                'Y' => 6,

                'Z' => 7,

                _ => {
                    println!("erreur.");
                    0
                }
            },

            _ => {
                println!("erreur.");
                0
            }
        }
    }

    println!("Resultat: {}", current_score.to_string());
}
