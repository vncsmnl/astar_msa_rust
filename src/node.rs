/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Class that holds all Node attributes, like cost from origin,
 * heuristic estimate, parent
 */

use std::fmt;
use crate::coord::Coord;
use crate::cost::Cost;
use crate::sequences::Sequences;

#[derive(Clone, Debug)]
pub struct Node<const N: usize> {
    pub pos: Coord<N>,
    f: i32,  // priority (g + h)
    g: i32,  // exact cost from start
    parenti: i32,  // parent index
}

impl<const N: usize> Node<N> {
    pub fn new() -> Self {
        Node {
            pos: Coord::new(0),
            f: 0,
            g: 0,
            parenti: 0,
        }
    }

    pub fn with_values(g: i32, pos: Coord<N>, parenti: i32) -> Self {
        Node {
            pos,
            f: g,  // Will be updated with heuristic
            g,
            parenti,
        }
    }

    pub fn set_max(&mut self) {
        self.f = i32::MAX;
        self.g = i32::MAX;
    }

    pub fn get_g(&self) -> i32 {
        self.g
    }

    pub fn get_f(&self) -> i32 {
        self.f
    }

    pub fn get_h(&self) -> i32 {
        self.f - self.g
    }

    pub fn get_parenti(&self) -> i32 {
        self.parenti
    }

    pub fn get_parent(&self) -> Coord<N> {
        // parenti is a bitmap indicating which dimensions were incremented
        let mut parent_pos = self.pos;
        for dim in 0..N {
            if (self.parenti & (1 << dim)) != 0 {
                let val = parent_pos.get(dim);
                if val > 0 {
                    parent_pos.set(dim, val - 1);
                }
            }
        }
        parent_pos
    }

    pub fn set_f(&mut self, f: i32) {
        self.f = f;
    }

    pub fn set_g(&mut self, g: i32) {
        self.g = g;
    }

    pub fn set_parenti(&mut self, parenti: i32) {
        self.parenti = parenti;
    }

    /// Check if coordinate is within boundaries
    fn border_check(&self, c: &Coord<N>) -> bool {
        for i in 0..N {
            if c.get(i) > Sequences::get_seq_len(i) as u16 {
                return false;
            }
        }
        true
    }

    /// Calculate pairwise alignment cost
    fn pair_cost(&self, neigh_num: usize, s1: usize, s2: usize) -> i32 {
        let pos1 = self.pos.get(s1) as usize;
        let pos2 = self.pos.get(s2) as usize;

        // Check which dimension is being incremented
        let inc_s1 = (neigh_num & (1 << s1)) != 0;
        let inc_s2 = (neigh_num & (1 << s2)) != 0;

        match (inc_s1, inc_s2) {
            (true, true) => {
                // Both sequences advance - match/mismatch
                if pos1 < Sequences::get_seq_len(s1) && pos2 < Sequences::get_seq_len(s2) {
                    let c1 = Sequences::get_seq_char(s1, pos1);
                    let c2 = Sequences::get_seq_char(s2, pos2);
                    Cost::cost(c1, c2)
                } else {
                    i32::MAX
                }
            }
            (true, false) => {
                // Only s1 advances - gap in s2
                Cost::get_gap_cost()
            }
            (false, true) => {
                // Only s2 advances - gap in s1
                Cost::get_gap_cost()
            }
            (false, false) => {
                // Neither advances - gap in both (shouldn't happen in practice)
                Cost::get_gap_gap()
            }
        }
    }

    /// Get all valid neighbors of this node
    pub fn get_neighbors(&self) -> Vec<Node<N>> {
        let mut neighbors = Vec::new();
        
        // Generate all 2^N - 1 possible neighbors (excluding staying in place)
        for neigh_num in 1..(1 << N) {
            let mut new_pos = self.pos;
            let mut valid = true;

            // Create new coordinate by incrementing selected dimensions
            for dim in 0..N {
                if (neigh_num & (1 << dim)) != 0 {
                    new_pos.set(dim, new_pos.get(dim) + 1);
                }
            }

            // Check boundaries
            if !self.border_check(&new_pos) {
                continue;
            }

            // Calculate cost for this neighbor
            let mut cost = 0;
            
            // Sum costs for all sequence pairs
            for s1 in 0..N {
                for s2 in (s1 + 1)..N {
                    let pair_cost = self.pair_cost(neigh_num, s1, s2);
                    if pair_cost == i32::MAX {
                        valid = false;
                        break;
                    }
                    cost += pair_cost;
                }
                if !valid {
                    break;
                }
            }

            if valid {
                let new_g = self.g + cost;
                let new_node = Node::with_values(new_g, new_pos, neigh_num as i32);
                neighbors.push(new_node);
            }
        }

        neighbors
    }
}

impl<const N: usize> PartialEq for Node<N> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<const N: usize> Eq for Node<N> {}

impl<const N: usize> fmt::Display for Node<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node[pos: {}, f: {}, g: {}, h: {}]", 
               self.pos, self.f, self.g, self.get_h())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node: Node<3> = Node::new();
        assert_eq!(node.get_g(), 0);
        assert_eq!(node.get_f(), 0);
    }

    #[test]
    fn test_node_heuristic() {
        let mut node: Node<3> = Node::with_values(10, Coord::new(0), 0);
        node.set_f(25);
        assert_eq!(node.get_h(), 15);
    }
}
