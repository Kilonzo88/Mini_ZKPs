use num_bigint::BigInt;

///Define a trait for hash functions
pub trait HashFunction {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt;
}

/// A simple addition hash function(for demo purposes only)
pub struct SimpleAddHash;

impl HashFunction for SimpleAddHash {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a + b
    }
}

/// A hash function that uses a unique combination formula
pub struct CustomHash;

impl HashFunction for CustomHash {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a + b
    }
}
