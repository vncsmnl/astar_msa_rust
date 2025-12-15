/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Read FASTA format files
 */

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::sequences::Sequences;

pub fn read_fasta_file<P: AsRef<Path>>(filename: P) -> Result<(), String> {
    let file = File::open(&filename)
        .map_err(|e| format!("Can't open file {:?}: {}", filename.as_ref(), e))?;
    
    let reader = BufReader::new(file);
    let mut current_seq = String::new();
    
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading line: {}", e))?;
        let line = line.trim();
        
        if line.is_empty() {
            if !current_seq.is_empty() {
                let upper_seq = current_seq.to_uppercase();
                Sequences::set_seq(upper_seq)
                    .map_err(|e| format!("Error setting sequence: {}", e))?;
                current_seq.clear();
            }
            continue;
        }
        
        if line.starts_with('>') {
            // Save previous sequence if exists
            if !current_seq.is_empty() {
                let upper_seq = current_seq.to_uppercase();
                Sequences::set_seq(upper_seq)
                    .map_err(|e| format!("Error setting sequence: {}", e))?;
                current_seq.clear();
            }
            // Set sequence name
            Sequences::set_name(line.to_string());
        } else {
            // Append to current sequence
            current_seq.push_str(line);
        }
    }
    
    // Don't forget the last sequence
    if !current_seq.is_empty() {
        let upper_seq = current_seq.to_uppercase();
        Sequences::set_seq(upper_seq)
            .map_err(|e| format!("Error setting sequence: {}", e))?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fasta() {
        // This test would require a test FASTA file
        // For now, just ensure the function compiles
        assert!(true);
    }
}
