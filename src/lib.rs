use rand::{distributions::Standard, prelude::*};

pub mod int_polynomial;
pub mod polynomial;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxiomFailure {
    Associativity,
    Commutativity,
    Identity,
    Inverse,
    Distributivity,
}

pub trait VectorSpace {
    type Field;

    const ZERO: Self;

    fn new_random() -> Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, field: &Self::Field) -> Self;
    fn neg(&self) -> Self;

    fn verify_axioms() -> Result<(), AxiomFailure>
    where
        Self: Sized + PartialEq,
        Standard: Distribution<Self::Field>,
    {
        let a = Self::new_random();
        let b = Self::new_random();
        let c = Self::new_random();

        // Associativity
        if a.add(&b).add(&c) != a.add(&b.add(&c)) {
            return Err(AxiomFailure::Associativity);
        }

        // Commutativity
        if a.add(&b) != b.add(&a) {
            return Err(AxiomFailure::Commutativity);
        }

        // Identity
        if a.add(&Self::ZERO) != a {
            return Err(AxiomFailure::Identity);
        }

        // Inverse
        if a.sub(&a) != Self::ZERO {
            return Err(AxiomFailure::Inverse);
        }

        // Distributivity
        let field = rand::random();

        if a.mul(&field).add(&b.mul(&field)) != a.add(&b).mul(&field) {
            return Err(AxiomFailure::Distributivity);
        }

        Ok(())
    }
}
