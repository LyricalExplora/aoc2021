pub struct Submarine {
    pub horizontal_position: i32,
    pub depth: i32,
    pub aim: i32,
    pub diagnostics: [i32; 12],
    pub diagnostic_count: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
            diagnostics: [0; 12],
            diagnostic_count: 0,
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

    pub fn process_diagnostic_bits(&mut self, bits: Vec<char>) {
        for c in bits.into_iter().enumerate() {
            let (i, x): (usize, char) = c;
            if x == '1' {
                self.diagnostics[i] = &self.diagnostics[i] + 1;
            }
        }
        self.diagnostic_count = &self.diagnostic_count + 1;
    }

    pub fn get_gamma(&self) -> Vec<i32> {
        let mut result= Vec::new();
        for bit in &self.diagnostics {
            if bit > &(&self.diagnostic_count / 2) {
                result.push(1);
            } else {
                result.push(0);
            }

        }
        result
    }

    pub fn get_epsilon(&self) -> Vec<i32> {
        let mut result= Vec::new();
        for bit in &self.diagnostics {
            if bit > &(&self.diagnostic_count / 2) {
                result.push(0);
            } else {
                result.push(1);
            }

        }
        result
    }

    pub fn get_power_consumption(&self) -> i32 {
        self.sum_diag_bits(self.get_epsilon()) * self.sum_diag_bits(self.get_gamma())
    }

    pub fn sum_diag_bits(&self, mut bits:  Vec<i32>) -> i32 {
        let mut result = 0;
        let mut bit_position = 0;
        let base:i32 = 2;
        while let Some(bit) = bits.pop() {
            result = result + (bit * (base.pow(bit_position)));
            bit_position += 1;
            println!("result, bit, bit_position {} {} {}", result, bit, bit_position);

        }
        result
    }
}