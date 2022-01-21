use std::fs::File;
use std::io::{BufReader, Error, Lines};
use std::num::ParseIntError;

const GRID_SIZE: usize = 1000;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct VentLine {
    pub start: Point,
    pub end: Point,
}

pub struct Hydrothermal {
    pub hydrothermal_grid: [[i32; GRID_SIZE]; GRID_SIZE],
    pub vent_lines: Vec<VentLine>,
}

impl Hydrothermal {
    pub fn new() -> Hydrothermal {
        Hydrothermal {
            hydrothermal_grid: [[0; GRID_SIZE]; GRID_SIZE],
            vent_lines: Vec::new(),
        }
    }

    fn hydrothermal_parse(line: String) -> Result<VentLine, ParseIntError> {
        let points: Vec<&str> = line.split(" -> ").collect();
        let start_coords: Vec<&str> = points[0].split(",").collect();
        let end_coords: Vec<&str> = points[1].split(",").collect();
        let start: Point = Point{ x: start_coords[0].parse()?, y: start_coords[1].parse()? };
        let end: Point = Point { x: end_coords[0].parse()?, y: end_coords[1].parse()? };
        Ok(VentLine { start: start, end: end })
    }

    pub fn store_hydrothermal_data(&mut self, reader: Result<Lines<BufReader<File>>, Error>) {
        // for each line, parse the start coordinates and end coordinates
        // make a Point for each pair
        // make a new VentLine from the start and end Points
        // push the Ventline into the vent_lines vector
        if let Ok(lines) = reader {
            for line in lines {
                if let Ok(line_value) = line {
                    if let Ok(ventline) = Hydrothermal::hydrothermal_parse(line_value) {
                        self.vent_lines.push(ventline);
                    }
                }
            }
        }
    }

    fn is_diagonal(line: &VentLine) -> bool {
        let mut x_diff: usize = 0;
        let mut y_diff= 0;
        if line.start.x < line.end.x {
            x_diff = line.end.x - line.start.x;
        } else {
            x_diff = line.start.x - line.end.x;
        }
        if line.start.y < line.end.y {
            y_diff = line.end.y - line.start.y;
        } else {
            y_diff = line.start.y - line.end.y;
        }

        // println!("{} {}", x_diff, y_diff);

        x_diff == y_diff
    }

    pub fn fill_grid(&mut self) {
        self.fill_no_diags();
        self.fill_diags() ;
    }

    fn fill_diags(&mut self) {
        for line in &self.vent_lines {
            if Hydrothermal::is_diagonal(&line) {
                let mut x_iter = line.start.x..=line.end.x;
                let mut y_iter = line.start.y..=line.end.y;
                let mut rev_x_iter = (line.end.x..=line.start.x).rev();
                let mut rev_y_iter = (line.end.y..=line.start.y).rev();

                if line.start.x < line.end.x {
                    if line.start.y < line.end.y {
                        // little x, little y so no reverse
                        while let Some(xc) = x_iter.next() {
                            if let Some(yc) = y_iter.next() {
                                self.hydrothermal_grid[xc][yc] += 1;
                            }
                        }
                    } else {
                        // little x, big y so reverse y
                        while let Some(xc) = x_iter.next() {
                            if let Some(yc) = rev_y_iter.next() {
                                self.hydrothermal_grid[xc][yc] += 1;
                            }
                        }
                    }
                } else {
                    if line.start.y < line.end.y {
                        // big x, little y so reverse x
                        while let Some(xc) = rev_x_iter.next() {
                            if let Some(yc) = y_iter.next() {
                                self.hydrothermal_grid[xc][yc] += 1;
                            }
                        }
                    } else {
                        // big x, big y so reverse both
                        while let Some(xc) = rev_x_iter.next() {
                            if let Some(yc) = rev_y_iter.next() {
                                self.hydrothermal_grid[xc][yc] += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn fill_no_diags(&mut self) {
        for line in &self.vent_lines {
            // 1st part, just do straight lines
            if line.start.y == line.end.y {
                if line.start.x > line.end.x {
                    for i in line.end.x..line.start.x + 1 {
                        self.hydrothermal_grid[i][line.start.y] += 1;
                    }
                } else {
                    for i in line.start.x..line.end.x + 1 {
                        self.hydrothermal_grid[i][line.start.y] += 1;
                    }
                }
            } else if line.start.x == line.end.x {
                if line.start.y > line.end.y {
                    for j in line.end.y..line.start.y + 1 {
                        self.hydrothermal_grid[line.start.x][j] += 1;
                    }
                } else {
                    for j in line.start.y..line.end.y + 1 {
                        self.hydrothermal_grid[line.start.x][j] += 1;
                    }
                }
            }
        }
    }

    pub fn draw_grid(&self) -> i32 {
        let mut result = 0;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.hydrothermal_grid[j][i] == 0 {
                    print!(".");
                }
                else {
                    if self.hydrothermal_grid[j][i] > 1 {
                        result += 1;
                    }
                    print!("{}", self.hydrothermal_grid[j][i]);
                }
            }
            println!("");
        }
        result
    }
}
