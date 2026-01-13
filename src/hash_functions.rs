use crate::field::FieldElement;

///Define a trait for hash functions
pub trait HashFunction {
    fn hash(&self, a: &FieldElement, b: &FieldElement) -> FieldElement;
}

/// A simple addition hash function(for demo purposes only)
pub struct SimpleAddHash;

impl HashFunction for SimpleAddHash {
    fn hash(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        a + b
    }
}

/// A hash function that uses a unique combination formula
pub struct CustomHash;

impl HashFunction for CustomHash {
    fn hash(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        a + b // Placeholder, could be more complex
    }
}
