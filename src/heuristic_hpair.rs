/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Heuristic using all pairwise alignment scores
 */

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rayon::prelude::*;
use std::time::Instant;

use crate::coord::Coord;
use crate::pair_align::PairAlign;
use crate::sequences::Sequences;

static HEURISTIC: Lazy<RwLock<HeuristicData>> = Lazy::new(|| {
    RwLock::new(HeuristicData::new())
});

struct HeuristicData {
    aligns: Vec<PairAlign>,
}

impl HeuristicData {
    fn new() -> Self {
        HeuristicData {
            aligns: Vec::new(),
        }
    }
}

pub struct HeuristicHPair;

impl HeuristicHPair {
    pub fn init() {
        let start = Instant::now();
        let seq_num = Sequences::get_seq_num();
        
        println!("Starting pairwise alignments...");
        
        // Create list of pairs to align
        let mut pairs = Vec::new();
        for i in 0..seq_num - 1 {
            for j in i + 1..seq_num {
                pairs.push((i, j));
            }
        }
        
        // Parallel computation of all pairwise alignments
        let aligns: Vec<PairAlign> = pairs.par_iter()
            .map(|&(i, j)| {
                let s1 = Sequences::get_seq(i);
                let s2 = Sequences::get_seq(j);
                PairAlign::new((i, j), &s1, &s2)
            })
            .collect();
        
        let mut data = HEURISTIC.write();
        data.aligns = aligns;
        
        let duration = start.elapsed();
        println!("Pairwise alignments completed in {:.3}s", duration.as_secs_f64());
    }

    pub fn calculate_h<const N: usize>(c: &Coord<N>) -> i32 {
        let data = HEURISTIC.read();
        let mut h = 0;
        
        for align in &data.aligns {
            let (i, j) = align.get_pair();
            let pos_i = c.get(i) as usize;
            let pos_j = c.get(j) as usize;
            h += align.get_score(pos_i, pos_j);
        }
        
        h
    }

    pub fn destroy_instance() {
        let mut data = HEURISTIC.write();
        data.aligns.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cost::Cost;

    #[test]
    fn test_heuristic_init() {
        Cost::set_cost_nuc();
        Sequences::clear();
        Sequences::set_seq("ACGT".to_string()).unwrap();
        Sequences::set_seq("AGCT".to_string()).unwrap();
        Sequences::set_seq("ACCT".to_string()).unwrap();
        
        HeuristicHPair::init();
        
        let coord: Coord<3> = Coord::new(0);
        let h = HeuristicHPair::calculate_h(&coord);
        assert!(h >= 0);
    }
}
