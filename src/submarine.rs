pub struct Submarine {
    pub horizontal_position: i32,
    pub depth: i32,
    pub aim: i32,
    pub diag_distribution: [i32; 12],
    pub diagnostic_count: i32,
    pub diagnostic_data: Vec<[i32; 12]>,
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

    pub fn store_diagnostic_distribution(&mut self, bits: Vec<char>) {
        let mut diag_row :[i32; 12] = [-1; 12];
        for c in bits.into_iter().enumerate() {
            let (i, x): (usize, char) = c;

            // track distribution of each column for gamma and epsilon calculations
            if x == '1' {
                self.diag_distribution[i] = &self.diag_distribution[i] + 1;
            }

            //
            diag_row[i] = x as i32 - 0x30; // cheap hack but good for 1s and 0s
        }
        self.diagnostic_data.push(diag_row);
        self.diagnostic_count = &self.diagnostic_count + 1;
    }

    pub fn maximize_least_bit(&self, bits :Vec<i32>) -> Vec<i32> {
        // this is a shortcut and may have a bug

        let mut result = bits;
        if let Some(peek) = result.pop() {
            result.push(1);
        }
        result
    }

    pub fn get_oxygen_rating(&self) -> Vec<i32> {
        self.get_gamma()
    }

    pub fn get_co2_scrubber_rating(&self) -> Vec<i32> {
        self.get_gamma()
    }

    pub fn get_gamma(&self) -> Vec<i32> {
        let mut result= Vec::new();
        for bit in &self.diag_distribution {
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
        for bit in &self.diag_distribution {
            if bit > &(&self.diagnostic_count / 2) {
                result.push(0);
            } else {
                result.push(1);
            }

        }
        result
    }

    pub fn get_life_support_rating(&self) -> i32 {
        self.sum_diag_bits(self.get_oxygen_rating()) *
            self.sum_diag_bits(self.get_co2_scrubber_rating())
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
        }
        result
    }
}