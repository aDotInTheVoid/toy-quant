use crate::complex::Complex;
use crate::registers::quantum::QuantumRegister;

use approx::assert_relative_eq;
use nalgebra::U4;

type Matrix = nalgebra::Matrix4<Complex>;
type MatrixU8 = nalgebra::Matrix4<u8>;
type Register2 = QuantumRegister<U4>;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryGate {
    mat: Matrix,
}

impl BinaryGate {
    pub fn new(mat: Matrix) -> Self {
        let x = mat * mat.transpose().map(|x| x.conj());
        assert_relative_eq!(x, Matrix::identity());
        Self { mat }
    }

    pub fn new_u8(mat: MatrixU8) -> Self {
        Self::new(mat.map(Complex::from))
    }

    pub fn apply(&self, qubits: Register2) -> Register2 {
        Register2::from_vector(self.mat * qubits.into_vector())
    }

    pub fn compose(&self, other: &Self) -> Self {
        Self::new(self.mat * other.mat)
    }

    pub fn swap(&self) -> Self {
        gates::swap().compose(self).compose(&gates::swap())
    }
}

pub mod gates {
    use super::*;
    #[rustfmt::skip]
    /// The Controlled Not (CNOT) gate
    /// 
    /// ```text
    /// |A> ---●--- |A>
    ///        | 
    /// |B> ---⊕--- |A ⊕ B>
    /// ```
    /// ```rust
    /// # use toy_quant::qubit::Qubit;
    /// # use toy_quant::gates::binary::gates::cnot;
    /// ```
    /// 
    pub fn cnot() -> BinaryGate {
        BinaryGate::new_u8(MatrixU8::new(
            1, 0, 0, 0, 
            0, 1, 0, 0,
            0, 0, 0, 1, 
            0, 0, 1, 0,
        ))
    }

    pub fn swap() -> BinaryGate {
        BinaryGate::new_u8(MatrixU8::new(
            1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cnot_propertys() {
        let x = gates::cnot();
        // CNOT ^2 = 1
        // ---+---+---
        //    |   |
        // ---@---@---
        // Equiv to
        // -----
        //
        // -----
        assert_relative_eq!(x.compose(&x).mat, Matrix::identity());
        // ---+---@---+---
        //    |   |   |
        // ---@---+---@---
        //
        // Equiv to
        //
        // ---X---
        //    |
        // ---X---
        assert_relative_eq!(
            x.compose(&x.swap()).compose(&x).mat,
            gates::swap().mat
        );
        assert_relative_eq!(
            gates::swap().mat,
            x.swap().compose(&x).compose(&x.swap()).mat
        )
    }

    #[test]
    fn swap_propertys() {
        let x = gates::swap().mat;
        assert_relative_eq!(x * x, Matrix::identity());
    }

    #[test]
    fn cnot_zero_one() {
        use crate::qubit::Qubit;
        use crate::registers::quantum::QuantumRegister;
        let cnot = gates::cnot();

        for (c_in, t_in, c_out, t_out) in [
            (
                Qubit::zero(),
                Qubit::zero(),
                Qubit::zero(),
                Qubit::zero(),
            ),
            (
                Qubit::zero(),
                Qubit::one(),
                Qubit::zero(),
                Qubit::one(),
            ),
            (Qubit::one(), Qubit::zero(), Qubit::one(), Qubit::one()),
            (Qubit::one(), Qubit::one(), Qubit::one(), Qubit::zero()),
        ]
        .iter()
        .cloned()
        {
            let reg_in = QuantumRegister::from_2_qubits(c_in, t_in);
            let reg_out =
                QuantumRegister::from_2_qubits(c_out, t_out);
            assert_eq!(cnot.apply(reg_in), reg_out);
        }
    }
}
