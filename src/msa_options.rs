/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Command-line options for MSA A-Star and PA-Star
 */

use clap::Parser;
use crate::coord_hash::HashType;
use crate::HASH_SHIFT;

#[derive(Parser, Debug)]
#[command(author, version, about = "PA-Star: Parallel A-Star for Multiple Sequence Alignment", long_about = None)]
pub struct AStarOptions {
    /// Input FASTA file
    #[arg(value_name = "FILE")]
    pub input_file: String,

    /// Output FASTA file with alignment
    #[arg(short = 'f', long, value_name = "FILE")]
    pub output_file: Option<String>,

    /// Use nucleotide cost matrix (default: PAM250 for proteins)
    #[arg(short = 'n', long)]
    pub nucleotide: bool,

    /// Force quit after alignment (skip cleanup)
    #[arg(long, default_value_t = true)]
    pub force_quit: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "PA-Star: Parallel A-Star for Multiple Sequence Alignment", long_about = None)]
pub struct PAStarOptions {
    /// Input FASTA file
    #[arg(value_name = "FILE")]
    pub input_file: String,

    /// Output FASTA file with alignment
    #[arg(short = 'f', long, value_name = "FILE")]
    pub output_file: Option<String>,

    /// Use nucleotide cost matrix (default: PAM250 for proteins)
    #[arg(short = 'n', long)]
    pub nucleotide: bool,

    /// Number of threads to use (default: number of CPUs)
    #[arg(short = 't', long)]
    pub threads: Option<usize>,

    /// Hash type: fzorder, pzorder, fsum, psum
    #[arg(long, default_value = "fzorder")]
    pub hash_type: String,

    /// Hash shift value
    #[arg(long, default_value_t = HASH_SHIFT)]
    pub hash_shift: usize,

    /// Disable thread affinity
    #[arg(long)]
    pub no_affinity: bool,

    /// Thread affinity list (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub affinity: Option<Vec<usize>>,

    /// Hybrid CPU configuration: p-cores-num
    #[arg(long)]
    pub p_cores_num: Option<usize>,

    /// Hybrid CPU configuration: p-cores-size
    #[arg(long)]
    pub p_cores_size: Option<usize>,

    /// Hybrid CPU configuration: e-cores-num
    #[arg(long)]
    pub e_cores_num: Option<usize>,

    /// Hybrid CPU configuration: e-cores-size
    #[arg(long)]
    pub e_cores_size: Option<usize>,

    /// Force quit after alignment (skip cleanup)
    #[arg(long, default_value_t = true)]
    pub force_quit: bool,
}

pub struct AStarOpt {
    pub force_quit: bool,
    pub output_file: Option<String>,
}

pub struct PAStarOpt {
    pub common: AStarOpt,
    pub hash_type: HashType,
    pub hash_shift: usize,
    pub threads_num: usize,
    pub no_affinity: bool,
    pub thread_affinity: Vec<usize>,
    pub hybrid_conf: HybridCpu,
}

#[derive(Clone, Debug)]
pub struct HybridCpu {
    pub p_cores_num: usize,
    pub p_cores_size: usize,
    pub e_cores_num: usize,
    pub e_cores_size: usize,
}

impl Default for HybridCpu {
    fn default() -> Self {
        HybridCpu {
            p_cores_num: 0,
            p_cores_size: 0,
            e_cores_num: 0,
            e_cores_size: 0,
        }
    }
}

impl From<AStarOptions> for AStarOpt {
    fn from(opts: AStarOptions) -> Self {
        AStarOpt {
            force_quit: opts.force_quit,
            output_file: opts.output_file,
        }
    }
}

impl From<PAStarOptions> for PAStarOpt {
    fn from(opts: PAStarOptions) -> Self {
        let threads_num = opts.threads.unwrap_or_else(num_cpus::get);
        
        let hash_type = HashType::from_str(&opts.hash_type)
            .unwrap_or(HashType::FZorder);
        
        let hybrid_conf = HybridCpu {
            p_cores_num: opts.p_cores_num.unwrap_or(0),
            p_cores_size: opts.p_cores_size.unwrap_or(0),
            e_cores_num: opts.e_cores_num.unwrap_or(0),
            e_cores_size: opts.e_cores_size.unwrap_or(0),
        };
        
        let thread_affinity = opts.affinity.unwrap_or_default();
        
        PAStarOpt {
            common: AStarOpt {
                force_quit: opts.force_quit,
                output_file: opts.output_file,
            },
            hash_type,
            hash_shift: opts.hash_shift,
            threads_num,
            no_affinity: opts.no_affinity,
            thread_affinity,
            hybrid_conf,
        }
    }
}
