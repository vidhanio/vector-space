use std::{
    array,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use rand::prelude::*;

use super::VectorSpace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntPolynomial<const N: usize> {
    pub coefficients: [i32; N],
}

impl<const N: usize> IntPolynomial<N> {
    pub fn new(coefficients: [i32; N]) -> Self {
        Self { coefficients }
    }
}

impl<const N: usize> VectorSpace for IntPolynomial<N> {
    type Field = f32;

    const ZERO: Self = Self {
        coefficients: [0; N],
    };

    fn new_random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            coefficients: array::from_fn(|_| rng.gen::<i8>().into()),
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut coefficients = [0; N];

        coefficients
            .iter_mut()
            .zip(self.iter().zip(other.iter()))
            .for_each(|(c, (a, b))| *c = a + b);

        Self { coefficients }
    }

    fn sub(&self, other: &Self) -> Self {
        let mut coefficients = [0; N];

        coefficients
            .iter_mut()
            .zip(self.iter().zip(other.iter()))
            .for_each(|(c, (a, b))| *c = a - b);

        Self { coefficients }
    }

    fn mul(&self, field: &Self::Field) -> Self {
        let mut coefficients = [0; N];

        coefficients
            .iter_mut()
            .zip(self.iter())
            .for_each(|(c, a)| *c = (*a as f32 * *field) as i32);

        Self { coefficients }
    }

    fn neg(&self) -> Self {
        let mut coefficients = [0; N];

        coefficients
            .iter_mut()
            .zip(self.iter())
            .for_each(|(c, a)| *c = -a);

        Self { coefficients }
    }
}

impl<const N: usize> Display for IntPolynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().enumerate().try_for_each(|(i, c)| match i {
            0 => write!(f, "{}", c),
            1 => write!(f, " + {}x", c),
            i => write!(f, " + {}x^{}", c, i),
        })
    }
}

impl<const N: usize> Deref for IntPolynomial<N> {
    type Target = [i32; N];

    fn deref(&self) -> &Self::Target {
        &self.coefficients
    }
}

impl<const N: usize> DerefMut for IntPolynomial<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coefficients
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_ok;

    use super::*;

    #[test]
    fn is_vector_space() {
        assert_ok!(IntPolynomial::<3>::verify_axioms());
    }
}
