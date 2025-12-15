/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Hash type configuration for coordinate hashing
 */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HashType {
    FZorder,  // Full Z-order curve
    PZorder,  // Partial Z-order curve (skip first dimension)
    FSum,     // Full sum
    PSum,     // Partial sum (skip first dimension)
}

pub struct CoordHash;

impl CoordHash {
    pub fn get_hash_name(hash_type: HashType) -> &'static str {
        match hash_type {
            HashType::FZorder => "Full Z-order",
            HashType::PZorder => "Partial Z-order",
            HashType::FSum => "Full Sum",
            HashType::PSum => "Partial Sum",
        }
    }
}

impl HashType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fzorder" | "full-zorder" => Some(HashType::FZorder),
            "pzorder" | "partial-zorder" => Some(HashType::PZorder),
            "fsum" | "full-sum" => Some(HashType::FSum),
            "psum" | "partial-sum" => Some(HashType::PSum),
            _ => None,
        }
    }
}
