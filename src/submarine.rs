pub struct Submarine {
    pub horizontal_position: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            horizontal_position: 0,
            depth: 0,
            aim: 0
        }
    }

    pub fn forward(&mut self, distance: i32) {
        self.horizontal_position = &self.horizontal_position + distance;
        self.depth = &self.depth + (&self.aim * distance);
    }

    pub fn down(&mut self, units: i32) {
        self.aim = &self.aim + units;
    }

    pub fn up(&mut self, units: i32) {
        self.aim = &self.aim - units;
    }
}