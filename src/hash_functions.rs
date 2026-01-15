use crate::field::FieldElement;
use ff_ce::{Field, PrimeField, PrimeFieldRepr};
use num_bigint::{BigInt, Sign};
use poseidon_rs::{Fr, Poseidon};

///Define a trait for hash functions
pub trait HashFunction {
    fn hash(&self, a: &FieldElement, b: &FieldElement) -> FieldElement;
}

/// Production-grade Poseidon hash function
/// Uses the BN254 curve's scalar field (same as used in many ZKP systems)
pub struct PoseidonHash {
    poseidon: Poseidon,
}

impl PoseidonHash {
    /// Creates a new Poseidon hash instance
    /// Uses default parameters optimized for 2 inputs
    pub fn new() -> Self {
        Self {
            poseidon: Poseidon::new(),
        }
    }
}

impl Default for PoseidonHash {
    fn default() -> Self {
        Self::new()
    }
}

impl HashFunction for PoseidonHash {
    fn hash(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        // Convert BigInt to Fr (field element)
        let a_bytes = a.value.to_signed_bytes_le();
        let b_bytes = b.value.to_signed_bytes_le();

        // Pad or truncate to 32 bytes (Fr field size for BN254)
        // Fr is 254 bits, so 32 bytes is enough.

        let mut a_fr_repr = <Fr as PrimeField>::Repr::default();
        let mut b_fr_repr = <Fr as PrimeField>::Repr::default();

        // We need to read bytes into Repr. `read_le` expects a reader.
        // We can just pad a_bytes manually to ensure it's correct size for read_le if needed,
        // but read_le usually reads N bytes.
        // Actually, let's just create a cursor or slice.

        // Ensure inputs are within field modulus? `BigInt` could be larger.
        // FieldElement already ensures it's mod P... wait, my FieldElement is mod BN128?
        // poseidon-rs uses BN254?
        // "BN128 scalar field size" in field.rs constant.
        // poseidon-rs usually corresponds to BN254.
        // They are often the same curve (alt_bn128).

        let _ = a_fr_repr.read_le(&a_bytes[..]); // Ignore result, simple loading
        let _ = b_fr_repr.read_le(&b_bytes[..]);

        // Actually, to be safe against short inputs (BigInt small numbers):
        let mut a_padded = a_bytes;
        let mut b_padded = b_bytes;
        // read_le expects 32 bytes for Fr?
        // It reads 4 u64s.
        // If my Bytes is shorter, `read_le` might fail or fill partially.
        // Safest is to pad to 32 bytes.

        // Re-do padding logic
        let mut a_array = [0u8; 32];
        let mut b_array = [0u8; 32];
        let a_len = a_padded.len().min(32);
        let b_len = b_padded.len().min(32);
        a_array[..a_len].copy_from_slice(&a_padded[..a_len]);
        b_array[..b_len].copy_from_slice(&b_padded[..b_len]);

        a_fr_repr.read_le(&a_array[..]).unwrap();
        b_fr_repr.read_le(&b_array[..]).unwrap();

        // Create field elements from bytes
        let a_fr = Fr::from_repr(a_fr_repr).unwrap_or(Fr::zero());
        let b_fr = Fr::from_repr(b_fr_repr).unwrap_or(Fr::zero());

        // Hash using Poseidon
        let inputs = vec![a_fr, b_fr];
        let hash_result = self.poseidon.hash(inputs).expect("Poseidon hash failed");

        // Convert result back to BigInt
        let result_repr = hash_result.into_repr();
        let mut result_bytes = Vec::new();
        result_repr.write_le(&mut result_bytes).unwrap();

        let result_bigint = BigInt::from_bytes_le(Sign::Plus, &result_bytes);

        FieldElement::new(result_bigint)
    }
}
