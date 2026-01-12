use crate::hash_functions::HashFunction;
use crate::r1cs::{Operation, R1CS, Variable};
use num_bigint::BigInt;
use std::io::Write;

pub enum Gate {
    Add(usize, usize, usize),  //Add: input1, input2, output
    Mul(usize, usize, usize),  //Mul: input1, input2, output
    Hash(usize, usize, usize), //Hash: input, output
}

pub struct Circuit {
    hash_function: Option<Box<dyn HashFunction>>, // Uses `Box<dyn ...>` to enable **Runtime Polymorphism**. For example (e.g., **SHA256 vs Poseidon**
    inputs: Vec<BigInt>,
    gates: Vec<Gate>,
    outputs: Vec<BigInt>,
}

impl Circuit {
    pub fn new(hash_function: Option<Box<dyn HashFunction>>) -> Self {
        Self {
            hash_function,
            inputs: Vec::new(),
            gates: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: BigInt) -> usize {
        let index = self.inputs.len();
        self.inputs.push(input);
        index
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn add_output(&mut self, output: BigInt) {
        self.outputs.push(output);
    }

    pub fn apply_hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        self.hash_function
            .as_ref()
            .expect("Hash gate used but no hash function provided")
            .hash(a, b)
    }

    /// Retrieves an input value by index, if it exists
    pub fn get_input(&self, index: usize) -> Option<&BigInt> {
        self.inputs.get(index)
    }

    /// Generates the proof and checks if the constraints are met, in which case it's saved to a binary file
    pub fn generate_proof(&self, proof_file: &str) {
        let mut r1cs = R1CS::new();
        r1cs.variables = self
            .inputs
            .iter()
            .enumerate()
            .map(|(index, value)| Variable {
                index,
                value: value.clone(),
            })
            .collect(); //Every input is turned to variables in R1cs

        for gate in &self.gates {
            match gate {
                //Addition Gate
                Gate::Add(a, b, output) => {
                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*output].clone(), BigInt::from(1))],
                        Operation::Add,
                    );
                }

                //Multiplication gate
                Gate::Mul(a, b, output) => {
                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*output].clone(), BigInt::from(1))],
                        Operation::Mul,
                    );
                }

                //Hashing gate
                Gate::Hash(a, b, output) => {
                    let computed_hash = self.apply_hash(&self.inputs[*a], &self.inputs[*b]);
                    r1cs.variables[*output].value = computed_hash.clone();
                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*output].clone(), BigInt::from(1))],
                        Operation::Hash,
                    );

                    println!(
                        "Applying Hash constraint: input_a = {}, input_b = {}, computed_hash = {}, output_index = {}",
                        self.inputs[*a], self.inputs[*b], computed_hash, output
                    );
                }
            }
        }

        // Validate all constraints
        let is_valid = r1cs.is_satisfied(|a, b| {
            self.hash_function
                .as_ref()
                .expect("Hash gate used but no hash function provided")
                .hash(a, b)
        });

        // Save proof as bytes to binary file
        let proof_data = bincode::serialize(&is_valid).expect("Failed to serialize proof"); //The result is a Vec<u8> (vector of bytes)
        std::fs::write(proof_file, proof_data).expect("Failed to write proof file");

        if is_valid {
            println!("✓ Proof generated successfully: {}", proof_file);
        } else {
            println!("✗ Proof generation failed: constraints not satisfied");
        }
    }

    /// Verifies the proof by deserializing it from a binary file and checking if it's valid
    pub fn verify_proof(&self, proof_file: &str) -> bool {
        let proof_data = std::fs::read(proof_file).expect("Failed to read proof file");
        let is_valid = bincode::deserialize(&proof_data).expect("Failed to deserialize proof");
        is_valid
    }
}
