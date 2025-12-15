/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief A multidimensional coordinate with fixed size
 */

use std::fmt;
use crate::coord_hash::HashType;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord<const N: usize> {
    coords: [u16; N],
}

impl<const N: usize> Coord<N> {
    pub fn new(init: u16) -> Self {
        Coord {
            coords: [init; N],
        }
    }

    pub fn from_array(coords: [u16; N]) -> Self {
        Coord { coords }
    }

    pub fn get(&self, index: usize) -> u16 {
        self.coords[index]
    }

    pub fn set(&mut self, index: usize, value: u16) {
        self.coords[index] = value;
    }

    /// Get neighbor coordinate by incrementing dimension n
    pub fn neigh(&self, n: usize) -> Self {
        let mut new_coord = *self;
        new_coord.coords[n] += 1;
        new_coord
    }

    /// Calculate sum of all coordinates
    pub fn get_sum(&self) -> u32 {
        self.coords.iter().map(|&x| x as u32).sum()
    }

    /// Calculate partial sum (for hash distribution)
    pub fn get_part_sum(&self) -> u32 {
        self.coords[1..].iter().map(|&x| x as u32).sum()
    }

    /// Calculate Z-order curve value (Morton code)
    pub fn get_z_order_curve(&self) -> u64 {
        let mut result = 0u64;
        for bit in 0..16 {
            for (dim, &coord) in self.coords.iter().enumerate() {
                if (coord >> bit) & 1 != 0 {
                    result |= 1u64 << (bit * N + dim);
                }
            }
        }
        result
    }

    /// Hash functions for thread distribution
    pub fn sum_hash(&self, size: usize, shift: usize) -> usize {
        ((self.get_sum() as usize) >> shift) % size
    }

    pub fn part_sum_hash(&self, size: usize, shift: usize) -> usize {
        ((self.get_part_sum() as usize) >> shift) % size
    }

    pub fn z_order_hash(&self, size: usize, shift: usize) -> usize {
        ((self.get_z_order_curve() as usize) >> shift) % size
    }

    pub fn part_z_order_hash(&self, size: usize, shift: usize) -> usize {
        let z = self.get_z_order_curve();
        // Remove first dimension bits
        let mut result = 0u64;
        let mut pos = 0;
        for bit_idx in 0..64 {
            if bit_idx % N != 0 {
                result |= ((z >> bit_idx) & 1) << pos;
                pos += 1;
            }
        }
        ((result as usize) >> shift) % size
    }

    /// Get thread ID based on hash type and thread map
    pub fn get_id(&self, size: usize, hash_type: HashType, shift: usize, thread_map: &[usize]) -> usize {
        let hash_value = match hash_type {
            HashType::FZorder => self.z_order_hash(size, shift),
            HashType::PZorder => self.part_z_order_hash(size, shift),
            HashType::FSum => self.sum_hash(size, shift),
            HashType::PSum => self.part_sum_hash(size, shift),
        };
        
        if thread_map.is_empty() {
            hash_value
        } else {
            thread_map[hash_value]
        }
    }
}

impl<const N: usize> PartialOrd for Coord<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Coord<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coords.cmp(&other.coords)
    }
}

impl<const N: usize> fmt::Display for Coord<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, coord) in self.coords.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", coord)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_creation() {
        let coord: Coord<3> = Coord::new(0);
        assert_eq!(coord.get(0), 0);
        assert_eq!(coord.get(1), 0);
        assert_eq!(coord.get(2), 0);
    }

    #[test]
    fn test_coord_neigh() {
        let coord: Coord<3> = Coord::new(5);
        let neigh = coord.neigh(1);
        assert_eq!(neigh.get(0), 5);
        assert_eq!(neigh.get(1), 6);
        assert_eq!(neigh.get(2), 5);
    }

    #[test]
    fn test_coord_sum() {
        let coord: Coord<3> = Coord::from_array([1, 2, 3]);
        assert_eq!(coord.get_sum(), 6);
    }
}
