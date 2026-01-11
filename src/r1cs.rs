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
}
