use std::io::Write;
use num_biginit::BigInt;
use crate::hash_functions::HashFunction;
use crate::rlcs::{Operation, R1CS, Variable};

pub enum Gate {
    Add(usize, usize, usize), //Add: input1, input2, output
    Mul(usize, usize, usize), //Mul: input1, input2, output
    Hash(usize, usize, usize), //Hash: input, output
}

pub struct Circuit {
    hash_function: Option<Box<dyn HashFunction>>,
    inputs: Vec<BigInt>,
    gates: Vec<Gate>,
    outputs: Vec<BigInt>,
}

impl Circuit {
    
}

///Apply hash function,  defaulting to a simple operationn if none is provided
pub fn apply_hash(&self, a: &BigInt) -> BigInt {
    
}

/// Retrieves an input value by index, if it exists
pub fn get_input(&self, index: usize) -> Option<&BigInt> {
    
}   