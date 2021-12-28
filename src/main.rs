mod submarine;
mod hydrothermals;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DEPTH_DATA_FILE: &str = "./sea_depth_data.txt";
const SUBMARINE_MOVEMENT_DATA_FILE: &str = "./submarine_movement_data.txt";
const SUB_BINARY_DIAGNOSTIC_DATA_FILE: &str = "./binary_diagnostic_data.txt";
const SUB_TEST_DIAGNOSTIC_DATA_FILE: &str = "./test_diag_data.txt";
const SUB_TEST_BINGO_DATA_FILE: &str = "./test_bingo_data.txt";
const SUB_BINGO_DATA_FILE: &str = "./bingo_data_file.txt";
const SUB_TEST_HYDRO_THERMAL_DATA_FILE: &str = "./test_hydro_thermal_data.txt";
const SUB_HYDRO_THERMAL_DATA_FILE: &str = "./hydro_thermal_data_file.txt";


fn main() {
    let mut my_sub = submarine::Submarine::new();
    // Day 1
    let depth_reader = read_lines(DEPTH_DATA_FILE);
    println!("Number of measurements that are larger than the previous measurement: {}",
             my_sub.get_increased_depth(depth_reader));

    // Day 2
    let movement_reader = read_lines(SUBMARINE_MOVEMENT_DATA_FILE);
    my_sub.move_sub(movement_reader);
    println!("Final horizontal position times depth: {}",  my_sub.horizontal_position * my_sub.depth);

    // Day 3
    let diag_reader = read_lines(SUB_BINARY_DIAGNOSTIC_DATA_FILE);
    my_sub.process_diagnostics(diag_reader);
    println!("My power consumption: {}", my_sub.get_power_consumption());
    println!("My life support rating: {}", my_sub.get_life_support_rating());

    // Day 4
    let bingo_reader = read_lines(SUB_BINGO_DATA_FILE);
    my_sub.play_bingo(bingo_reader);
    println!("Winning bingo score: {}", my_sub.bingo_winning_score);

    // Day 5
    let hydrothermal_reader = read_lines(SUB_TEST_HYDRO_THERMAL_DATA_FILE);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}