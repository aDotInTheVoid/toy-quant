//! Gates that map a qubit to a qubit

use std::f32::consts::FRAC_1_SQRT_2;

use crate::complex::Complex;
use crate::matrix::Matrix2x2;
use crate::qubit::Qubit;

use num_traits::identities::{one, zero};

/// A Unary Gate. Maps a qubit to a qubit
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryGate {
    mat: Matrix2x2<Complex>,
}

impl UnaryGate {
    /// Create a unary gate.
    ///
    /// Panics if mat is not [Unitary](https://en.wikipedia.org/wiki/Unitary_matrix)
    pub fn new(mat: Matrix2x2<Complex>) -> Self {
        let x = &mat * &mat.transpose().map(|x| x.conj());
        x.assert_approx_eq(&Matrix2x2::identity());
        Self { mat }
    }
    /// Takes a qubit and runs the gate on it.
    #[allow(clippy::many_single_char_names)]
    pub fn run(&self, q: Qubit) -> Qubit {
        let &Matrix2x2(a, b, c, d) = &self.mat;
        let Qubit { p_0: e, p_1: f } = q;
        // https://www.wolframalpha.com/input/?i=%7B%7Ba%2Cb%7D%2C%7Bc%2Cd%7D%7D+*+%7B%7Be%7D%2C%7Bf%7D%7D
        Qubit::new(a * e + b * f, c * e + d * f)
    }
}

pub mod gates {
    use super::*;
    /// Create a [not / Pauli-X](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-X_gate) gate.
    pub fn not() -> UnaryGate {
        UnaryGate::new(Matrix2x2(zero(), one(), one(), zero()))
    }

    /// Create a [z / Pauli-Z](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-Z_(%7F'%22%60UNIQ--postMath-00000028-QINU%60%22'%7F)_gate) gate
    pub fn z() -> UnaryGate {
        UnaryGate::new(Matrix2x2(one(), zero(), zero(), -one::<Complex>()))
    }

    /// Create a [h / Hadamard](https://en.wikipedia.org/wiki/Quantum_logic_gate#Hadamard_(H)_gate) gate
    pub fn h() -> UnaryGate {
        UnaryGate::new(Matrix2x2(one(), one(), one(), -one::<Complex>()).map(|x| x * FRAC_1_SQRT_2))
    }

    /// Gates from [Pauli matrices](https://en.wikipedia.org/wiki/Pauli_matrices)
    pub mod pauli {
        use super::*;
        /// [Pauli X](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-X_gate), equivalent to NOT
        pub fn x() -> UnaryGate {
            not()
        }
        /// [Pauli-Y](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-Y_gate)
        pub fn y() -> UnaryGate {
            UnaryGate::new(Matrix2x2(zero(), -Complex::i(), Complex::i(), zero()))
        }
        /// [Pauli-Z](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-Z_(%7F'%22%60UNIQ--postMath-00000028-QINU%60%22'%7F)_gate), sometimes called Z
        pub fn z() -> UnaryGate {
            super::z()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn not_ab_is_ba() {
            let not = not();
            assert_eq!(not.run(Qubit::zero()), Qubit::one());
            assert_eq!(not.run(Qubit::one()), Qubit::zero());
            assert_eq!(not.run(Qubit::plus()), Qubit::plus());
        }
        #[test]
        fn z_works() {
            let z = z();
            assert_eq!(z.run(Qubit::zero()), Qubit::zero());
            assert_eq!(z.run(Qubit::one()), -Qubit::one());
            assert_eq!(z.run(-Qubit::one()), Qubit::one())
        }
        #[test]
        fn h_works() {
            let h = h();
            h.run(Qubit::zero()).assert_approx_eq(&Qubit::plus());
            h.run(Qubit::one()).assert_approx_eq(&Qubit::minus());
            h.run(-Qubit::one()).assert_approx_eq(&-Qubit::minus());
            h.run(-Qubit::zero()).assert_approx_eq(&-Qubit::plus());
            h.run(Qubit::plus()).assert_approx_eq(&Qubit::zero());
            h.run(Qubit::minus()).assert_approx_eq(&Qubit::one());
            h.run(-Qubit::plus()).assert_approx_eq(&-Qubit::zero());
            h.run(-Qubit::minus()).assert_approx_eq(&-Qubit::one());
        }

        #[test]
        fn h_squared_is_i() {
            let h = h().mat;
            let i = &h * &h;
            i.assert_approx_eq(&Matrix2x2::identity())
        }
        #[test]
        fn pauli_squared_is_i() {
            for i in &[pauli::x(), pauli::y(), pauli::z()] {
                let m = &i.mat;
                let i = m * m;
                i.assert_approx_eq(&Matrix2x2::identity())
            }
        }
    }
}
