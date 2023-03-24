mod bindings;

// Expose relevant types with idiomatic names.
pub use bindings::{KZGSettings as KzgSettings, C_KZG_RET as CkzgError};
// Expose the constants.
pub use bindings::{
    BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_FIELD_ELEMENT, BYTES_PER_PROOF,
    FIELD_ELEMENTS_PER_BLOB,
};
// Expose the remaining relevant types.
pub use bindings::{Blob, Bytes32, Bytes48, Error, FFTSettings, KzgCommitment, KzgProof};
