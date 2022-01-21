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

    fn sum_distance(distance: i64) -> i64 {

        let mut result = 0;

        for i in 0..distance {
            result += i;
        }
        result
    }

    pub fn get_min_dist_sum(&self) -> i64 {
        let mut result = i64::MAX;
        if let Some(min) = self.positions.iter().min() {
            if let Some(max) = self.positions.iter().max() {
                for position in *min..=*max {
                    // find the minimum value for each position across all the crabs
                    let mut new_minimum = 0;
                    for crab in &self.positions {
                        new_minimum += (crab - position).abs() + CrabSwarm::sum_distance((crab - position).abs());
                    }
                    result = result.min(new_minimum);
                }
            }
        }
        result
    }
}