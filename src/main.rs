mod circuit;
mod field;
mod hash_functions;
mod r1cs;

use field::FieldElement;
use hash_functions::{HashFunction, SimpleAddHash};

fn main() {
    println!("--- ZKP Mini Framework ---");

    // Demonstrate Simple Hash with FieldElement
    let hasher = SimpleAddHash;
    let input1 = FieldElement::from_i32(10);
    let input2 = FieldElement::from_i32(20);

    let hash_result = hasher.hash(&input1, &input2);

    println!("Input 1: {}", input1);
    println!("Input 2: {}", input2);
    println!("Hash Result: {}", hash_result);
}
