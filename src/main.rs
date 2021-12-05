use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}


// for each line in the file after first,
// compare against the prior linee
// if numbers is greater, increment the count
// return the count at the end of the file
fn get_increased() -> i32 {
    let mut current_window = [-1, -1, -1];
    println!("{:?}", current_window);
    let mut position:usize = 0;
    let mut increased_count = 0;
    // Day 1 part 1let mut prior_line = -1;
    let mut prior_sum = -1;


    if let Ok(lines) = read_lines("/Users/chris/dev/advent_21_1/data.txt") {
        for line in lines {
            if let Ok(line_value) = line {
                if let Ok(value) = line_value.parse() {
                    current_window[position] = value;
                    println!("{:?}", current_window);
                    if prior_sum == -1 {
                        if position == 2 {
                            prior_sum = current_window.iter().sum();
                            println!("first window sum {} and current window {:?}", prior_sum, current_window);
                        }
                    } else {
                        let current_sum = current_window.iter().sum();
                        println!("prior window sum {} and current sum {} and increased_count {}", prior_sum, current_sum, increased_count);
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

fn main() {
    println!("Number of measurements that are larger than the previous measurement: {}", get_increased());
}
