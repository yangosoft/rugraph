use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::vec::Vec;
/// `MultiDiGraph` is actually a `generic` multi directed graph where each node of type `T`
///  and edge of type `E`
///  must implement: `T: Ord + Clone + std::fmt::Display + std::fmt::Debug` and
///  `E: Ord + Clone + std::fmt::Display + std::fmt::Debug`
pub struct MultiDiGraph<T,E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    /// Nodes are stored in the heap
    nodes: RefCell<Vec<Rc<MultiNode<T,E>>>>,
}

/// A `Node` is represented as a generic `T` and a list of pointers to their neighbors (allocated in the heap)
struct MultiNode<T,E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    elem: T,
    neighbors: RefCell<Vec<Rc<Edge<T,E>>>>,
}

struct Edge<T,E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    node: Rc<MultiNode::<T,E>>,
    edge: E
}

impl<T,E> MultiNode<T,E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new(elem: T) -> Self {
        MultiNode::<T,E> {
            elem: elem,
            neighbors: RefCell::new(Vec::new()),
        }
    }
}

impl<T,E> MultiDiGraph<T,E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new() -> Self {
        MultiDiGraph::<T,E> {
            nodes: RefCell::new(vec![]),
        }
    }

    /// Adds a new node `elem` to the graph
    pub fn add_node(&mut self, elem: T) {
    
        if self.node_exists(elem.clone()) {
            return;
        }

        let mut nodes = self.nodes.borrow_mut();
        let n = Rc::new(MultiNode::<T,E>::new(elem));

        nodes.push(n);  
    }

    ///Creates a new edge from node `from` to node `to`
    ///nodes `from` and `to` must be previously added to the graph
    pub fn add_edge(&mut self, from: T, to: T, edge: E) {
        if !self.node_exists(from.clone()) || !self.node_exists(to.clone()) || self.is_directly_connected_by(from.clone(), to.clone(),edge.clone()) {
            return;
        }

        let nodes = self.nodes.borrow_mut();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();
        let idx_to = nodes.iter().position(|r| r.elem == to).unwrap();

        

       
        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();
        
        

        n.neighbors.borrow_mut().push(Rc::new(Edge{ node: m.clone(), edge: edge }));
    }

    pub fn node_exists(&self, from: T) -> bool {
        let nodes = self.nodes.borrow();
        let idx_from = nodes.iter().position(|r| r.elem == from);
        match idx_from {
            None => {
                return false;
            }
            Some(_value) => {
                return true;
            }
        }
    }

    /// Returns if node `to` is a neighbord of `from`
    pub fn is_directly_connected(&self, from: T, to: T) -> bool {
        let nodes = self.nodes.borrow();
        let ret_idx_from = self.get_index_by_node_id(from.clone());
        let idx_from;
        match ret_idx_from {
            Ok(v) => idx_from = v,
            Err(e) => {
                println!("Error {}", e);
                return false;
            }
        };

        let ret_idx_to = self.get_index_by_node_id(to.clone());
        let idx_to;
        match ret_idx_to {
            Ok(v) => idx_to = v,
            Err(e) => {
                println!("Error {}", e);
                return false;
            }
        };

        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();
        for e in n.neighbors.borrow().iter() {
            if Rc::ptr_eq(&e.node, &m) {
                //println!("Node {} is connected to {}", from, to);
                return true;
            }
        }
        //println!("Node {} is NOT connected to {}", from, to);
        return false;
    }


    /// Returns if node `to` is a neighbord of `from` by edge `edge`
    pub fn is_directly_connected_by(&self, from: T, to: T, edge: E) -> bool {
        let nodes = self.nodes.borrow();
        let ret_idx_from = self.get_index_by_node_id(from.clone());
        let idx_from;
        match ret_idx_from {
            Ok(v) => idx_from = v,
            Err(e) => {
                println!("Error {}", e);
                return false;
            }
        };

        let ret_idx_to = self.get_index_by_node_id(to.clone());
        let idx_to;
        match ret_idx_to {
            Ok(v) => idx_to = v,
            Err(e) => {
                println!("Error {}", e);
                return false;
            }
        };

        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();
        for e in n.neighbors.borrow().iter() {
            if Rc::ptr_eq(&e.node, &m) && (e.edge == edge) {
                //println!("Node {} is connected to {}", from, to);
                return true;
            }
        }
        //println!("Node {} is NOT connected to {}", from, to);
        return false;
    }

    fn get_index_by_node_id(&self, from: T) -> Result<usize, &'static str> {
        let nodes = self.nodes.borrow();
        let idx_from = nodes.iter().position(|r| r.elem == from);
        match idx_from {
            None => Err("Element not found"),
            Some(value) => Ok(value),
        }
    }


    pub fn get_neighbors(&self, from: T) -> Vec<(T,E)> {
        let mut neighbors = Vec::<(T,E)>::new();

        if !self.node_exists(from.clone()) {
            return neighbors;
        }

        let nodes = self.nodes.borrow();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();

        let n = &nodes[idx_from];

        //n.neighbors
        for e in n.neighbors.borrow().iter() {
            neighbors.push((e.node.elem.clone(),e.edge.clone()));
        }

        return neighbors;
    }

    /// Returns if a node `from` is connected to a node `to`
    pub fn is_connected(&self, from: T, to: T) -> bool {
        println!("Checking from {} to {}", from, to);
        let mut seen = Vec::<(T,E)>::new();
        let mut to_process = Vec::<(T,E)>::new();
        

        let neighbors = self.get_neighbors(from.clone());
        for n in neighbors.iter()
        {
            to_process.push(n.clone());
        }
        println!(" |-> Neighbors of {} : {:?}",from,neighbors);

        let mut end = false;
        while !end {
            let node = to_process.pop().unwrap().clone();
            let node_id = node.0;

            let neighbors = self.get_neighbors(node_id.clone());
            println!(" |-> Neighbors of {} : {:?}",node_id,neighbors);
            
            let contains = neighbors.iter().any(|r| r.0 == to.clone());
            println!("    |-> Neighbors of {} contains {}? {}",node_id,from,contains);


            if contains {
                return true;
            } else {
                for n in neighbors.iter() {
                    if !seen.contains(n) {
                        to_process.push(n.clone());
                        seen.push(n.clone());
                    }
                }
            }

            end = to_process.is_empty();
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::MultiDiGraph;
    use std::fs::File;
    #[test]
    fn multidigraph_test1() {
        let mut graph = MultiDiGraph::<i32,i32>::new();
        graph.add_node(1);

        let exists = graph.node_exists(1);
        assert_eq!(exists, true);
        let exists = graph.node_exists(99);
        assert_eq!(exists, false);

        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_node(6);
        graph.add_node(7);
        graph.add_edge(1, 2, 0);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 0);
        graph.add_edge(2, 4, 0);
        graph.add_edge(2, 5, 1);
        graph.add_edge(5, 7, 0);

        let ret = graph.is_directly_connected(1, 2);
        assert_eq!(ret, true);

        let ret = graph.is_directly_connected(1, 3);
        assert_eq!(ret, false);

        let s = graph.get_neighbors(2);
        assert_eq!(s, [(3,0), (4,0), (5,1)]);

        let ret = graph.is_connected(1, 7);
        assert_eq!(ret, true);

        let ret = graph.is_connected(1, 6);
        assert_eq!(ret, false);
    }
}

