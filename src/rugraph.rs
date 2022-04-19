use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::vec::Vec;
/// `Graph` is actually a `generic` directed graph where each node of type `T`
///  must implement: `T: Ord + Clone + std::fmt::Display + std::fmt::Debug`
pub struct Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    /// Nodes are stored in the heap
    nodes: RefCell<Vec<Rc<Node<T>>>>,
}
/// A `Node` is represented as a generic `T` and a list of pointers to their neighbors (allocated in the heap)
struct Node<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    elem: T,
    neighbors: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new(elem: T) -> Self {
        Node::<T> {
            elem: elem,
            neighbors: RefCell::new(Vec::new()),
        }
    }
}

impl<T> Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new() -> Self {
        Graph::<T> {
            nodes: RefCell::new(vec![]),
        }
    }

    /// Adds a new node `elem` to the graph
    pub fn add_node(&mut self, elem: T) {
        /*let ret_idx_from = self.get_index_by_node_id(elem.clone());

        let mut found = true;
        match ret_idx_from {
            Ok(_v) => {} ,
            Err(_e) => { found = false; }
        };

        if found {
            return;
        }*/

        if self.node_exists(elem.clone()) {
            return;
        }

        //TODO: exception if elem already in nodes
        //let idx = self.get_index_by_node_id(elem.clone());

        let mut nodes = self.nodes.borrow_mut();
        let n = Rc::new(Node::<T>::new(elem));

        //println!("Adding new node {}", n.elem);

        nodes.push(n);
        //println!("nodes length: {}", nodes.len());
    }

    ///Creates a new edge from node `from` to node `to`
    ///nodes `from` and `to` must be previously added to the graph
    pub fn add_edge(&mut self, from: T, to: T) {
        if !self.node_exists(from.clone()) || !self.node_exists(to.clone()) || self.is_directly_connected(from.clone(), to.clone()) {
            return;
        }

        let nodes = self.nodes.borrow_mut();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();
        let idx_to = nodes.iter().position(|r| r.elem == to).unwrap();

        let elem = nodes[idx_to].elem.clone();
        /*println!(
            "Index from {} -> {} index to {} -> {}",
            idx_from, from, idx_to, to
        );*/

       
        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();
        


        n.neighbors.borrow_mut().push(m);
    }

    /// Returns a vector containing the `neighbors` of node `from`
    pub fn get_neighbors(&self, from: T) -> Vec<T> {
        let mut neighbors = Vec::<T>::new();

        if !self.node_exists(from.clone()) {
            return neighbors;
        }

        let nodes = self.nodes.borrow();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();

        let n = &nodes[idx_from];

        //n.neighbors
        for e in n.neighbors.borrow().iter() {
            neighbors.push(e.elem.clone());
        }

        return neighbors;
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

    fn get_index_by_node_id(&self, from: T) -> Result<usize, &'static str> {
        let nodes = self.nodes.borrow();
        let idx_from = nodes.iter().position(|r| r.elem == from);
        match idx_from {
            None => Err("Element not found"),
            Some(value) => Ok(value),
        }
    }

    /// Returns if a node `from` is connected to a node `to`
    pub fn is_connected(&self, from: T, to: T) -> bool {
        //println!("Checking from {} to {}", from, to);
        let mut seen = Vec::<T>::new();
        let mut to_process = Vec::<T>::new();
        seen.push(from.clone());
        to_process.push(from.clone());

        let mut end = false;
        while !end {
            let node_id = to_process.pop().unwrap().clone();

            let neighbors = self.get_neighbors(node_id.clone());
            //println!("  |-> Node {} neighbors {:?}", node_id, neighbors);
            if neighbors.contains(&to) {
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
            if Rc::ptr_eq(e, &m) {
                //println!("Node {} is connected to {}", from, to);
                return true;
            }
        }
        //println!("Node {} is NOT connected to {}", from, to);
        return false;
    }

    /// Returns a `Vec<Vec<T>>` containing all the simple paths
    /// from node `from` to node `to`
    pub fn all_simple_paths(&self, from: T, to: T) -> Vec<Vec<T>> {
        let mut ret = Vec::<Vec<T>>::new();
        let mut current_path = Vec::<T>::new();
        let mut visited = Vec::<T>::new();

        self.dfs(from, to, &mut ret, &mut current_path, &mut visited);

        return ret;
    }

    /// Exports the graph to a dot file. `file` must be a valid
    /// file ready to be written.
    /// `graph_name` is the name of the graph
    pub fn to_dot_file(&self, file: &mut File, graph_name: &String) {
        let s = self.to_dot_string(graph_name.borrow());
        file.write_all(s.as_bytes()).expect("Error writing file!");
    }

    /// Returns an `String` with a dot file representation of the graph
    pub fn to_dot_string(&self, graph_name: &String) -> String {
        let mut s = String::from("digraph ") + graph_name + &String::from("{\n");
        let nodes = self.nodes.borrow();
        for n in nodes.iter() {
            s = s + &n.elem.to_string();
            for m in n.neighbors.borrow().iter() {
                s = s + &String::from(" -> ") + &m.elem.to_string();
            }

            s = s + &String::from(";\n");
        }
        s = s + &String::from("}\n");
        return s;
    }

    fn dfs(
        &self,
        from: T,
        to: T,
        simple_path: &mut Vec<Vec<T>>,
        current_path: &mut Vec<T>,
        visited: &mut Vec<T>,
    ) {
        if visited.contains(&from) {
            return;
        }
        visited.push(from.clone());
        current_path.push(from.clone());
        if from == to {
            simple_path.push(current_path.clone());
            if visited.contains(&from) {
                let index = visited.iter().position(|x| *x == from).unwrap();
                visited.remove(index);
                current_path.pop();
                return;
            }
        }

        let neighbors = self.get_neighbors(from.clone());
        for n in neighbors.iter() {
            self.dfs(n.clone(), to.clone(), simple_path, current_path, visited);
        }

        current_path.pop();
        if visited.contains(&from) {
            let index = visited.iter().position(|x| *x == from).unwrap();
            visited.remove(index);
        }
    }
}

