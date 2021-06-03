use rug::Float;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct PiCache {
    pub digits: Vec<u8>,
    pub calculated: bool,
    pub precision: u32,
}

#[allow(dead_code)]
impl PiCache {
    pub fn calculate(&mut self, precision: u32) {
        self.calculated = false;

        let one_precise = Float::with_val(precision, 1.0);
        let four_precise = Float::with_val(precision, 4.0);
        let five_precise = Float::with_val(precision, 5.0);
        let precise_239 = Float::with_val(precision, 239.0);

        let a = four_precise.clone() * (one_precise.clone() / five_precise).atan();
        let b = (one_precise / precise_239).atan();

        let mut digits = (four_precise * (a - b)).to_string().into_bytes();
        digits.remove(0);
        digits.remove(0);

        self.digits = digits;
        self.calculated = true;
        self.precision = precision;
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
                    return current_sequence_start as i128;
                }
                search_index += 1;
            } else {
                search_index = 0;
            }
        }

        return -1;
    }

    pub fn get_digits_in_range_str(&self, range: (usize, usize)) -> String {
        let mut digits_str = String::new();
        let digits = &self.digits[range.0..range.1];

        for ch in digits.iter() {
            digits_str.push(*ch as char);
        }

        return digits_str;
    }

    pub fn get_size_bytes(&self) -> usize {
        let bytes = self.digits.len() * std::mem::size_of::<u8>();
        return bytes;
    }
}
