use std::fs::File;
use std::io::{BufReader, Error, Lines};

pub struct FishPopulation {
    fish_population: Vec<i32>,
    day_zeros: usize,
    day_ones: usize,
    day_twos: usize,
    day_threes: usize,
    day_fours: usize,
    day_fives: usize,
    day_sixes: usize,
    day_sevens: usize,
    day_eights: usize,
}

impl FishPopulation {
    pub fn new() -> FishPopulation {
        FishPopulation {
            fish_population: Vec::new(),
            day_zeros: 0,
            day_ones: 0,
            day_twos: 0,
            day_threes: 0,
            day_fours: 0,
            day_fives: 0,
            day_sixes: 0,
            day_sevens: 0,
            day_eights: 0,
        }
    }

    pub fn advance_day(&mut self) {
        let mut temp: usize = self.day_zeros;
        self.day_zeros = self.day_ones;
        self.day_ones = self.day_twos;
        self.day_twos = self.day_threes;
        self.day_threes = self.day_fours;
        self.day_fours = self.day_fives;
        self.day_fives = self.day_sixes;
        self.day_sixes = self.day_sevens + temp;
        self.day_sevens = self.day_eights;
        self.day_eights = temp;
    }

    pub fn total_fish(&self) -> usize {
        self.day_zeros + self.day_ones + self.day_twos + self.day_threes + self.day_fours + self.day_fives +
            self.day_sixes + self.day_sevens + self.day_eights
    }

    pub fn store_population_data(&mut self, reader: Result<Lines<BufReader<File>>, Error>) {
        // take a line of initial fish data and create an initial population
        if let Ok(lines) = reader {
            for line in lines {
                if let Ok(line_value) = line {
                    let initial_fish: Vec<&str> = line_value.split(",").collect();
                    for fish_string in initial_fish {
                        if let Ok(fish) = fish_string.parse() {
                            // First part solution.
                            // self.fish_population.push(fish);

                            // println!("{}", fish);
                            match fish {
                                0 => self.day_zeros += 1,
                                1 => self.day_ones += 1,
                                2 => self.day_twos += 1,
                                3 => self.day_threes += 1,
                                4 => self.day_fours += 1,
                                5 => self.day_fives += 1,
                                6 => self.day_sixes += 1,
                                7 => self.day_sevens += 1,
                                _ => self.day_eights += 1,
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn future(&mut self, days: i32) -> usize {
        
        for i in 0..days {
            self.advance_day();
        }

        self.total_fish()

        // First part solution. Tooo slooooowwww..
        //FishPopulation::future_population(days, &self.fish_population).len()
    }

    // pass vector as immutable slice in rust because we don't need to change our initial population
    fn future_population(days_remaining: i32, initial_population: &[i32]) -> Vec<i32> {
        if days_remaining != 0 {
            let mut new_population = Vec::new();
            let mut new_fish_count = 0;
            for fish in initial_population {
                if *fish == 0 {
                    new_fish_count += 1;
                    new_population.push(6);
                } else {
                    new_population.push(fish - 1);
                }
            }
            for i in 0..new_fish_count {
                new_population.push(8);
            }
            println!("days remaining: {}", days_remaining);
            FishPopulation::future_population(days_remaining - 1, &new_population)
        } else {
            for fish in initial_population {
                print!("{}", fish);
            }
            println!("");
            initial_population.to_vec()
        }
    }
}

