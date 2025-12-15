# PA-Star2 - Rust Implementation

PA-Star is a software that performs a parallel A-Star search to solve the Multiple Sequence Alignment (MSA) problem. This is a complete Rust rewrite of the original C++ implementation.

## Features

- **Serial A-Star**: Classic A-Star algorithm for MSA
- **Parallel A-Star**: Multi-threaded implementation using work distribution
- **Hybrid CPU Support**: Optimized for asymmetric processors (Intel 12th-14th Gen)
- **Multiple Cost Matrices**: Support for PAM250 (proteins) and nucleotide scoring

## Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

## Installation

```bash
# Clone the repository
git clone https://github.com/vncsmnl/astar_msa_rust
cd astar_msa_rust

# Build in release mode (optimized)
make release
# or
cargo build --release

# Binaries will be copied to bin/
# Also available in target/release/
```

## Usage

### Using Compiled Binaries (Recommended)

After building with `make release`, use the binaries directly from `bin/`:

```bash
# Serial A-Star
./bin/msa_astar data/seqs/3/synthetic_easy.fasta

# Parallel A-Star
./bin/msa_pastar data/seqs/4/3pmg_ref1.fasta
```

### Using Cargo Run

#### Serial A-Star

```bash
# Basic usage
cargo run --release --bin msa_astar -- data/seqs/3/synthetic_easy.fasta

# With nucleotide scoring
cargo run --release --bin msa_astar -- -n data/seqs/NUC/EASY_instances/1.fasta

# Save output to file
cargo run --release --bin msa_astar -- -f output.fasta data/seqs/3/synthetic_easy.fasta
```

### Parallel A-Star

```bash
# Use all available cores
cargo run --release --bin msa_pastar -- data/seqs/4/3pmg_ref1.fasta

# Specify number of threads
cargo run --release --bin msa_pastar -- -t 4 data/seqs/4/3pmg_ref1.fasta

# With hash configuration
cargo run --release --bin msa_pastar -- --hash-type pzorder --hash-shift 10 data/seqs/5/EASY_instances/synthetic_easy.fasta

# Hybrid CPU configuration (Intel 12th Gen example: 8 P-cores, 8 E-cores)
cargo run --release --bin msa_pastar -- --p-cores-num 8 --p-cores-size 1 --e-cores-num 8 --e-cores-size 1 data/seqs/4/3pmg_ref1.fasta
```

## Command-Line Options

### Common Options

- `<FILE>`: Input FASTA file (required)
- `-f, --output-file <FILE>`: Output FASTA file with alignment
- `-n, --nucleotide`: Use nucleotide cost matrix (default: PAM250 for proteins)

### PA-Star Specific Options

- `-t, --threads <NUM>`: Number of threads (default: number of CPUs)
- `--hash-type <TYPE>`: Hash type: fzorder, pzorder, fsum, psum (default: fzorder)
- `--hash-shift <NUM>`: Hash shift value (default: 8)
- `--no-affinity`: Disable thread affinity
- `--affinity <LIST>`: Thread affinity list (comma-separated core IDs)
- `--p-cores-num <NUM>`: Number of P-cores (hybrid CPU)
- `--p-cores-size <NUM>`: Size of P-core groups (hybrid CPU)
- `--e-cores-num <NUM>`: Number of E-cores (hybrid CPU)
- `--e-cores-size <NUM>`: Size of E-core groups (hybrid CPU)

## Examples

```bash
# Easy test
cargo run --release --bin msa_astar -- data/seqs/Benchmark/1gpb_cutted.fasta

# Medium test with 2 threads
cargo run --release --bin msa_pastar -- -t 2 data/seqs/4/3pmg_ref1.fasta

# Nucleotide alignment
cargo run --release --bin msa_astar -- -n data/seqs/NUC/SARS-COV-2_2/all.fasta

# Save output
cargo run --release --bin msa_pastar -- -f aligned.fasta data/seqs/3/synthetic_veryeasy.fasta
```

## Performance

The Rust implementation provides:
- Memory safety without garbage collection
- Zero-cost abstractions
- Fearless concurrency with data race prevention
- Performance comparable to or better than C++

## Architecture

The project is organized into modules:

- `coord`: Multidimensional coordinates
- `node`: Search space nodes
- `cost`: Alignment cost matrices
- `sequences`: Sequence management
- `heuristic_hpair`: Pairwise alignment heuristic
- `astar`: Serial A-Star algorithm
- `pastar`: Parallel A-Star algorithm
- `priority_list`: Priority queue implementation
- `backtrace`: Alignment reconstruction

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_coord_creation
```

## Benchmarking

```bash
# Build with optimizations
cargo build --release

# Time an alignment
time ./target/release/msa_pastar data/seqs/5/EASY_instances/synthetic_easy.fasta
```

## License

MIT License - See LICENSE.txt

## Authors

- Original C++ version: Daniel Sundfeld
- Rust port: [Current maintainer]

## Citation

If you use PA-Star in your research, please cite:

[Original PA-Star paper citation]

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

## Differences from C++ Version

1. **Memory Safety**: Rust's ownership system eliminates many classes of bugs
2. **Concurrency**: Using Rayon for data parallelism instead of manual thread management
3. **Dependencies**: Using modern Rust crates instead of Boost
4. **Type Safety**: Const generics for compile-time sequence number checking
5. **Error Handling**: Using Result types instead of exceptions

## Troubleshooting

### Build Errors

If you encounter build errors, ensure:
- Rust version is 1.70 or later: `rustc --version`
- Dependencies are up to date: `cargo update`

### Performance Issues

For optimal performance:
- Always build in release mode: `--release`
- Adjust thread count for your CPU
- Use appropriate hash function and shift values

### Memory Issues

For large sequences:
- Monitor memory usage
- Adjust thread count if needed
- Consider using a machine with more RAM
