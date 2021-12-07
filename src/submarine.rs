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

    pub fn store_diagnostics(&mut self, bits: Vec<char>) {
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

    // i'd love to not borrow in the case we have a value in co2. how?
    pub fn get_co2(&mut self) -> Vec<i32> {
        if self.co2_scrubber_rating.len() == 0 {
            self.calculate_life_support_ratings(false);
        }
        self.co2_scrubber_rating.to_vec()
    }

    pub fn get_oxygen(&mut self) -> Vec<i32> {
        if self.oxygen_rating.len() == 0 {
            self.calculate_life_support_ratings(true);
        }
        self.oxygen_rating.to_vec()
    }

    pub fn calculate_gamma(&mut self)  {
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

    pub fn calculate_epsilon(&mut self) {
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