use std::fs::File;
use std::io::{BufReader, Error, Lines};

pub struct CrabSwarm {
    positions: Vec<i64>,
}

impl CrabSwarm {
    pub fn new() -> CrabSwarm {
        CrabSwarm{
            positions: Vec::new(),
        }
    }

    pub fn store_crab_swarm_position(&mut self, reader: Result<Lines<BufReader<File>>, Error>) {
        // take a line of initial crab position data and store it
        if let Ok(lines) = reader {
            for line in lines {
                if let Ok(line_value) = line {
                    let initial_crabs: Vec<&str> = line_value.split(",").collect();
                    for crab_string in initial_crabs {
                        if let Ok(crab) = crab_string.parse::<i64>() {
                            // Part 1, assume single variable dimension but plan for 2 dimensions
                            self.positions.push(crab);
                        }
                    }
                }
            }
        }
        self.positions.sort();
    }

    pub fn median(&self) -> i64 {
        let mid = self.positions.len() / 2;
        self.positions[mid]
    }

    pub fn get_min_dist_sum(&self) -> i64 {
        let min = *self.positions.iter().min() ;
        let max = *self.positions.iter().max().unwrap();
        let mut result = i64::MAX;

        for postion in min..=max {
            let mut new_result = 0;
            for crab in &self.positions {
                new_result += (crab - postion).abs();
            }

            result = result.min(new_result);
        }
        result
    }
}