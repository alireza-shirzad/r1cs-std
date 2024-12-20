use ark_r1cs_std::{
    alloc::AllocVar, convert::ToConstraintFieldGadget, fields::emulated_fp::EmulatedFpVar, GR1CSVar,
};
use ark_relations::gr1cs::ConstraintSystem;

#[test]
fn to_constraint_field_test() {
    type F = ark_bls12_377::Fr;
    type CF = ark_bls12_377::Fq;

    let cs = ConstraintSystem::<CF>::new_ref();

    let a = EmulatedFpVar::Constant(F::from(12u8));
    let b = EmulatedFpVar::new_input(cs.clone(), || Ok(F::from(6u8))).unwrap();

    let b2 = &b + &b;

    let a_to_constraint_field = a.to_constraint_field().unwrap();
    let b2_to_constraint_field = b2.to_constraint_field().unwrap();

    assert_eq!(a_to_constraint_field.len(), b2_to_constraint_field.len());
    for (left, right) in a_to_constraint_field
        .iter()
        .zip(b2_to_constraint_field.iter())
    {
        assert_eq!(left.value(), right.value());
    }
}
