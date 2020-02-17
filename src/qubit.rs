use std::f32::consts::FRAC_1_SQRT_2;
use std::ops::Neg;

use assert_approx_eq::assert_approx_eq;
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub struct Qubit {
    pub(crate) p_0: Complex,
    pub(crate) p_1: Complex,
}

impl Qubit {
    pub fn sample_is_zero(&self) -> bool {
        SmallRng::from_entropy().gen_bool(self.p_0.mag_square().into())
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
        assert_approx_eq!(1.0, p_0.mag_square() + p_1.mag_square());
        Qubit { p_0, p_1 }
    }

    pub fn zero() -> Self {
        Qubit {
            p_0: Complex::one(),
            p_1: Complex::zero(),
        }
    }

    pub fn one() -> Self {
        Qubit {
            p_0: Complex::zero(),
            p_1: Complex::one(),
        }
    }

    pub fn plus() -> Self {
        Qubit {
            p_0: FRAC_1_SQRT_2.into(),
            p_1: FRAC_1_SQRT_2.into(),
        }
    }

    pub fn minus() -> Self {
        Qubit {
            p_0: FRAC_1_SQRT_2.into(),
            p_1: (-FRAC_1_SQRT_2).into(),
        }
    }

    /// |ψ〉= cos(θ/2)|0〉+ e^iφ sin(θ/2)|1〉
    pub fn from_theta_phi(theta: f32, phi: f32) -> Self {
        Qubit::new(
            (theta / 2.0).cos().into(), // Rustc can do the type interface
            Complex::exp_ix(phi) * (theta / 2.0).sin(),
        )
    }
    /// |ψ〉= e^iγ (cos(θ/2)|0〉+e^iφ sin(θ/2)|1〉),
    pub fn from_theta_phi_gamma(theta: f32, phi: f32, gamma: f32) -> Self {
        let phase_shift = Complex::exp_ix(gamma);
        let ket_0: Complex = (theta / 2.0).cos().into();
        let ket_1 = Complex::exp_ix(phi) * (theta / 2.0).sin();
        Qubit::new(phase_shift * ket_0, phase_shift * ket_1)
    }

    #[cfg(test)]
    pub fn assert_approx_eq(&self, other: &Self) {
        assert_approx_eq!((self.p_0 - other.p_0).norm(), 0.0);
        assert_approx_eq!((self.p_1 - other.p_1).norm(), 0.0);
    }
}

impl Neg for Qubit {
    type Output = Qubit;
    fn neg(self) -> Self {
        Self {
            p_0: -self.p_0,
            p_1: -self.p_1,
        }
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
        Qubit::from_theta_phi_gamma(45675235.523, 56421465.23, 134432.43);
    }
}
