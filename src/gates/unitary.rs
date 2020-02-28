//! Gates that map a qubit to a qubit

use std::f32::consts::FRAC_1_SQRT_2;

use crate::complex::Complex;
use crate::qubit::Qubit;

use approx::assert_relative_eq;
use nalgebra;

use num_traits::identities::{one, zero};

type Matrix = nalgebra::Matrix2<Complex>;

/// A Unary Gate. Maps a qubit to a qubit
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryGate {
    mat: Matrix,
}

impl UnaryGate {
    /// Create a unary gate.
    ///
    /// Panics if mat is not [Unitary](https://en.wikipedia.org/wiki/Unitary_matrix)
    pub fn new(mat: Matrix) -> Self {
        let x = mat * mat.transpose().map(|x| x.conj());
        assert_relative_eq!(x, Matrix::identity());
        Self { mat }
    }
    /// Takes a qubit and runs the gate on it.
    pub fn run(&self, q: Qubit) -> Qubit {
        Qubit {
            inner: self.mat * q.inner,
        }
    }
}

pub mod gates {
    use super::*;
    /// Create a [not / Pauli-X](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-X_gate) gate.
    pub fn not() -> UnaryGate {
        UnaryGate::new(Matrix::new(zero(), one(), one(), zero()))
    }

    /// Create a [z / Pauli-Z](https://en.wikipedia.org/wiki/Quantum_logic_gate#Pauli-Z_(%7F'%22%60UNIQ--postMath-00000028-QINU%60%22'%7F)_gate) gate
    pub fn z() -> UnaryGate {
        UnaryGate::new(Matrix::new(
            one(),
            zero(),
            zero(),
            -one::<Complex>(),
        ))
    }

    /// Create a [h / Hadamard](https://en.wikipedia.org/wiki/Quantum_logic_gate#Hadamard_(H)_gate) gate
    pub fn h() -> UnaryGate {
        UnaryGate::new(
            Matrix::new(one(), one(), one(), -one::<Complex>())
                .map(|x| x * FRAC_1_SQRT_2),
        )
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
            UnaryGate::new(Matrix::new(
                zero(),
                -Complex::i(),
                Complex::i(),
                zero(),
            ))
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
            assert_relative_eq!(
                h.run(Qubit::zero()),
                (&Qubit::plus())
            );
            assert_relative_eq!(h.run(Qubit::one()), &Qubit::minus());
            assert_relative_eq!(
                h.run(-Qubit::one()),
                &-Qubit::minus()
            );
            assert_relative_eq!(
                h.run(-Qubit::zero()),
                &-Qubit::plus()
            );
            assert_relative_eq!(h.run(Qubit::plus()), &Qubit::zero());
            assert_relative_eq!(h.run(Qubit::minus()), &Qubit::one());
            assert_relative_eq!(
                h.run(-Qubit::plus()),
                &-Qubit::zero()
            );
            assert_relative_eq!(
                h.run(-Qubit::minus()),
                &-Qubit::one()
            );
        }

        #[test]
        fn h_squared_is_i() {
            let h = h().mat;
            let i = &h * &h;
            assert_relative_eq!(i, Matrix::identity());
        }
        #[test]
        fn pauli_squared_is_i() {
            for i in &[pauli::x(), pauli::y(), pauli::z()] {
                let m = &i.mat;
                let i = m * m;
                assert_relative_eq!(i, Matrix::identity());
            }
        }
    }
}
