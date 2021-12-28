use std::fs::File;
use std::io::{BufReader, Error, Lines};

pub struct Point {
    x: i32,
    y: i32,
}

pub struct VentLine {
    start: Point,
    end: Point,
}

pub struct Hydrothermal {
    pub hydrothermal_grid: [[i32; 1000]; 1000],
}

impl Hydrothermal {
    pub fn new() -> Hydrothermal {
        Hydrothermal {
            hydrothermal_grid: [[0; 1000]; 1000]
        }
    }
}