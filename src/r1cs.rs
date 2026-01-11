use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    pub index: usize,
    pub value: BigInt,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Add,
    Mul,
    Hash,
}

/// A Constraint represents the equation: `(Sum A) * (Sum B) = (Sum C)`
///
/// **Understanding the Tuple `(Variable, BigInt)`:**
/// This tuple represents a single term in an equation, like **"2x"**.
///
/// * **The `Variable` is "x"**: It identifies *which* number we are talking about.
/// * **The `BigInt` is "2"**: It is the **Multiplier** (scalar). It scales the variable.
///
/// **Example:**
/// If you want to represent `3x + 5y`, you would create a generic vector:
/// `vec![ (x, 3), (y, 5) ]`
#[derive(Serialize, Deserialize)]
pub struct Constraint {
    pub left: Vec<(Variable, BigInt)>,
    pub right: Vec<(Variable, BigInt)>,
    pub output: Vec<(Variable, BigInt)>,
    pub operation: Operation,
}

/// The R1CS (Rank-1 Constraint System) is the "World" of the proof.
///
/// It combines two things:
/// 1. **Logic (Relationships)**: The `constraints`. These are the rules that must hold true.
/// 2. **Memory (Assignments)**: The `variables`. These are the actual values for a specific run.
///
/// If `variables` satisfy all `constraints`, the proof is valid.
#[derive(Serialize, Deserialize)]
pub struct R1CS {
    pub variables: Vec<Variable>,     // The Data (Witness)
    pub constraints: Vec<Constraint>, // The Logic (Circuit itself)
}

impl R1CS {
    /// Creates a new, empty Constraint System.
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a new logic rule (Constraint) to the system.
    ///
    /// The Constraint says: `(Left * Right) = Output`
    /// This defines HOW the variables must relate to each other.
    pub fn add_constraint(
        &mut self,
        left: Vec<(Variable, BigInt)>,
        right: Vec<(Variable, BigInt)>,
        output: Vec<(Variable, BigInt)>,
        operation: Operation,
    ) {
        let constraint = Constraint {
            left,
            right,
            output,
            operation,
        };
        self.constraints.push(constraint);
    }

    /// Adds a known value to the witness memory.
    /// This is where we store the actual numbers (e.g., "x is 5").
    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    /// Verifies that all constraints in the R1CS are satisfied by the current variable assignments.
    ///
    /// For each constraint, this function:
    /// 1. Computes the weighted sum of the left, right, and output terms
    /// 2. Checks that the operation holds: `left OP right = output`
    ///    - For `Add`: `left + right = output`
    ///    - For `Mul`: `left * right = output`
    ///    - For `Hash`: `hash(left, right) = output`
    ///
    /// # Arguments
    /// * `hash_function` - A closure that computes the hash of two `BigInt` values
    ///
    /// # Returns
    /// * `true` if all constraints are satisfied
    /// * `false` if any constraint fails (prints which constraint type failed)
    pub fn is_satisfied<K>(&self, hash_function: K) -> bool
    where
        K: Fn(&BigInt, &BigInt) -> BigInt, // Closure to compute hash
    {
        for constraint in &self.constraints {
            let left_val: BigInt = constraint
                .left
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();
            let right_val: BigInt = constraint
                .right
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();
            let output_val: BigInt = constraint
                .output
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();

            match constraint.operation {
                Operation::Add => {
                    if left_val + right_val != output_val {
                        println!("Constraint Add failed");
                        return false;
                    }
                }
                Operation::Mul => {
                    if left_val * right_val != output_val {
                        println!("Constraint Mul failed");
                        return false;
                    }
                }
                Operation::Hash => {
                    let computed_hash = hash_function(&left_val, &right_val);
                    if computed_hash != output_val {
                        println!("Constraint Hash failed");
                        return false;
                    }
                }
            }
        }
        true
    }
}
