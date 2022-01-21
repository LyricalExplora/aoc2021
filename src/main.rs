mod submarine;
mod hydrothermals;
mod lanternfish;
mod crabs;

use std::fs::{File, read};
use std::io::{self, BufRead};
use std::path::Path;

const DEPTH_DATA_FILE: &str = "./sea_depth_data.txt";
const SUBMARINE_MOVEMENT_DATA_FILE: &str = "./submarine_movement_data.txt";
const SUB_BINARY_DIAGNOSTIC_DATA_FILE: &str = "./binary_diagnostic_data.txt";
const SUB_TEST_DIAGNOSTIC_DATA_FILE: &str = "./test_diag_data.txt";
const SUB_TEST_BINGO_DATA_FILE: &str = "./test_bingo_data.txt";
const SUB_BINGO_DATA_FILE: &str = "./bingo_data_file.txt";
const SUB_TEST_HYDRO_THERMAL_DATA_FILE: &str = "./test_hydro_thermal_data.txt";
const SUB_TEST2_HYDRO_THERMAL_DATA_FILE: &str = "./test_hydro_thermal_data2.txt";
const SUB_HYDRO_THERMAL_DATA_FILE: &str = "./hydro_thermal_data.txt";
const FISH_POPULATION_DATA_FILE: &str = "./lanternfish_data.txt";
const TEST_FISH_POPULATION_DATA_FILE: &str = "./test_lanternfish_data.txt";
const CRAB_POPULATION_DATA_FILE: &str = "./crab_fuel_data.txt";
const TEST_CRAB_POPULATION_DATA_FILE: &str = "./test_crab_fuel_data.txt";



fn main() {
    let mut my_sub = submarine::Submarine::new();
    // Day 1 - depth
    let depth_reader = read_lines(DEPTH_DATA_FILE);
    // println!("Number of measurements that are larger than the previous measurement: {}",
    //         my_sub.get_increased_depth(depth_reader));

    // Day 2 - movement
    let movement_reader = read_lines(SUBMARINE_MOVEMENT_DATA_FILE);
    my_sub.move_sub(movement_reader);
    //println!("Final horizontal position times depth: {}",  my_sub.horizontal_position * my_sub.depth);

    // Day 3 - life support
    let diag_reader = read_lines(SUB_BINARY_DIAGNOSTIC_DATA_FILE);
    my_sub.process_diagnostics(diag_reader);
    //println!("My power consumption: {}", my_sub.get_power_consumption());
    //println!("My life support rating: {}", my_sub.get_life_support_rating());

    // Day 4 - bingo
    let bingo_reader = read_lines(SUB_BINGO_DATA_FILE);
    my_sub.play_bingo(bingo_reader);
    // println!("Winning bingo score: {}", my_sub.bingo_winning_score);

    // Day 5 - hydrothermal lines
    let mut my_thermals = hydrothermals::Hydrothermal::new();
    my_thermals.store_hydrothermal_data(read_lines(SUB_HYDRO_THERMAL_DATA_FILE));
    my_thermals.fill_grid();
    // println!("Number of overlapping lines: {} ", my_thermals.draw_grid());

    // Day 6 - lanternfish
    let mut fish_population = lanternfish::FishPopulation::new();
    fish_population.store_population_data(read_lines(FISH_POPULATION_DATA_FILE));
    // println!("Fish population after 80 days: {}", fish_population.future(256));

    // Day 7 - Treachery of Whales aka Crab Fuel
    let mut crab_swarm = crabs::CrabSwarm::new();
    crab_swarm.store_crab_swarm_position(read_lines(CRAB_POPULATION_DATA_FILE));
    println!("Crab swarm minimum fuel required, part 1: {} ",  crab_swarm.get_min_dist_sum());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}