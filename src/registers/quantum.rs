use std::convert::TryInto;

use rand;

use super::classical::ClassicalRegister;
use crate::complex::Complex;

pub struct QuantumRegister {
    n_qubits: u8,
    qubits: Vec<Complex>,
}

impl QuantumRegister {
    pub fn from_classical(cr: ClassicalRegister) -> Self {
        let mut qubits = vec![Complex::zero(); 2usize.pow(8)];
        qubits[cr.bits as usize] = Complex::one();

        let ret = Self {
            n_qubits: 8,
            qubits,
        };

        // This should always be true
        debug_assert!(Self::is_valid(&ret));
        ret
    }

    #[must_use]
    fn is_valid(&self) -> bool {
        let mut acc = 0.0;
        for i in &self.qubits {
            acc += i.mag_square()
        }
        (acc - 1.0).abs() <= 1.0e-6
    }

    // Target should be a random float. Used for edge case tests.
    fn collapse_with_target(&self, target: f32) -> ClassicalRegister {
        let target = target % 1.0;
        let mut current = 0.0;
        // Handle for floating point problems
        let mut reserve: Option<u8> = None;
        for (bits, im_prob) in self.qubits.iter().enumerate() {
            let prob = im_prob.mag_square();
            current += prob;
            if current > target {
                return ClassicalRegister {
                    bits: bits
                        .try_into()
                        .expect("This should never be more than 255"),
                };
            // Set the reserve to whatever
            } else if prob != 0.0 {
                reserve = Some(bits.try_into().unwrap());
            }
        }
        // If we didn't get anything, use the reserve which must be something
        // because some item must have non zero probability
        ClassicalRegister {
            bits: reserve.unwrap(),
        }
    }

    pub fn collapse(&self) -> ClassicalRegister {
        self.collapse_with_target(rand::random::<f32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_classical() {
        for i in 0..255 {
            let reg = QuantumRegister::from_classical(i.into());
            assert_eq!(reg.collapse(), i.into())
        }
    }

    #[test]
    fn collapse_bell_state_even_dist() {
        let bell_state = bell_state();
        let mut count_00 = 0;
        let mut count_11 = 0;
        for _ in 0..1000 {
            let collapsed = bell_state.collapse();
            match collapsed.bits {
                0b00 => count_00 += 1,
                0b11 => count_11 += 1,
                _ => unreachable!(
                    "Bell state can only collapse to 00 or 11"
                ),
            }
        }
        assert_eq!(count_00 + count_11, 1000);
        assert!(425 < count_00, "Too few |00>");
        assert!(count_00 < 575, "Too many |00>");
        assert!(425 < count_11, "Too few |11>");
        assert!(count_11 < 575, "Too many |11>");
    }

    fn bell_state() -> QuantumRegister {
        let ket_00 = 0b00;
        let ket_11 = 0b11;
        let mut qubits = vec![Complex::zero(); 2usize.pow(2)];
        qubits[ket_00] = std::f32::consts::FRAC_1_SQRT_2.into();
        qubits[ket_11] = std::f32::consts::FRAC_1_SQRT_2.into();
        QuantumRegister {
            n_qubits: 2,
            qubits,
        }
    }

    #[test]
    // Expensive to run
    fn bell_collapse_never_panics_slow() {
        let bell_state = bell_state();
        for _ in 1..100000000u64 {
            bell_state.collapse();
        }
    }

    #[test]
    fn bell_state_colapse_target_approx_1() {
        let target = 0.99999994;
        let res = bell_state().collapse_with_target(target).bits;
        match res {
            0b00 | 0b11 => {}
            _ => panic!("Invalid bell state collapse"),
        }
    }
}
