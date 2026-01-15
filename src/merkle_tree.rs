use crate::field::FieldElement;
use crate::hash_functions::{HashFunction, PoseidonHash};

pub struct MerkleTree {
    pub leaves: Vec<FieldElement>,
    pub levels: Vec<Vec<FieldElement>>,
    pub root: FieldElement,
}

impl MerkleTree {
    pub fn new(leaves: Vec<FieldElement>) -> Self {
        let hasher = PoseidonHash::new();
        let mut levels = vec![leaves.clone()];
        let mut current_level = leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            // Should be even number of leaves for simplicity in this basic implementation
            // If odd, we could duplicate the last one, but let's assume even for now as per tutorial
            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    &current_level[i] // Duplicate if odd
                };

                let hash = hasher.hash(left, right);
                next_level.push(hash);
            }

            levels.push(next_level.clone());
            current_level = next_level;
        }

        let root = current_level[0].clone();

        Self {
            leaves,
            levels,
            root,
        }
    }

    pub fn get_root(&self) -> FieldElement {
        self.root.clone()
    }

    /// Returns the Merkle Path for a given leaf index.
    /// The path consists of the sibling nodes needed to recompute the root.
    pub fn get_proof(&self, mut index: usize) -> Vec<FieldElement> {
        let mut path = Vec::new();

        // Iterate through levels (excluding the root)
        for level in &self.levels[0..self.levels.len() - 1] {
            let encoded_sibling = if index % 2 == 0 {
                // We are left, sibling is right
                if index + 1 < level.len() {
                    level[index + 1].clone()
                } else {
                    level[index].clone() // Duplicate
                }
            } else {
                // We are right, sibling is left
                level[index - 1].clone()
            };

            path.push(encoded_sibling);
            index /= 2;
        }

        path
    }
}
