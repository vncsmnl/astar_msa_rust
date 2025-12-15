/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Singleton that holds all sequences being aligned
 */

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use crate::coord::Coord;
use crate::node::Node;

static SEQUENCES: Lazy<RwLock<SequencesData>> = Lazy::new(|| {
    RwLock::new(SequencesData::new())
});

struct SequencesData {
    seqs: Vec<Vec<u8>>,
    seqs_name: Vec<String>,
    final_coord: Vec<usize>,
}

impl SequencesData {
    fn new() -> Self {
        SequencesData {
            seqs: Vec::new(),
            seqs_name: Vec::new(),
            final_coord: Vec::new(),
        }
    }
}

pub struct Sequences;

impl Sequences {
    pub fn set_seq(seq: String) -> Result<(), String> {
        let mut data = SEQUENCES.write();
        let seq_bytes: Vec<u8> = seq.into_bytes();
        let seq_len = seq_bytes.len();
        
        data.seqs.push(seq_bytes);
        data.final_coord.push(seq_len);
        
        Ok(())
    }

    pub fn set_name(name: String) {
        let mut data = SEQUENCES.write();
        data.seqs_name.push(name);
    }

    pub fn get_seq_num() -> usize {
        let data = SEQUENCES.read();
        data.seqs.len()
    }

    pub fn get_seq(index: usize) -> Vec<u8> {
        let data = SEQUENCES.read();
        data.seqs.get(index).cloned().unwrap_or_default()
    }

    pub fn get_seq_len(index: usize) -> usize {
        let data = SEQUENCES.read();
        data.seqs.get(index).map(|s| s.len()).unwrap_or(0)
    }

    pub fn get_seq_char(index: usize, pos: usize) -> u8 {
        let data = SEQUENCES.read();
        data.seqs.get(index)
            .and_then(|s| s.get(pos).copied())
            .unwrap_or(b' ')
    }

    pub fn get_seq_name(index: usize) -> String {
        let data = SEQUENCES.read();
        data.seqs_name.get(index).cloned().unwrap_or_default()
    }

    pub fn get_final_coord<const N: usize>() -> Coord<N> {
        let data = SEQUENCES.read();
        let mut coords = [0u16; N];
        for i in 0..N {
            coords[i] = data.final_coord.get(i).copied().unwrap_or(0) as u16;
        }
        Coord::from_array(coords)
    }

    pub fn get_initial_coord<const N: usize>() -> Coord<N> {
        Coord::new(0)
    }

    pub fn get_initial_node<const N: usize>() -> Node<N> {
        Node::with_values(0, Self::get_initial_coord(), 0)
    }

    pub fn is_final<const N: usize>(c: &Coord<N>) -> bool {
        let final_coord = Self::get_final_coord::<N>();
        c == &final_coord
    }

    pub fn clear() {
        let mut data = SEQUENCES.write();
        data.seqs.clear();
        data.seqs_name.clear();
        data.final_coord.clear();
    }

    pub fn destroy_instance() {
        Self::clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequences() {
        Sequences::clear();
        Sequences::set_seq("ACGT".to_string()).unwrap();
        Sequences::set_seq("AGCT".to_string()).unwrap();
        
        assert_eq!(Sequences::get_seq_num(), 2);
        assert_eq!(Sequences::get_seq_len(0), 4);
        assert_eq!(Sequences::get_seq_char(0, 0), b'A');
    }

    #[test]
    fn test_final_coord() {
        Sequences::clear();
        Sequences::set_seq("ACGT".to_string()).unwrap();
        Sequences::set_seq("AG".to_string()).unwrap();
        
        let final_coord: Coord<2> = Sequences::get_final_coord();
        assert_eq!(final_coord.get(0), 4);
        assert_eq!(final_coord.get(1), 2);
    }
}
