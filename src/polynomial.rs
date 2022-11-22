use std::{
    array,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use rand::prelude::*;

use super::VectorSpace;

#[derive(Debug, Clone, Copy)]
pub struct Polynomial<const N: usize> {
    pub coefficients: [<Self as VectorSpace>::Field; N],
}

impl<const N: usize> Polynomial<N> {
    pub fn new(coefficients: [<Self as VectorSpace>::Field; N]) -> Self {
        Self { coefficients }
    }
}

impl<const N: usize> VectorSpace for Polynomial<N> {
    type Field = f64;

    const ZERO: Self = Self {
        coefficients: [0.0; N],
    };

    fn new_random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            coefficients: array::from_fn(|_| rng.gen()),
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut coefficients = [0.0; N];

        coefficients
            .iter_mut()
            .zip(self.iter().zip(other.iter()))
            .for_each(|(c, (a, b))| *c = a + b);

        Self { coefficients }
    }

    fn sub(&self, other: &Self) -> Self {
        let mut coefficients = [0.0; N];

        coefficients
            .iter_mut()
            .zip(self.iter().zip(other.iter()))
            .for_each(|(c, (a, b))| *c = a - b);

        Self { coefficients }
    }

    fn mul(&self, field: &Self::Field) -> Self {
        let mut coefficients = [0.0; N];

        coefficients
            .iter_mut()
            .zip(self.iter())
            .for_each(|(c, a)| *c = a * field);

        Self { coefficients }
    }

    fn neg(&self) -> Self {
        let mut coefficients = [0.0; N];

        coefficients
            .iter_mut()
            .zip(self.iter())
            .for_each(|(c, a)| *c = -a);

        Self { coefficients }
    }
}

impl<const N: usize> Display for Polynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().enumerate().try_for_each(|(i, c)| match i {
            0 => write!(f, "{}", c),
            1 => write!(f, " + {}x", c),
            i => write!(f, " + {}x^{}", c, i),
        })
    }
}

impl<const N: usize> PartialEq for Polynomial<N> {
    fn eq(&self, other: &Self) -> bool {
        const TOLERANCE: f64 = 1e-6;

        self.iter()
            .zip(other.iter())
            .all(|(a, b)| (a - b).abs() < TOLERANCE)
    }
}

impl<const N: usize> Deref for Polynomial<N> {
    type Target = [<Self as VectorSpace>::Field; N];

    fn deref(&self) -> &Self::Target {
        &self.coefficients
    }
}

impl<const N: usize> DerefMut for Polynomial<N> {
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
        assert_ok!(Polynomial::<3>::verify_axioms());
    }
}
