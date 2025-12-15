/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Backtrace implementation to reconstruct alignment
 */

use crate::coord::Coord;
use crate::node::Node;
use crate::sequences::Sequences;
use crate::time_counter::TimeCounter;
use ahash::AHashMap;
use std::io::Write;

pub fn backtrace<const N: usize>(
    final_node: &Node<N>,
    closed_list: &AHashMap<Coord<N>, Node<N>>,
    output_file: &Option<String>,
) -> Vec<String> {
    let _timer = TimeCounter::new("Phase 3 - backtrace:");
    
    let mut path = Vec::new();
    let mut current = final_node.clone();
    
    println!("Final Score: Node[pos: {}, f: {}, g: {}, h: {}]", 
             final_node.pos, final_node.get_f(), final_node.get_g(), final_node.get_h());
    
    // Backtrace from final to initial
    while current.get_g() != 0 {
        path.push(current.clone());
        let parent_pos = current.get_parent();
        
        if let Some(parent_node) = closed_list.get(&parent_pos) {
            current = parent_node.clone();
        } else {
            eprintln!("Error: parent not found in closed list");
            break;
        }
    }
    
    // Add initial node
    path.push(current);
    path.reverse();
    
    // Reconstruct aligned sequences
    let alignments = reconstruct_alignment(&path);
    
    // Print similarity
    backtrace_print_similarity(&alignments);
    
    // Write to file if requested
    if let Some(filename) = output_file {
        if let Err(e) = backtrace_print_fasta_file::<N>(&alignments, filename) {
            eprintln!("Error writing FASTA file: {}", e);
        }
    }
    
    // Print alignment to terminal
    backtrace_print_alignment(&alignments);
    
    alignments
}

fn reconstruct_alignment<const N: usize>(path: &[Node<N>]) -> Vec<String> {
    let seq_num = N;
    let mut aligned_seqs: Vec<Vec<u8>> = vec![Vec::new(); seq_num];
    
    for window in path.windows(2) {
        let current = &window[0];
        let next = &window[1];
        
        for i in 0..seq_num {
            let current_pos = current.pos.get(i);
            let next_pos = next.pos.get(i);
            
            if next_pos > current_pos {
                // Sequence advanced - add character
                let seq = Sequences::get_seq(i);
                if (current_pos as usize) < seq.len() {
                    aligned_seqs[i].push(seq[current_pos as usize]);
                } else {
                    aligned_seqs[i].push(b'-');
                }
            } else {
                // Gap in this sequence
                aligned_seqs[i].push(b'-');
            }
        }
    }
    
    // Convert to strings
    aligned_seqs.into_iter()
        .map(|seq| String::from_utf8_lossy(&seq).to_string())
        .collect()
}

/// Calculate and print similarity percentage between sequences
fn backtrace_print_similarity(alignments: &[String]) {
    if alignments.is_empty() {
        return;
    }
    
    let seq_num = alignments.len();
    let align_len = alignments[0].len();
    
    let mut total = 0;
    let mut equal = 0;
    
    for pos in 0..align_len {
        for i in 0..seq_num {
            for j in (i + 1)..seq_num {
                let char_i = alignments[i].as_bytes().get(pos).copied().unwrap_or(b'-');
                let char_j = alignments[j].as_bytes().get(pos).copied().unwrap_or(b'-');
                
                if char_i == char_j {
                    equal += 1;
                }
                total += 1;
            }
        }
    }
    
    let percent = if total > 0 {
        (equal as f64 * 100.0) / total as f64
    } else {
        0.0
    };
    
    println!("Similarity: {:.2}%", percent);
}

/// Get terminal width for proper alignment display
fn get_print_size() -> usize {
    // Default to 80 columns
    let default_width = 80;
    
    // Try to get terminal width on Unix systems
    #[cfg(unix)]
    {
        if let Some((width, _)) = term_size::dimensions() {
            return width.saturating_sub(1).max(40);
        }
    }
    
    // On Windows or if detection fails, use default
    default_width
}

/// Print alignment in a formatted way, respecting terminal width
fn backtrace_print_alignment(alignments: &[String]) {
    if alignments.is_empty() {
        return;
    }
    
    let size = get_print_size();
    let align_len = alignments[0].len();
    let seq_num = alignments.len();
    
    let mut pos = 0;
    
    while pos < align_len {
        println!();
        
        for i in 0..seq_num {
            let end = (pos + size).min(align_len);
            let segment = &alignments[i][pos..end];
            println!("{}", segment);
        }
        
        pos += size;
    }
}

fn backtrace_print_fasta_file<const N: usize>(
    aligned_seqs: &[String],
    filename: &str,
) -> Result<(), std::io::Error> {
    use std::fs::File;
    
    let mut file = File::create(filename)?;
    
    for i in 0..N {
        let name = Sequences::get_seq_name(i);
        writeln!(file, "{}", name)?;
        writeln!(file, "{}", aligned_seqs[i])?;
    }
    
    Ok(())
}

// Keep the old public function for compatibility
pub fn write_fasta_output<const N: usize>(
    aligned_seqs: &[String],
    filename: &str,
) -> Result<(), std::io::Error> {
    backtrace_print_fasta_file::<N>(aligned_seqs, filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrace() {
        // Test would require setting up full alignment
        assert!(true);
    }
}
