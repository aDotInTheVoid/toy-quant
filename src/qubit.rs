//! A single unentangled qubit

use std::f32::consts::FRAC_1_SQRT_2;
use std::ops::Neg;

use approx::assert_relative_eq;
use nalgebra::Vector2;
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::complex::Complex;

use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Clone, PartialEq)]
pub struct Qubit {
    pub(crate) inner: Vector2<Complex>,
}

impl Qubit {
    pub fn sample_is_zero(&self) -> bool {
        SmallRng::from_entropy()
            .gen_bool(self.inner.index(0).mag_square().into())
    }
    pub fn sample_is_one(&self) -> bool {
        !self.sample_is_zero()
    }
    pub fn sample(&self) -> f32 {
        if self.sample_is_zero() {
            0.0
        } else {
            1.0
        }
    }
    pub fn new(p_0: Complex, p_1: Complex) -> Self {
        assert_relative_eq!(1.0, p_0.mag_square() + p_1.mag_square());
        Qubit {
            inner: Vector2::new(p_0, p_1),
        }
    }
    pub fn zero() -> Self {
        Self::new(Complex::one(), Complex::zero())
    }
    pub fn one() -> Self {
        Self::new(Complex::zero(), Complex::one())
    }

    pub fn plus() -> Self {
        Self::new(FRAC_1_SQRT_2.into(), FRAC_1_SQRT_2.into())
    }

    pub fn minus() -> Self {
        Self::new(FRAC_1_SQRT_2.into(), (-FRAC_1_SQRT_2).into())
    }

    pub fn from_theta_phi(theta: f32, phi: f32) -> Self {
        Qubit::new(
            (theta / 2.0).cos().into(),
            Complex::exp_ix(phi) * (theta / 2.0).sin(),
        )
    }

    pub fn from_theta_phi_gamma(
        theta: f32,
        phi: f32,
        gamma: f32,
    ) -> Self {
        let phase_shift = Complex::exp_ix(gamma);
        let ket_0: Complex = (theta / 2.0).cos().into();
        let ket_1 = Complex::exp_ix(phi) * (theta / 2.0).sin();
        Qubit::new(phase_shift * ket_0, phase_shift * ket_1)
    }
}

impl Neg for Qubit {
    type Output = Qubit;
    fn neg(self) -> Self {
        Self { inner: -self.inner }
    }
}

impl AbsDiffEq for Qubit {
    type Epsilon = <Vector2<Complex> as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        Vector2::<Complex>::default_epsilon()
    }
    fn abs_diff_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
    ) -> bool {
        Vector2::<Complex>::abs_diff_eq(
            &self.inner,
            &other.inner,
            epsilon,
        )
    }
}

impl RelativeEq for Qubit {
    fn default_max_relative(
    ) -> <Vector2<Complex> as AbsDiffEq>::Epsilon {
        Vector2::<Complex>::default_epsilon()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        Vector2::<Complex>::relative_eq(
            &self.inner,
            &other.inner,
            epsilon,
            max_relative,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_vals_no_panic() {
        Qubit::plus();
        Qubit::zero();
        Qubit::one();
    }
    #[test]
    #[should_panic]
    fn too_low_panics() {
        Qubit::new(Complex::zero(), Complex::zero());
    }
    #[test]
    #[should_panic]
    fn too_high_panics() {
        Qubit::new(Complex::one(), Complex::one());
    }
    #[test]
    fn legal_new_constructors() {
        Qubit::new(Complex::i(), Complex::zero());
        Qubit::new(-Complex::i(), Complex::zero());
        Qubit::new(Complex::i(), -Complex::zero());
        Qubit::new(Complex::one(), Complex::zero());
        Qubit::new(Complex::zero(), -Complex::one());
    }

    #[test]
    fn from_ang_for_any_vals() {
        Qubit::from_theta_phi(5.2, 542.23);
        Qubit::from_theta_phi(32.52, 145.54);
        Qubit::from_theta_phi(24.54, 5154.576);
        Qubit::from_theta_phi_gamma(35235.523, 321465.23, 5432.43);
        Qubit::from_theta_phi_gamma(12235.523, 211465.23, 21432.43);
        Qubit::from_theta_phi_gamma(
            45675235.523,
            56421465.23,
            134432.43,
        );
    }
}
