use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Mul, Sub};

// BN128 scalar field size
const MODULUS_STR: &str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldElement {
    pub value: BigInt,
}

impl FieldElement {
    pub fn new(value: BigInt) -> Self {
        let modulus = Self::get_modulus();
        let mut v = value % &modulus;
        if v < BigInt::zero() {
            v += &modulus;
        }
        FieldElement { value: v }
    }

    pub fn get_modulus() -> BigInt {
        BigInt::parse_bytes(MODULUS_STR.as_bytes(), 10).expect("Invalid modulus string")
    }

    pub fn from_i32(v: i32) -> Self {
        Self::new(v.to_bigint().unwrap())
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl Add<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: &FieldElement) -> FieldElement {
        FieldElement::new(&self.value + &other.value)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

impl Mul<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &FieldElement) -> FieldElement {
        FieldElement::new(&self.value * &other.value)
    }
}
