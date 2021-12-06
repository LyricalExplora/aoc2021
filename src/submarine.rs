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

    pub fn maximize_least_bit(&self, bits :Vec<i32>) -> Vec<i32> {
        // this is a shortcut and may have a bug

        let mut result = bits;
        if let Some(peek) = result.pop() {
            result.push(1);
        }
        result
    }

    pub fn calculate_oxygen_rating(&mut self)  {
        // create a temp space for current results
        let mut temp:Vec<i32> = Vec::new();
        let mut temp2:Vec<i32> = Vec::new();
        let mut diag_row: [i32; 12] = [0; 12];

        // pop one
        if let Some(diag_row) = self.diagnostic_data.pop() {

        }
        // evaluate it for the current column
        // include an evaluation for if its the last item or if count is balanced (need counters for both)
        // if match, push into temp2
        // when done, make temp = temp 2
        // increment column for evaluation
        // if no more columns, return last pushed value
        self.oxygen_rating = temp;

    }

    pub fn get_co2_scrubber_rating(&self)  {

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

    //pub fn get_life_support_rating(&self) -> i32 {
        //self.calculate_oxygen_rating();
        //self.calculate_co2_scrubber_rating();
     //   self.sum_diag_bits() *
       //     self.sum_diag_bits()
  //  }

    pub fn get_power_consumption(&self) -> i32 {
        let mut temp_gamma = self.gamma.to_vec();
        let mut temp_epsilon = self.epsilon.to_vec();
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