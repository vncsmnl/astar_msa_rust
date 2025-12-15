/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Parallel A-Star algorithm for multiple sequence alignment
 */

use ahash::AHashMap;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::coord::Coord;
use crate::node::Node;
use crate::priority_list::PriorityList;
use crate::heuristic_hpair::HeuristicHPair;
use crate::sequences::Sequences;
use crate::time_counter::TimeCounter;
use crate::backtrace;
use crate::msa_options::PAStarOpt;

pub struct PAStar<const N: usize> {
    options: PAStarOpt,
    open_lists: Vec<Mutex<PriorityList<N>>>,
    closed_lists: Vec<Mutex<AHashMap<Coord<N>, Node<N>>>>,
    thread_map: Vec<usize>,
    map_size: usize,
    final_node: Arc<Mutex<Option<Node<N>>>>,
    end_cond: Arc<AtomicBool>,
    nodes_processed: Vec<AtomicUsize>,
}

impl<const N: usize> PAStar<N> {
    pub fn new(node_zero: Node<N>, options: PAStarOpt) -> Self {
        let threads_num = options.threads_num;
        
        println!(
            "Running PA-Star with: {} threads, {:?} hash, {} shift",
            threads_num,
            options.hash_type,
            options.hash_shift
        );
        
        let mut open_lists = Vec::with_capacity(threads_num);
        let mut closed_lists = Vec::with_capacity(threads_num);
        let mut nodes_processed = Vec::with_capacity(threads_num);
        
        for _ in 0..threads_num {
            open_lists.push(Mutex::new(PriorityList::new()));
            closed_lists.push(Mutex::new(AHashMap::new()));
            nodes_processed.push(AtomicUsize::new(0));
        }
        
        // Add initial node to first thread
        open_lists[0].lock().push(node_zero);
        
        // Configure thread map for hybrid CPUs
        let (thread_map, map_size) = Self::configure_thread_map(&options);
        
        PAStar {
            options,
            open_lists,
            closed_lists,
            thread_map,
            map_size,
            final_node: Arc::new(Mutex::new(None)),
            end_cond: Arc::new(AtomicBool::new(false)),
            nodes_processed,
        }
    }
    
    fn configure_thread_map(options: &PAStarOpt) -> (Vec<usize>, usize) {
        let hybrid = &options.hybrid_conf;
        
        if hybrid.p_cores_num == 0 && hybrid.e_cores_num == 0 {
            // No hybrid configuration - simple 1:1 mapping
            return (Vec::new(), options.threads_num);
        }
        
        let map_size = hybrid.p_cores_num * hybrid.p_cores_size +
                       hybrid.e_cores_num * hybrid.e_cores_size;
        let mut thread_map = Vec::with_capacity(map_size);
        
        // Map P-cores
        for i in 0..hybrid.p_cores_num {
            for _ in 0..hybrid.p_cores_size {
                thread_map.push(i);
            }
        }
        
        // Map E-cores
        for i in 0..hybrid.e_cores_num {
            for _ in 0..hybrid.e_cores_size {
                thread_map.push(i + hybrid.p_cores_num);
            }
        }
        
        (thread_map, map_size)
    }
    
    fn get_thread_id(&self, coord: &Coord<N>) -> usize {
        if self.thread_map.is_empty() {
            coord.get_id(
                self.options.threads_num,
                self.options.hash_type,
                self.options.hash_shift,
                &[]
            )
        } else {
            coord.get_id(
                self.map_size,
                self.options.hash_type,
                self.options.hash_shift,
                &self.thread_map
            )
        }
    }
    
    pub fn run(&self, coord_final: Coord<N>) -> Result<(), String> {
        let _timer = TimeCounter::new("\nPhase 2: PA-Star running time:");
        
        // Parallel execution
        (0..self.options.threads_num).into_par_iter().for_each(|tid| {
            self.worker(tid, coord_final);
        });
        
        // Get final node
        let final_node = self.final_node.lock().clone();
        
        match final_node {
            Some(node) => {
                // Print statistics
                let total_nodes: usize = self.nodes_processed
                    .iter()
                    .map(|n| n.load(Ordering::Relaxed))
                    .sum();
                
                println!("Total nodes processed: {}", total_nodes);
                
                // Merge closed lists for backtrace
                let mut merged_closed = AHashMap::new();
                for closed_list in &self.closed_lists {
                    let list = closed_list.lock();
                    for (coord, node) in list.iter() {
                        merged_closed.entry(*coord)
                            .and_modify(|e: &mut Node<N>| {
                                if node.get_g() < e.get_g() {
                                    *e = node.clone();
                                }
                            })
                            .or_insert_with(|| node.clone());
                    }
                }
                
                backtrace::backtrace(&node, &merged_closed, &self.options.common.output_file);
                Ok(())
            }
            None => Err("No solution found".to_string()),
        }
    }
    
