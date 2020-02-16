use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    re: f32,
    im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Complex {
        Complex { re, im }
    }

    pub fn mod_arg(r: f32, theta: f32) -> Complex {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn exp_ix(x: f32) -> Complex {
        Complex::mod_arg(1.0, x)
    }

    pub fn from_re(re: f32) -> Complex {
        Self { re, im: 0.0 }
    }
    pub fn zero() -> Complex {
        Complex::new(0.0, 0.0)
    }

    pub fn one() -> Complex {
        Complex::new(1.0, 0.0)
    }

    pub fn i() -> Complex {
        Complex::new(0.0, 1.0)
    }

    pub fn mag_square(&self) -> f32 {
        self.re.powi(2) + self.im.powi(2)
    }

    pub fn norm(&self) -> f32 {
        self.re.hypot(self.im)
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
        let imp = b * c + a * d;
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
