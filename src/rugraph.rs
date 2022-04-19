use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::thread::current;
use std::vec::Vec;

/// `Graph` is actually a directed graph where each node is an u32
pub struct Graph {
    /// Nodes are stored in the heap
    nodes: RefCell<Vec<Rc<Node>>>,
}
/// A `Node` is represented as an u32 and a list of pointers to their neighbors
struct Node {
    elem: u32,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: RefCell::new(vec![]),
        }
    }

    pub fn add_node(&mut self, elem: u32) {
        let mut n = Rc::new(Node {
            elem: elem,
            neighbors: RefCell::new(Vec::new()),
        });
        println!("Adding new node {}", n.elem);
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(n);
        println!("nodes length: {}", nodes.len());
    }

    pub fn add_edge(&mut self, from: u32, to: u32) {
        let nodes = self.nodes.borrow_mut();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();
        let idx_to = nodes.iter().position(|r| r.elem == to).unwrap();
        println!(
            "Index from {} -> {} index to {} -> {}",
            idx_from, from, idx_to, to
        );

        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();
        n.neighbors.borrow_mut().push(m);
    }

    /// Returns a vector containing the `neighbors` of node `from`
    pub fn get_neighbors(&self, from: u32) -> Vec<u32> {
        let nodes = self.nodes.borrow();
        let mut neighbors = Vec::<u32>::new();
        let idx_from = self.get_index_by_node_id(from);
        let n = &nodes[idx_from];

        //n.neighbors
        for e in n.neighbors.borrow().iter() {
            neighbors.push(e.elem);
        }

        return neighbors;
    }

    fn get_index_by_node_id(&self, from: u32) -> usize {
        let nodes = self.nodes.borrow();
        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();
        return idx_from;
    }

    /// Returns if a node `from` is connected to a node `to`
    pub fn is_connected(&self, from: u32, to: u32) -> bool {
        println!("Checking from {} to {}", from, to);
        let mut seen = Vec::<u32>::new();
        let mut to_process = Vec::<u32>::new();
        seen.push(from);
        to_process.push(from);

        let mut end = false;
        while !end {
            let node_id = to_process.pop().unwrap();

            let neighbors = self.get_neighbors(node_id);
            println!("  |-> Node {} neighbors {:?}", node_id, neighbors);
            if neighbors.contains(&to) {
                return true;
            } else {
                for n in neighbors.iter() {
                    if !seen.contains(n) {
                        to_process.push(*n);
                        seen.push(*n);
                    }
                }
            }

            end = to_process.is_empty();
        }

        return false;
    }

    pub fn is_directly_connected(&self, from: u32, to: u32) -> bool {
        let nodes = self.nodes.borrow();
        let idx_from = self.get_index_by_node_id(from);
        let idx_to = self.get_index_by_node_id(to);
        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();
        for e in n.neighbors.borrow().iter() {
            if Rc::ptr_eq(e, &m) {
                println!("Node {} is connected to {}", from, to);
                return true;
            }
        }
        println!("Node {} is NOT connected to {}", from, to);
        return false;
    }

    pub fn all_simple_paths(&self, from: u32, to: u32) -> Vec<Vec<u32>> {
        let mut ret = Vec::<Vec<u32>>::new();
        let mut current_path = Vec::<u32>::new();
        let mut visited = Vec::<u32>::new();

        self.dfs(from, to, &mut ret, &mut current_path, &mut visited);

        return ret;
    }

    fn dfs(
        &self,
        from: u32,
        to: u32,
        simple_path: &mut Vec<Vec<u32>>,
        current_path: &mut Vec<u32>,
        visited: &mut Vec<u32>,
    ) {
        if visited.contains(&from) {
            return;
        }
        visited.push(from);
        current_path.push(from);
        if from == to {
            simple_path.push(current_path.clone());
            if visited.contains(&from) {
                let index = visited.iter().position(|x| *x == from).unwrap();
                visited.remove(index);
                current_path.pop();
                return;
            }
        }

        let neighbors = self.get_neighbors(from);
        for n in neighbors.iter() {
            self.dfs(n.clone(), to, simple_path, current_path, visited);
        }

        current_path.pop();
        if visited.contains(&from) {
            let index = visited.iter().position(|x| *x == from).unwrap();
            visited.remove(index);
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Graph;
    #[test]
    fn it_works() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_node(6);
        graph.add_node(7);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        graph.add_edge(5, 7);

        let ret = graph.is_directly_connected(1, 2);
        assert_eq!(ret, true);

        let ret = graph.is_directly_connected(1, 3);
        assert_eq!(ret, false);

        let s = graph.get_neighbors(2);
        assert_eq!(s, [3, 4, 5]);

        let ret = graph.is_connected(1, 7);
        assert_eq!(ret, true);

        let ret = graph.is_connected(1, 6);
        assert_eq!(ret, false);
    }

    #[test]
    fn paths() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_node(6);
        graph.add_node(7);
        graph.add_node(8);
        graph.add_node(9);
        graph.add_node(10);
        graph.add_node(11);

        graph.add_edge(1, 2);
        graph.add_edge(1, 5);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(3, 9);
        graph.add_edge(9, 10);
        graph.add_edge(9, 11);

        graph.add_edge(4, 5);

        graph.add_edge(3, 7);
        graph.add_edge(7, 6);
        graph.add_edge(7, 8);

        graph.add_edge(8, 5);
        graph.add_edge(10, 5);

        let ret = graph.is_connected(1, 5);
        assert_eq!(ret, true);

        let paths = graph.all_simple_paths(1, 5);
        println!("{:?}", paths);
    }
}