    fn worker(&self, tid: usize, coord_final: Coord<N>) {
        // Set thread affinity if configured
        if !self.options.no_affinity && tid < self.options.thread_affinity.len() {
            let core_id = self.options.thread_affinity[tid];
            let _ = core_affinity::set_for_current(core_affinity::CoreId { id: core_id });
        }
        
        let mut empty_iterations = 0;
        const MAX_EMPTY_ITERATIONS: usize = 100;
        
        while !self.end_cond.load(Ordering::Relaxed) {
            // Try to dequeue a node
            let current = {
                let mut open_list = self.open_lists[tid].lock();
                open_list.pop()
            };
            
            let current = match current {
                Some(node) => {
                    empty_iterations = 0;
                    node
                },
                None => {
                    // No work available
                    empty_iterations += 1;
                    
                    if empty_iterations > MAX_EMPTY_ITERATIONS {
                        // Check if all lists are truly empty
                        if self.all_lists_empty() {
                            break;
                        }
                    }
                    
                    // Small delay to avoid busy waiting
                    std::thread::yield_now();
                    continue;
                }
            };
            
            // Check if already processed with better cost
            {
                let closed_list = self.closed_lists[tid].lock();
                if let Some(existing) = closed_list.get(&current.pos) {
                    if current.get_g() >= existing.get_g() {
                        continue;
                    }
                }
            }
            
            // Check if final
            if current.pos == coord_final {
                let mut final_node = self.final_node.lock();
                let should_update = match *final_node {
                    Some(ref existing) => current.get_g() < existing.get_g(),
                    None => true,
                };
                
                if should_update {
                    *final_node = Some(current.clone());
                    self.end_cond.store(true, Ordering::Relaxed);
                }
                continue;
            }
            
            // Add to closed list
            {
                let mut closed_list = self.closed_lists[tid].lock();
                closed_list.insert(current.pos, current.clone());
            }
            
            self.nodes_processed[tid].fetch_add(1, Ordering::Relaxed);
            
            // Generate neighbors
            let neighbors = current.get_neighbors();
            
            for mut neighbor in neighbors {
                // Calculate heuristic
                let h = HeuristicHPair::calculate_h(&neighbor.pos);
                neighbor.set_f(neighbor.get_g() + h);
                
                // Determine which thread should handle this node
                let target_tid = self.get_thread_id(&neighbor.pos);
                
                // Check if already in target's closed list
                let should_add = {
                    let closed_list = self.closed_lists[target_tid].lock();
                    if let Some(existing) = closed_list.get(&neighbor.pos) {
                        neighbor.get_g() < existing.get_g()
                    } else {
                        true
                    }
                };
                
                if should_add {
                    let mut open_list = self.open_lists[target_tid].lock();
                    open_list.push(neighbor);
                }
            }
        }
    }
    
    fn all_lists_empty(&self) -> bool {
        self.open_lists.iter().all(|list| list.lock().is_empty())
    }
}

pub fn run_pastar_for_sequences(options: PAStarOpt) -> Result<(), String> {
    match Sequences::get_seq_num() {
        2 => {
            let pastar = PAStar::<2>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        3 => {
            let pastar = PAStar::<3>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        4 => {
            let pastar = PAStar::<4>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        5 => {
            let pastar = PAStar::<5>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        6 => {
            let pastar = PAStar::<6>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        7 => {
            let pastar = PAStar::<7>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        8 => {
            let pastar = PAStar::<8>::new(
                Sequences::get_initial_node(),
                options
            );
            pastar.run(Sequences::get_final_coord())
        },
        n => Err(format!("Unsupported number of sequences: {}. Supported: 2-8", n)),
    }
}