impl<T> Drop for Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    fn drop(&mut self) {
        self.nodes.borrow_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use std::fs::File;
    #[test]
    fn it_works() {
        let mut graph = Graph::<i32>::new();
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
        let mut graph = Graph::<i32>::new();
        graph.add_node(1);
        graph.add_node(1);
        graph.add_node(1);
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
        graph.add_edge(1, 2);
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
        assert_eq!(
            paths,
            vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 9, 10, 5],
                vec![1, 2, 3, 7, 8, 5],
                vec![1, 5]
            ]
        );

        let mut fd = File::create("test2.dot").expect("error creating file");
        graph.to_dot_file(&mut fd, &String::from("paths_test"))
    }

    #[test]
    fn generics() {
        let mut graph = Graph::<String>::new();
        graph.add_node("a".to_string());
        graph.add_node("b".to_string());
        graph.add_node("c".to_string());
        graph.add_node("d".to_string());
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());
        graph.add_edge("c".to_string(), "d".to_string());
        graph.add_edge("a".to_string(), "d".to_string());

        let paths = graph.all_simple_paths("a".to_string(), "d".to_string());
        println!("{:?}", paths);

        assert_eq!(paths, vec![vec!["a", "b", "c", "d"], vec!["a", "d"]]);
    }

    #[test]
    fn to_dot() {
        let mut fd = File::create("test1.dot").expect("error creating file");
        let mut graph = Graph::<String>::new();
        graph.add_node("a".to_string());
        graph.add_node("b".to_string());
        graph.add_node("c".to_string());
        graph.add_node("d".to_string());
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());
        graph.add_edge("c".to_string(), "d".to_string());
        graph.add_edge("a".to_string(), "d".to_string());
        graph.to_dot_file(&mut fd, &String::from("to_dot_test"));
        let s = graph.to_dot_string(&String::from("to_dot_test"));
        println!("Dot:\n{}", s);
        assert_eq!(s.is_empty(), false);
    }
}
