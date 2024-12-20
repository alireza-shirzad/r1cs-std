use crate::Vec;
use ark_ff::Field;
use ark_relations::gr1cs::{Namespace, SynthesisError};
use core::borrow::Borrow;

/// Describes the mode that a variable should be allocated in within
/// a `ConstraintSystem`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum AllocationMode {
    /// Indicate to the `ConstraintSystem` that the high-level variable should
    /// be allocated as a constant. That is, no `Variable`s should be
    /// generated.
    Constant = 0,

    /// Indicate to the `ConstraintSystem` that the high-level variable should
    /// be allocated as a public input to the `ConstraintSystem`.
    Input = 1,

    /// Indicate to the `ConstraintSystem` that the high-level variable should
    /// be allocated as a private witness to the `ConstraintSystem`.
    Witness = 2,
}

impl AllocationMode {
    /// Outputs the maximum according to the relation `Constant < Input <
    /// Witness`.
    pub fn max(&self, other: Self) -> Self {
        use AllocationMode::*;
        match (self, other) {
            (Constant, _) => other,
            (Input, Constant) => *self,
            (Input, _) => other,
            (Witness, _) => *self,
        }
    }
}

/// Specifies how variables of type `Self` should be allocated in a
/// `ConstraintSystem`.
pub trait AllocVar<V: ?Sized, F: Field>: Sized {
    /// Allocates a new variable of type `Self` in the `ConstraintSystem` `cs`.
    /// The mode of allocation is decided by `mode`.
    fn new_variable<T: Borrow<V>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError>;

    /// Allocates a new constant of type `Self` in the `ConstraintSystem` `cs`.
    ///
    /// This should *not* allocate any new variables or constraints in `cs`.
    #[tracing::instrument(target = "gr1cs", skip(cs, t))]
    fn new_constant(
        cs: impl Into<Namespace<F>>,
        t: impl Borrow<V>,
    ) -> Result<Self, SynthesisError> {
        Self::new_variable(cs, || Ok(t), AllocationMode::Constant)
    }

    /// Allocates a new public input of type `Self` in the `ConstraintSystem`
    /// `cs`.
    #[tracing::instrument(target = "gr1cs", skip(cs, f))]
    fn new_input<T: Borrow<V>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
    ) -> Result<Self, SynthesisError> {
        Self::new_variable(cs, f, AllocationMode::Input)
    }

    /// Allocates a new private witness of type `Self` in the `ConstraintSystem`
    /// `cs`.
    #[tracing::instrument(target = "gr1cs", skip(cs, f))]
    fn new_witness<T: Borrow<V>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
    ) -> Result<Self, SynthesisError> {
        Self::new_variable(cs, f, AllocationMode::Witness)
    }
}

/// This blanket implementation just allocates variables in `Self`
/// element by element.
impl<I, F: Field, A: AllocVar<I, F>> AllocVar<[I], F> for Vec<A> {
    fn new_variable<T: Borrow<[I]>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let ns = cs.into();
        let cs = ns.cs();
        f().and_then(|v| {
            v.borrow()
                .iter()
                .map(|e| A::new_variable(cs.clone(), || Ok(e), mode))
                .collect()
        })
    }
}

/// Dummy impl for `()`.
impl<F: Field> AllocVar<(), F> for () {
    fn new_variable<T: Borrow<()>>(
        _cs: impl Into<Namespace<F>>,
        _f: impl FnOnce() -> Result<T, SynthesisError>,
        _mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        Ok(())
    }
}

/// This blanket implementation just allocates variables in `Self`
/// element by element.
impl<I, F: Field, A: AllocVar<I, F>, const N: usize> AllocVar<[I; N], F> for [A; N] {
    fn new_variable<T: Borrow<[I; N]>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let ns = cs.into();
        let cs = ns.cs();
        f().map(|v| {
            let v = v.borrow();
            core::array::from_fn(|i| A::new_variable(cs.clone(), || Ok(&v[i]), mode).unwrap())
        })
    }
}

/// This blanket implementation just allocates variables in `Self`
/// element by element.
impl<I, F: Field, A: AllocVar<I, F>, const N: usize> AllocVar<[I], F> for [A; N] {
    fn new_variable<T: Borrow<[I]>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let ns = cs.into();
        let cs = ns.cs();
        f().map(|v| {
            let v = v.borrow();
            core::array::from_fn(|i| A::new_variable(cs.clone(), || Ok(&v[i]), mode).unwrap())
        })
    }
}
