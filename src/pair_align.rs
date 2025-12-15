/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Pairwise sequence alignment using dynamic programming
 */

use crate::cost::Cost;
use std::cmp::min;

pub type Pair = (usize, usize);

pub struct PairAlign {
    pair: Pair,
    matrix: Vec<Vec<i32>>,
    s1_len: usize,
    s2_len: usize,
}

impl PairAlign {
    pub fn new(pair: Pair, s1: &[u8], s2: &[u8]) -> Self {
        let s1_len = s1.len();
        let s2_len = s2.len();
        
        let mut align = PairAlign {
            pair,
            matrix: vec![vec![0; s2_len + 1]; s1_len + 1],
            s1_len,
            s2_len,
        };
        
        align.align(s1, s2);
        align
    }

    fn align(&mut self, s1: &[u8], s2: &[u8]) {
        // Initialize borders
        self.matrix[self.s1_len][self.s2_len] = 0;
        
        // Fill last row
        for j in (0..self.s2_len).rev() {
            self.matrix[self.s1_len][j] = self.matrix[self.s1_len][j + 1] + Cost::get_gap_cost();
        }
        
        // Fill last column
        for i in (0..self.s1_len).rev() {
            self.matrix[i][self.s2_len] = self.matrix[i + 1][self.s2_len] + Cost::get_gap_cost();
        }
        
        // Fill the rest of the matrix
        for i in (0..self.s1_len).rev() {
            for j in (0..self.s2_len).rev() {
                self.pair_cost(i, j, s1, s2);
            }
        }
    }

    fn pair_cost(&mut self, i: usize, j: usize, s1: &[u8], s2: &[u8]) {
        let c0 = self.matrix[i + 1][j] + Cost::get_gap_cost();
        let c1 = self.matrix[i][j + 1] + Cost::get_gap_cost();
        let min_value = min(c0, c1);
        
        let c2 = self.matrix[i + 1][j + 1] + Cost::cost(s1[i], s2[j]);
        let min_value = min(c2, min_value);
        
        self.matrix[i][j] = min_value;
    }

    pub fn get_score(&self, i: usize, j: usize) -> i32 {
        if i <= self.s1_len && j <= self.s2_len {
            self.matrix[i][j]
        } else {
            0
        }
    }

    pub fn get_pair(&self) -> Pair {
        self.pair
    }

    pub fn get_final_score(&self) -> i32 {
        self.matrix[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cost::Cost;

    #[test]
    fn test_pair_align() {
        Cost::set_cost_nuc();
        let s1 = b"ACGT";
        let s2 = b"AGCT";
        let align = PairAlign::new((0, 1), s1, s2);
        
        // Score should be calculated
        assert!(align.get_final_score() >= 0);
    }
}
