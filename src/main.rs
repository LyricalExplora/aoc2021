mod submarine;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::submarine::Submarine;

const DEPTH_DATA_FILE: &str = "./sea_depth_data.txt";
const SUBMARINE_MOVEMENT_DATA_FILE: &str = "./submarine_movement_data.txt";
const SUB_BINARY_DIAGNOSTIC_DATA_FILE: &str = "./binary_diagnostic_data.txt";

fn main() {
    let my_sub = move_sub(submarine::Submarine::new());
    // Day 1
    println!("Number of measurements that are larger than the previous measurement: {}", get_increased_depth());

    // Day 2
    println!("Final horizontal position times depth: {}",  my_sub.horizontal_position * my_sub.depth);

    let my_sub = process_diagnostics(my_sub);
    // Day 3
    println!("My diagnostics and count: {:?}, {}", my_sub.diagnostics, my_sub.diagnostic_count);
    println!("My gamma:\t{:?}", my_sub.get_gamma());
    println!("My epsilon:\t{:?}", my_sub.get_epsilon());
    println!("My power consumption: {}", my_sub.get_power_consumption());
}

fn process_diagnostics(mut uboat: Submarine) -> Submarine {
    if let Ok(lines) = read_lines(SUB_BINARY_DIAGNOSTIC_DATA_FILE) {
        for line in lines {
            if let Ok(bits) = line {
                let char_vec: Vec<char> = bits.chars().collect();
                uboat.process_diagnostic_bits(char_vec);
            }
        }
    }
    uboat
}

fn move_sub(mut uboat: Submarine) -> Submarine {
    if let Ok(lines) = read_lines(SUBMARINE_MOVEMENT_DATA_FILE) {
        for line in lines {
            if let Ok(movement) = line {
                let mut iter = movement.split_whitespace();
                if let Some(direction) = iter.next() {
                    if let Some(distance_string) = iter.next() {
                        if let Ok(distance_value) = distance_string.parse() {
                            if direction == "forward" {
                                uboat.forward(distance_value);
                            } else if direction == "down" {
                                uboat.down(distance_value);
                            } else if direction == "up" {
                                uboat.up(distance_value);
                            }
                        }
                    }
                }
            }
        }
    }
    uboat
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// for each line in the file after first,
// compare against the prior line
// if numbers is greater, increment the count
// return the count at the end of the file
fn get_increased_depth() -> i32 {
    let mut current_window = [-1, -1, -1];
        let mut position:usize = 0;
    let mut increased_count = 0;
    // Day 1 part 1let mut prior_line = -1;
    let mut prior_sum = -1;

    if let Ok(lines) = read_lines(DEPTH_DATA_FILE) {
        for line in lines {
            if let Ok(line_value) = line {
                if let Ok(value) = line_value.parse() {
                    current_window[position] = value;
                    if prior_sum == -1 {
                        if position == 2 {
                            prior_sum = current_window.iter().sum();
                        }
                    } else {
                        let current_sum = current_window.iter().sum();
                        if current_sum > prior_sum {
                            increased_count += 1;
                        }
                        prior_sum = current_sum;
                    }
                    if position == 2 {
                        position = 0;
                    } else {
                        position += 1;
                    }
                    /* Day Part 1. Done.
                     if prior_line != -1 {
                         if prior_line < value {
                             increased_count += 1;
                         }
                     }
                     prior_line = value;

                     */
                }
            }
        }
    }
    increased_count
}


