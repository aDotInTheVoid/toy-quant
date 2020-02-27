/// Matrices
use assert_approx_eq::assert_approx_eq;
use num_traits::identities::{one, zero, One, Zero};

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::complex::Complex;

/// A 2x2 Matrix
///
/// ``` text
/// self.0 self.1
/// self.2 self.3
/// ```
#[derive(Clone, Eq, Hash, Debug, PartialEq, Default)]
pub struct Matrix2x2<T>(pub T, pub T, pub T, pub T);

#[rustfmt::skip]
impl<'a, T> Mul<&'a Matrix2x2<T>> for &'a Matrix2x2<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
    type Output = Matrix2x2<T>;
    #[allow(clippy::many_single_char_names)]
    fn mul(self, rhs: Self) -> Matrix2x2<T> {
        let &Matrix2x2(a, b, c, d) = self;
        let &Matrix2x2(w, x, y, z) = rhs;
        // https://www.wolframalpha.com/input/?i=%7B%7Ba%2Cb%7D%2C%7Bc%2Cd%7D%7D+%7B%7Bw%2Cx%7D%2C%7By%2Cz%7D%7D
        Matrix2x2(
            a*w + b*y,   a*x + b*z, 
            c*w + d*y,   c*x + d*z
        )
    }
}

impl<T> Matrix2x2<T> {
    /// Run f on every element on self
    pub fn map_inline<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        f(&mut self.0);
        f(&mut self.1);
        f(&mut self.2);
        f(&mut self.3);
    }

    /// Create a matrix by applying f on every element in self
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(T) -> T,
        T: Copy,
    {
        Self(f(self.0), f(self.1), f(self.2), f(self.3))
    }

    /// The [Determinant](https://en.wikipedia.org/wiki/Determinant) of self
    pub fn det(&self) -> T
    where
        T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
    {
        let &Matrix2x2(a, b, c, d) = self;
        a * d - b * c
    }

    #[rustfmt::skip]
    /// The [transpose](https://en.wikipedia.org/wiki/Transpose)
    pub fn transpose(&self) -> Self
    where
        T: Copy,
    {
        let &Matrix2x2(a, b, c, d) = self;
        Matrix2x2(
            a, c,
            b, d
        )
    }

    /// Calculate the [Inverse](https://en.wikipedia.org/wiki/Invertible_matrix) of the matrix
    ///
    /// Panics if the matrix is [Singular](https://en.wikipedia.org/wiki/Singular_matrix)
    pub fn inv(&self) -> Self
    where
        T: Div<T, Output = T>
            + Neg<Output = T>
            + Copy
            + Sub<T, Output = T>
            + Mul<T, Output = T>,
    {
        let &Self(a, b, c, d) = self;
        Self(d, -b, -c, a).map(|x| x / self.det())
    }

    /// Returns the 2x2 [identity matrix](https://en.wikipedia.org/wiki/Identity_matrix)
    pub fn identity() -> Self
    where
        T: Zero + One,
    {
        Self(one(), zero(), zero(), one())
    }
}

impl Matrix2x2<Complex> {
    /// Panics if self and other have element more that 1.0e-6 apart
    pub fn assert_approx_eq(&self, other: &Self) {
        assert_approx_eq!(self.0.norm(), other.0.norm());
        assert_approx_eq!(self.1.norm(), other.1.norm());
        assert_approx_eq!(self.2.norm(), other.2.norm());
        assert_approx_eq!(self.3.norm(), other.3.norm());
    }
}
