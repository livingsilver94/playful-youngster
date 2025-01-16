use std::mem::size_of;
use std::ops::RangeInclusive;

#[derive(Default)]
pub struct BitFlags8(u8);

impl BitFlags8 {
    pub const fn get(&self, index: usize) -> bool {
        Self::validate_index(index);

        self.0 & (1 << index) != 0
    }

    pub const fn get_range(&self, range: RangeInclusive<usize>) -> u8 {
        Self::validate_index(*range.end());

        let number_of_ones = *range.end() - *range.start() + 1;
        let mask = (1 << number_of_ones) - 1;
        (self.0 >> *range.start()) & mask as u8
    }

    pub const fn set_range(&mut self, range: RangeInclusive<usize>, value: u8) {
        Self::validate_index(*range.end());

        let number_of_ones = *range.end() - *range.start() + 1;
        let mask = (1 << number_of_ones) - 1;

        let value: usize = value as usize & mask;
        self.0 = ((self.0 as usize & !(mask << *range.start())) | (value << *range.start())) as u8;
    }

    const fn validate_index(index: usize) {
        let limit = size_of::<Self>() * 8;
        if (index as usize) >= limit {
            panic!("index out of range");
        }
    }
}

impl From<u8> for BitFlags8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for BitFlags8 {
    fn into(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_within_limit() {
        let tests: Vec<(u8, usize, bool)> = vec![
            (0, 0, false),
            (1, 0, true),
            (2, 0, false),
            (3, 0, true),
            (0b00010000, 4, true),
            (0b10000000, 7, true),
        ];
        for (flag, index, expected) in tests {
            assert_eq!(BitFlags8::from(flag).get(index), expected);
        }
    }

    #[test]
    #[should_panic]
    fn get_outside_limit() {
        BitFlags8::from(0).get(8);
    }

    #[test]
    fn get_range_within_limit() {
        let tests: Vec<(u8, RangeInclusive<usize>, u8)> = vec![
            (0b00000001, 0..=0, 0b1),
            (0b00000111, 0..=2, 0b111),
            (0b11101111, 3..=5, 0b101),
            (0b00111111, 6..=7, 0b00),
            (0b10101010, 0..=7, 0b10101010),
        ];
        for (flag, range, expected) in tests {
            assert_eq!(BitFlags8::from(flag).get_range(range), expected);
        }
    }

    #[test]
    #[should_panic]
    fn get_range_outside_limit() {
        BitFlags8::from(0).get_range(7..=8);
    }

    #[test]
    fn set_range_within_limit() {
        let tests: Vec<(u8, RangeInclusive<usize>, u8, u8)> = vec![
            (0b00000001, 0..=0, 0b0, 0b00000000),
            (0b00000001, 0..=1, 0b11, 0b00000011),
            (0b00011000, 3..=4, 0b00, 0b00000000),
            (0b11111111, 2..=5, 0b0000, 0b11000011),
            (0b10101010, 0..=7, 0b11111111, 0b11111111),
        ];
        for (flag, range, value, expected) in tests {
            let mut flags = BitFlags8::from(flag);
            flags.set_range(range, value);
            assert_eq!(Into::<u8>::into(flags), expected);
        }
    }
}
