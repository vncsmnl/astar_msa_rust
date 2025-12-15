/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 */

pub mod coord;
pub mod coord_hash;
pub mod cost;
pub mod node;
pub mod sequences;
pub mod read_fasta;
pub mod pair_align;
pub mod heuristic_hpair;
pub mod astar;
pub mod pastar;
pub mod backtrace;
pub mod time_counter;
pub mod msa_options;
pub mod priority_list;
pub mod priority_types;

pub const VERSION: &str = "2.0.0";

// Maximum number of sequences helper
pub const MAX_SEQUENCES: usize = 64;

// Hash shift default value
pub const HASH_SHIFT: usize = 12;

// Re-export commonly used types
pub use coord::Coord;
pub use cost::Cost;
pub use node::Node;
pub use sequences::Sequences;
