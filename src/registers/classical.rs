use std::iter::FromIterator;
use std::iter::*;

#[derive(Debug)]
pub struct ClassicalRegister {
    pub bits: u8,
}

/// This will panic if the iterator has more that 64 elements
impl FromIterator<bool> for ClassicalRegister {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        let mut bits = 0;
        for (n_bits, bit) in iter.into_iter().enumerate() {
            bits |= (bit as u8) << n_bits;
        }
        Self { bits }
    }
}

impl ClassicalRegister {
    pub fn index(&self, index: u8) -> bool {
        ((self.bits >> index) & 1) == 1
    }

    pub fn set(&mut self, index: u8, val: bool) {
        if val {
            self.bits |= 1 << index;
        } else {
            self.bits &= !(1 << index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::repeat;

    #[test]
    fn from_bit_array() {
        let x: ClassicalRegister = [true, true, false, false, true, false, true]
            .iter()
            .copied()
            .collect();
        assert_eq!(x.bits, 0b1010011);

        let y: ClassicalRegister = [true; 0].iter().copied().collect();
        assert_eq!(y.bits, 0);

        let z: ClassicalRegister = repeat(true).take(8).collect();
        assert_eq!(z.bits, std::u8::MAX);

        let a: ClassicalRegister = repeat(false).take(8).collect();
        assert_eq!(a.bits, 0);
    }

    #[test]
    #[should_panic(expected = "attempt to shift left with overflow")]
    fn from_overfull_iter() {
        let _ = repeat(true).take(9).collect::<ClassicalRegister>();
    }

    #[test]
    fn index() {
        let reg = ClassicalRegister {
            // -----76543210
            bits: 0b11001110,
        };
        assert_eq!(reg.index(0), false);
        assert_eq!(reg.index(1), true);
        assert_eq!(reg.index(2), true);
        assert_eq!(reg.index(3), true);
        assert_eq!(reg.index(4), false);
        assert_eq!(reg.index(5), false);
        assert_eq!(reg.index(6), true);
        assert_eq!(reg.index(7), true);
    }

    #[test]
    #[should_panic]
    fn index_8_panics() {
        ClassicalRegister { bits: 8 }.index(8);
    }

    #[test]
    #[should_panic]
    fn index_100_on_8_panics() {
        ClassicalRegister { bits: 0 }.index(100);
    }

    #[test]
    fn set_index() {
        let mut reg = ClassicalRegister { bits: 0 };
        reg.set(1, true);
        eprintln!("{:b}", reg.bits);
        reg.set(3, true);
        eprintln!("{:b}", reg.bits);

        reg.set(7, true);
        eprintln!("{:b}", reg.bits);

        reg.set(2, false);
        eprintln!("{:b}", reg.bits);

        reg.set(0, false);
        eprintln!("{:b}", reg.bits);

        // --------------------76543210
        assert_eq!(reg.bits, 0b10001010);
        reg.set(7, false);
        reg.set(0, true);
        // --------------------76543210
        assert_eq!(reg.bits, 0b00001011);
        reg.set(6, true);
        reg.set(1, false);
        reg.set(1, true);
        reg.set(1, false);
        reg.set(6, false);
        reg.set(1, true);
        reg.set(0, false);
        reg.set(6, true);
        // --------------------76543210
        assert_eq!(reg.bits, 0b01001010);
    }
}
