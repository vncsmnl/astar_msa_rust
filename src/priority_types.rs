/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Priority list types and structures
 */

use std::cmp::Ordering;
use crate::node::Node;

#[derive(Clone)]
pub struct PriorityNode<const N: usize> {
    pub node: Node<N>,
}

impl<const N: usize> PriorityNode<N> {
    pub fn new(node: Node<N>) -> Self {
        PriorityNode { node }
    }
}

impl<const N: usize> PartialEq for PriorityNode<N> {
    fn eq(&self, other: &Self) -> bool {
        self.node.pos == other.node.pos
    }
}

impl<const N: usize> Eq for PriorityNode<N> {}

impl<const N: usize> PartialOrd for PriorityNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for PriorityNode<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (lower f value = higher priority)
        other.node.get_f().cmp(&self.node.get_f())
            .then_with(|| other.node.pos.cmp(&self.node.pos))
    }
}

impl<const N: usize> std::hash::Hash for PriorityNode<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.pos.hash(state);
    }
}
