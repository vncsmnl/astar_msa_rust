/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Serial A-Star algorithm for multiple sequence alignment
 */

use ahash::AHashMap;
use crate::coord::Coord;
use crate::node::Node;
use crate::priority_list::PriorityList;
use crate::heuristic_hpair::HeuristicHPair;
use crate::sequences::Sequences;
use crate::time_counter::TimeCounter;
use crate::backtrace;
use crate::msa_options::AStarOpt;

pub fn a_star<const N: usize>(
    node_zero: Node<N>,
    coord_final: Coord<N>,
    options: &AStarOpt,
) -> Result<(), String> {
    let _timer = TimeCounter::new("\nPhase 2: A-Star running time:");
    
    let mut open_list = PriorityList::new();
    let mut closed_list: AHashMap<Coord<N>, Node<N>> = AHashMap::new();
    
    open_list.push(node_zero);
    
    let mut nodes_expanded = 0usize;
    let mut final_node: Option<Node<N>> = None;
    
    while !open_list.is_empty() {
        let current = match open_list.pop() {
            Some(node) => node,
            None => break,
        };
        
        // Check if better node already found
        if let Some(existing) = closed_list.get(&current.pos) {
            if current.get_g() >= existing.get_g() {
                continue;
            }
        }
        
        // Check if we reached the goal
        let is_final = current.pos == coord_final;
        closed_list.insert(current.pos, current.clone());
        
        if is_final {
            final_node = Some(current.clone());
            break;
        }
        
        nodes_expanded += 1;
        
        // Generate neighbors
        let neighbors = current.get_neighbors();
        
        for mut neighbor in neighbors {
            // Calculate heuristic
            let h = HeuristicHPair::calculate_h(&neighbor.pos);
            neighbor.set_f(neighbor.get_g() + h);
            
            // Check if already in closed list with better cost
            if let Some(existing) = closed_list.get(&neighbor.pos) {
                if neighbor.get_g() >= existing.get_g() {
                    continue;
                }
                closed_list.remove(&neighbor.pos);
            }
            
            open_list.push(neighbor);
        }
    }
    
    println!("Nodes expanded: {}", nodes_expanded);
    println!("Closed list size: {}", closed_list.len());
    
    match final_node {
        Some(node) => {
            backtrace::backtrace(&node, &closed_list, &options.output_file);
            Ok(())
        }
        None => Err("No solution found".to_string()),
    }
}

pub fn run_astar_for_sequences(options: &AStarOpt) -> Result<(), String> {
    match Sequences::get_seq_num() {
        2 => a_star::<2>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        3 => a_star::<3>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        4 => a_star::<4>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        5 => a_star::<5>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        6 => a_star::<6>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        7 => a_star::<7>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        8 => a_star::<8>(
            Sequences::get_initial_node(),
            Sequences::get_final_coord(),
            options
        ),
        n => Err(format!("Unsupported number of sequences: {}. Supported: 2-8", n)),
    }
}
