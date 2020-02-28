//! Complex numbers
use num_traits::identities::{One, Zero};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub,
    SubAssign,
};

/// A complex number
///
/// ```rust
/// # use toy_quant::complex::Complex;
/// let x = Complex::new(1.0, 0.0);
/// assert_eq!(x, Complex::one());
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f32,
    im: f32,
}

impl Complex {
    /// Create a complex number from a real and imaginary part
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    /// Create a complex number from a modulus and argument
    pub fn mod_arg(r: f32, theta: f32) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Create a complex number e^ix, equivalent to mod_arg(1, x)
    pub fn exp_ix(x: f32) -> Complex {
        Complex::mod_arg(1.0, x)
    }

    /// Create a complex number with a real part and no imaginary part
    pub fn from_re(re: f32) -> Complex {
        Self { re, im: 0.0 }
    }

    /// The complex number 0 + 0i
    pub fn zero() -> Complex {
        Complex::new(0.0, 0.0)
    }

    /// The complex number 1 + 0i
    pub fn one() -> Complex {
        Complex::new(1.0, 0.0)
    }

    /// √-1
    pub fn i() -> Complex {
        Complex::new(0.0, 1.0)
    }

    /// |x|²
    pub fn mag_square(self) -> f32 {
        self.re.powi(2) + self.im.powi(2)
    }

    /// |x|
    pub fn norm(self) -> f32 {
        self.re.hypot(self.im)
    }
    /// The complex conjugate. Re(x) - i Im (x). a-bi
    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            re: self.re * other,
            im: self.im * other,
        }
    }
}

impl Mul<Complex> for f32 {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        other * self
    }
}

impl From<f32> for Complex {
    fn from(num: f32) -> Complex {
        Complex::from_re(num)
    }
}

impl Div<Complex> for Complex {
    type Output = Self;
    // We have tests for this, and clippy freaks out
    // when I have a addition in a division function.
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Self) -> Self {
        let a = self.re;
        let b = self.im;
        let c = other.re;
        let d = other.im;
        // a+bi   (a+bi)(c-di)   (ac+bd) + i(bc-ad)
        // ---- = ------------ = ------------------
        // c+di   (c+di)(c-di)     c^2 + d^2
        let denom = c.powi(2) + d.powi(2);
        let rep = a * c + b * d;
        let imp = b * c - a * d;
        Self {
            re: rep / denom,
            im: imp / denom,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Complex) {
        *self = *self + other;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Complex) {
        *self = *self - other;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Complex) {
        *self = *self * other;
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, other: Complex) {
        *self = *self / other;
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        0.0.into()
    }
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl One for Complex {
    fn one() -> Self {
        (1.0).into()
    }
    fn is_one(&self) -> bool {
        self == &Self::one()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn divide() {
        let a = Complex::new(3.0, 2.0);
        let b = Complex::new(4.0, -3.0);
        assert_eq!(a / b, Complex::new(6.0 / 25.0, 17.0 / 25.0));
        let a = Complex::new(4.0, 5.0);
        let b = Complex::new(2.0, 6.0);
        assert_eq!(a / b, Complex::new(19.0 / 20.0, -7.0 / 20.0));
        let a = Complex::new(2.0, -1.0);
        let b = Complex::new(-3.0, 6.0);
        assert_eq!(a / b, Complex::new(-4.0 / 15.0, -1.0 / 5.0));
        let a = Complex::new(2.0, -1.0);
        let b = Complex::new(-3.0, 6.0);
        assert_eq!(a / b, Complex::new(-4.0 / 15.0, -1.0 / 5.0));
        let a = Complex::new(-6.0, -3.0);
        let b = Complex::new(4.0, 6.0);
        assert_eq!(a / b, Complex::new(-21.0 / 26.0, 6.0 / 13.0));
    }
}
