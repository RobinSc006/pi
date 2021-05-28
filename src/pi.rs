use rug::Float;

pub struct PiCache {
    digits: Vec<u8>,
}

impl PiCache {
    pub fn calculate(precision: u32) -> PiCache {
        let one_precise = Float::with_val(precision, 1.0);
        let four_precise = Float::with_val(precision, 4.0);
        let five_precise = Float::with_val(precision, 5.0);
        let precise_239 = Float::with_val(precision, 239.0);

        let a = four_precise.clone() * (one_precise.clone() / five_precise).atan();
        let b = (one_precise / precise_239).atan();

        return Self {
            digits: (four_precise * (a - b)).to_string().into_bytes(),
        };
    }

    pub fn get_digits_to_prec(&self, prec: usize) -> &[u8] {
        return &self.digits[0..prec];
    }

    pub fn search(&self, sequence: String) -> i128 {
        let search = sequence.as_bytes();
        let mut search_index: usize = 0;

        let mut current_sequence_start: i128 = 0;

        for (index, digit) in self.digits.iter().enumerate() {
            if *digit == search[search_index] {
                if search_index == 0 {
                    current_sequence_start = index as i128;
                }
                if search_index + 1 == search.len() {
                    return (current_sequence_start - 1) as i128;
                }
                search_index += 1;
            } else {
                search_index = 0;
            }
        }

        return -1;
    }
}
