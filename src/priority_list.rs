/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Priority list implementation using binary heap
 */

use std::collections::BinaryHeap;
use crate::node::Node;
use crate::priority_types::PriorityNode;

pub struct PriorityList<const N: usize> {
    heap: BinaryHeap<PriorityNode<N>>,
}

impl<const N: usize> PriorityList<N> {
    pub fn new() -> Self {
        PriorityList {
            heap: BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PriorityList {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, node: Node<N>) {
        self.heap.push(PriorityNode::new(node));
    }

    pub fn pop(&mut self) -> Option<Node<N>> {
        self.heap.pop().map(|pn| pn.node)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<const N: usize> Default for PriorityList<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Coord;

    #[test]
    fn test_priority_list() {
        let mut plist: PriorityList<3> = PriorityList::new();
        
        let mut node1 = Node::with_values(10, Coord::new(0), 0);
        node1.set_f(20);
        
        let mut node2 = Node::with_values(5, Coord::new(1), 0);
        node2.set_f(15);
        
        plist.push(node1);
        plist.push(node2);
        
        assert_eq!(plist.len(), 2);
        
        // Should pop node2 first (lower f value)
        let popped = plist.pop().unwrap();
        assert_eq!(popped.get_f(), 15);
    }
}
