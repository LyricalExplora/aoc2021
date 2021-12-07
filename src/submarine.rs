use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub struct Submarine {
    pub horizontal_position: i32,
    pub depth: i32,
    pub aim: i32,
    pub gamma: Vec<i32>,
    pub epsilon: Vec<i32>,
    pub oxygen_rating: Vec<i32>,
    pub co2_scrubber_rating: Vec<i32>,
    pub diag_distribution: [i32; 12],
    pub diagnostic_count: i32,
    pub diagnostic_data: Vec<[i32; 12]>,
    pub bingo_draw: Vec<i32>,
    pub bingo_boards: Vec<Vec<i32>>,
    pub bingo_winning_score: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
            diag_distribution: [0; 12],
            diagnostic_count: 0,
            diagnostic_data: Vec::new(),
            gamma: Vec::new(),
            epsilon: Vec::new(),
            oxygen_rating: Vec::new(),
            co2_scrubber_rating: Vec::new(),
            bingo_draw: Vec::new(),
            bingo_boards: vec![vec![0; 5]; 5],
            bingo_winning_score: -1,
        }
    }

    pub fn play_bingo(&mut self, filename :&str) {
        if let Ok(lines) = Submarine::read_lines(filename) {
            for (index, line) in lines.enumerate() {
                if index == 0 {
                    if let Ok(line_value) = line {
                        let values = line_value.split(',');
                        for v in values {
                            if let Ok(parsed) = v.parse() {
                                self.bingo_draw.push(parsed);
                            }
                        }
                    }
                } else {
                    if let Ok(line_value) = line {
                        if line_value.len() > 0 {
                            println!("{:?}", line_value);
                        }
                        //if let Ok(value) = line_value.parse() {}
                    }
                }
            }
        }
    }

    pub fn process_diagnostics(&mut self, filename: &str) {
        if let Ok(lines) = Submarine::read_lines(filename) {
            for line in lines {
                if let Ok(bits) = line {
                    let char_vec: Vec<char> = bits.chars().collect();
                    self.store_diagnostics(char_vec);
                }
            }
        }
    }

    pub fn move_sub(&mut self, filename: &str) {
        if let Ok(lines) = Submarine::read_lines(filename) {
            for line in lines {
                if let Ok(movement) = line {
                    let mut iter = movement.split_whitespace();
                    if let Some(direction) = iter.next() {
                        if let Some(distance_string) = iter.next() {
                            if let Ok(distance_value) = distance_string.parse() {
                                if direction == "forward" {
                                    self.forward(distance_value);
                                } else if direction == "down" {
                                    self.down(distance_value);
                                } else if direction == "up" {
                                    self.up(distance_value);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn get_increased_depth(&self, filename: &str) -> i32 {
        let mut current_window = [-1, -1, -1];
        let mut position:usize = 0;
        let mut increased_count = 0;
        // Day 1 part 1let mut prior_line = -1;
        let mut prior_sum = -1;

        if let Ok(lines) = Submarine::read_lines(filename) {
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

    pub fn get_life_support_rating(&mut self) -> i32 {
        let temp_oxygen = self.get_oxygen();
        let temp_co2 = self.get_co2();
        self.sum_diag_bits(temp_oxygen) * self.sum_diag_bits(temp_co2)
    }

    pub fn get_power_consumption(&self) -> i32 {
        let temp_gamma = self.gamma.to_vec();
        let temp_epsilon = self.epsilon.to_vec();
        self.sum_diag_bits(temp_gamma) * self.sum_diag_bits(temp_epsilon)
    }

    pub fn calculate_life_support_ratings(&mut self, oxygen: bool)  {
        let mut local_diags = self.diagnostic_data.to_vec();
        for column in 0..12 {
            let mut ones: Vec<[i32; 12]> = Vec::new();
            let mut zeros: Vec<[i32; 12]> = Vec::new();
            if local_diags.len() > 1 {
                while let Some(diag_row) = local_diags.pop() {
                    if diag_row[column] == 0 {
                        zeros.push(diag_row);
                    } else {
                        ones.push(diag_row)
                    }
                }
                if oxygen { // we are calculating oxygen, so store ones when equal
                    if zeros.len() > ones.len() {
                        local_diags = zeros;
                    } else {
                        local_diags = ones;
                    }
                } else { // we are handling co2, so store zeros when equal
                    if zeros.len() <= ones.len() {
                        local_diags = zeros;
                    } else {
                        local_diags = ones;
                    }
                }
            }
        }
        if let Some(result) = local_diags.pop() {
            if oxygen {
                self.oxygen_rating = result.to_vec();
            } else {
                self.co2_scrubber_rating = result.to_vec();
            }
        }
    }

    fn get_co2(&mut self) -> Vec<i32> {
        if self.co2_scrubber_rating.len() == 0 {
            self.calculate_life_support_ratings(false);
        }
        self.co2_scrubber_rating.to_vec()
    }

    fn get_oxygen(&mut self) -> Vec<i32> {
        if self.oxygen_rating.len() == 0 {
            self.calculate_life_support_ratings(true);
        }
        self.oxygen_rating.to_vec()
    }

    fn calculate_gamma(&mut self)  {
        let mut result= Vec::new();
        for bit in &self.diag_distribution {
            if bit > &(&self.diagnostic_count / 2) {
                result.push(1);
            } else {
                result.push(0);
            }

        }
        self.gamma = result;
    }

    fn calculate_epsilon(&mut self) {
        let mut result= Vec::new();
        for bit in &self.diag_distribution {
            if bit > &(&self.diagnostic_count / 2) {
                result.push(0);
            } else {
                result.push(1);
            }
        }
        self.epsilon = result;
    }

    fn sum_diag_bits(&self, mut bits:  Vec<i32>) -> i32 {
        let mut result = 0;
        let mut bit_position = 0;
        let base:i32 = 2;
        while let Some(bit) = bits.pop() {
            result = result + (bit * (base.pow(bit_position)));
            bit_position += 1;
        }
        result
    }

    fn forward(&mut self, distance: i32) {
        self.horizontal_position = &self.horizontal_position + distance;
        self.depth = &self.depth + (&self.aim * distance);
    }

    fn down(&mut self, units: i32) {
        self.aim = &self.aim + units;
    }

    fn up(&mut self, units: i32) {
        self.aim = &self.aim - units;
    }

    fn store_diagnostics(&mut self, bits: Vec<char>) {
        let mut diag_row :[i32; 12] = [-1; 12];
        for c in bits.into_iter().enumerate() {
            let (i, x): (usize, char) = c;
            // track distribution of each column for gamma and epsilon calculations
            if x == '1' {
                self.diag_distribution[i] = &self.diag_distribution[i] + 1;
            }
            diag_row[i] = x as i32 - 0x30; // cheap char to i32 for 1s and 0s. UTF-8 dangerous, tho
        }
        self.diagnostic_data.push(diag_row);
        self.diagnostic_count = &self.diagnostic_count + 1;
        self.calculate_gamma();
        self.calculate_epsilon();
    }

}