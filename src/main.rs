mod submarine;


use crate::submarine::Submarine;

const DEPTH_DATA_FILE: &str = "./sea_depth_data.txt";
const SUBMARINE_MOVEMENT_DATA_FILE: &str = "./submarine_movement_data.txt";
const SUB_BINARY_DIAGNOSTIC_DATA_FILE: &str = "./binary_diagnostic_data.txt";
const SUB_TEST_DIAGNOSTIC_DATA_FILE: &str = "./test_diag_data.txt";
const SUB_TEST_BINGO_DATA_FILE: &str = "./test_bingo_data.txt";
const SUB_BINGO_DATA_FILE: &str = "./bingo_data_file.txt";

fn main() {
    let mut my_sub = submarine::Submarine::new();
    my_sub.move_sub(SUBMARINE_MOVEMENT_DATA_FILE);
    my_sub.process_diagnostics(SUB_TEST_DIAGNOSTIC_DATA_FILE);
    my_sub.process_diagnostics(SUB_BINARY_DIAGNOSTIC_DATA_FILE);
    my_sub.play_bingo(SUB_TEST_BINGO_DATA_FILE);

    // Day 1
    println!("Number of measurements that are larger than the previous measurement: {}", my_sub.get_increased_depth(DEPTH_DATA_FILE));

    // Day 2
    println!("Final horizontal position times depth: {}",  my_sub.horizontal_position * my_sub.depth);

    // Day 3
    println!("My power consumption: {}", my_sub.get_power_consumption());
    println!("My life support rating: {}", my_sub.get_life_support_rating());

    // Day 4
    println!("Winning bingo score: {}", my_sub.bingo_winning_score);
    println!("{:?}", my_sub.bingo_draw);
    println!("{:?}", my_sub.bingo_boards[1][1][1]);
}


