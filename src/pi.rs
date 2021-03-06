use std::time::{Duration, Instant};

use rug::Float;

#[derive(Clone, Debug, Default)]
pub struct PiCache {
    pub digits: Vec<u8>,
    pub calculated: bool,
    pub searched: bool,
    pub precision: u32,
    pub generation_time: Duration,
    pub current_search_result: i128,
}

#[allow(dead_code)]
impl PiCache {
    /// Computes Pi using machin's formula
    /// https://en.wikipedia.org/wiki/Machin-like_formula
    pub fn calculate(&mut self, precision: u32) {
        let start = Instant::now();

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

        self.precision = digits.len() as u32;
        self.calculated = true;
        self.digits = digits;

        self.generation_time = start.elapsed();
    }
    /// Searches for a given sequence in generated digits
    pub fn search(&mut self, sequence: String) {
        let search = sequence.as_bytes();
        let mut search_index: usize = 0;

        let mut current_sequence_start: i128 = 0;

        for (index, digit) in self.digits.iter().enumerate() {
            if *digit == search[search_index] {
                if search_index == 0 {
                    current_sequence_start = index as i128;
                }
                if search_index + 1 == search.len() {
                    self.current_search_result = current_sequence_start as i128;
                    self.searched = true;
                    return;
                }
                search_index += 1;
            } else {
                search_index = 0;
            }
        }

        self.searched = true;
        self.current_search_result = -1;
    }

    pub fn get_digits_in_range_str(&self, range: (usize, usize)) -> String {
        let mut digits_str = String::new();
        let digits = &self.digits[range.0..range.1];

        for ch in digits.iter() {
            digits_str.push(*ch as char);
        }

        return digits_str;
    }

    pub fn get_digits_in_range(&self, range: (usize, usize)) -> Vec<u8> {
        return self.digits[range.0..range.1].to_vec();
    }

    /// Returns size of the digits vector in bytes
    pub fn get_size_bytes(&self) -> usize {
        let bytes = self.digits.len() * std::mem::size_of::<u8>();
        return bytes;
    }
}
