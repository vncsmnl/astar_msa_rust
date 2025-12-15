/*!
 * \author Daniel Sundfeld
 * \copyright MIT License
 *
 * \brief Main entry point for parallel A-Star MSA
 */

use clap::Parser;
use astar_msa_rust::{
    pastar,
    cost::Cost,
    heuristic_hpair::HeuristicHPair,
    msa_options::{PAStarOptions, PAStarOpt},
    read_fasta::read_fasta_file,
    sequences::Sequences,
    VERSION,
};

fn main() {
    let args = PAStarOptions::parse();
    
    println!("MSA PA-Star version {}", VERSION);
    println!("Input file: {}", args.input_file);
    
    // Set cost matrix
    if args.nucleotide {
        println!("Using nucleotide cost matrix");
        Cost::set_cost_nuc();
    } else {
        println!("Using PAM250 cost matrix");
        Cost::set_cost_pam250();
    }
    
    // Read FASTA file
    if let Err(e) = read_fasta_file(&args.input_file) {
        eprintln!("Error reading FASTA file: {}", e);
        std::process::exit(1);
    }
    
    let seq_num = Sequences::get_seq_num();
    println!("Number of sequences: {}", seq_num);
    
    if seq_num < 2 {
        eprintln!("Error: Need at least 2 sequences");
        std::process::exit(1);
    }
    
    // Print sequence information
    for i in 0..seq_num {
        println!("Sequence {}: {} (length: {})",
            i,
            Sequences::get_seq_name(i),
            Sequences::get_seq_len(i)
        );
    }
    
    // Initialize heuristic
    println!("\nPhase 1: Initializing heuristic...");
    HeuristicHPair::init();
    
    // Run PA-Star
    println!("\nPerforming search with Parallel A-Star ({})", VERSION);
    let options = PAStarOpt::from(args);
    
    match pastar::run_pastar_for_sequences(options) {
        Ok(()) => {
            println!("\nAlignment completed successfully!");
        }
        Err(e) => {
            eprintln!("Error during alignment: {}", e);
            std::process::exit(1);
        }
    }
}
