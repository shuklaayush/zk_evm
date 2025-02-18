//! This module contains type aliases and custom `Error` definition for
//! convenient proof generation.

use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

/// A type alias for proofs generated by the zkEVM.
pub type PlonkyProofIntern = ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2>;

/// A type alias for the set of preprocessed circuits necessary to generate
/// succinct block proofs.
pub type AllRecursiveCircuits = evm_arithmetization::fixed_recursive_verifier::AllRecursiveCircuits<
    GoldilocksField,
    PoseidonGoldilocksConfig,
    2,
>;

/// A type alias for the verifier data necessary to verify succinct block
/// proofs.
/// While the prover state [`AllRecursiveCircuits`] can also verify proofs, this
/// [`VerifierData`] is much lighter, allowing anyone to verify block proofs,
/// regardless of the underlying hardware.
pub type VerifierData =
    plonky2::plonk::circuit_data::VerifierCircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2>;
