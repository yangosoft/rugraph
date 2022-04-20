use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::vec::Vec;

/// `MultiDiGraph` is actually a `generic` multi directed graph where each node of type `T`
///  and edge of type `E`
///  must implement: `T: Ord + Clone + std::fmt::Display + std::fmt::Debug` and
///  `E: Ord + Clone + std::fmt::Display + std::fmt::Debug`
pub struct MultiDiGraph<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    /// Nodes are stored in the heap
    nodes: RefCell<Vec<Rc<MultiNode<T, E>>>>,
}

/// A `Node` is represented as a generic `T` and a list of pointers to their neighbors (allocated in the heap)
struct MultiNode<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    elem: T,
    neighbors: RefCell<Vec<Rc<Edge<T, E>>>>,
}

struct Edge<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    node: Rc<MultiNode<T, E>>,
    edge: E,
}

impl<T, E> MultiNode<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new(elem: T) -> Self {
        MultiNode::<T, E> {
            elem: elem,
            neighbors: RefCell::new(Vec::new()),
        }
    }
}

impl<T, E> MultiDiGraph<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new() -> Self {
        MultiDiGraph::<T, E> {
            nodes: RefCell::new(vec![]),
        }
    }

    /// Adds a new node `elem` to the graph
    pub fn add_node(&mut self, elem: T) {
        if self.node_exists(elem.clone()) {
            return;
        }

        let mut nodes = self.nodes.borrow_mut();
        let n = Rc::new(MultiNode::<T, E>::new(elem));

        nodes.push(n);
    }

    ///Creates a new edge from node `from` to node `to`
    ///nodes `from` and `to` must be previously added to the graph
    pub fn add_edge(&mut self, from: T, to: T, edge: E) {
        if !self.node_exists(from.clone())
            || !self.node_exists(to.clone())
            || self.is_directly_connected_by(from.clone(), to.clone(), edge.clone())
        {
            return;
        }

        let nodes = self.nodes.borrow_mut();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();
        let idx_to = nodes.iter().position(|r| r.elem == to).unwrap();

        let n = &nodes[idx_from];
        let m = nodes[idx_to].clone();

        n.neighbors.borrow_mut().push(Rc::new(Edge {
            node: m.clone(),
            edge: edge,
        }));
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

    pub fn get_neighbors(&self, from: T) -> Vec<(T, E)> {
        let mut neighbors = Vec::<(T, E)>::new();

        if !self.node_exists(from.clone()) {
            return neighbors;
        }

        let nodes = self.nodes.borrow();

        let idx_from = nodes.iter().position(|r| r.elem == from).unwrap();

        let n = &nodes[idx_from];

        //n.neighbors
        for e in n.neighbors.borrow().iter() {
            neighbors.push((e.node.elem.clone(), e.edge.clone()));
        }

        return neighbors;
    }

    /// Returns if a node `from` is connected to a node `to`
    pub fn is_connected(&self, from: T, to: T) -> bool {
        //println!("Checking from {} to {}", from, to);
        let mut seen = Vec::<(T, E)>::new();
        let mut to_process = Vec::<(T, E)>::new();

        let neighbors = self.get_neighbors(from.clone());
        for n in neighbors.iter() {
            to_process.push(n.clone());
        }
        //println!(" |-> Neighbors of {} : {:?}",from,neighbors);

        let mut end = false;
        while !end {
            let node = to_process.pop().unwrap().clone();
            let node_id = node.0;

            let neighbors = self.get_neighbors(node_id.clone());
            //println!(" |-> Neighbors of {} : {:?}",node_id,neighbors);
            let contains = neighbors.iter().any(|r| r.0 == to.clone());
            //println!("    |-> Neighbors of {} contains {}? {}",node_id,from,contains);

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

    /// Returns a vector `Vec<Vec<(T, T, E)>>` containing all the simple paths
    /// from node `from` to node `to` in a vector of tuples `(from,to,edge)`
    pub fn all_simple_paths(&self, from: T, to: T) -> Vec<Vec<(T, T, E)>> {
        let mut ret = Vec::<Vec<(T, T, E)>>::new();
        let mut current_path = Vec::<(T, T, E)>::new();
        let mut visited = Vec::<(T, T, E)>::new();
        let neighbors = self.get_neighbors(from.clone());
        if neighbors.len() == 0 {
            return ret;
        }
        for n in neighbors.iter() {
            self.dfs(
                from.clone(),
                n.0.clone(),
                to.clone(),
                n.0.clone(),
                n.1.clone(),
                &mut ret,
                &mut current_path,
                &mut visited,
            );
        }
        return ret;
    }

    fn dfs(
        &self,
        previous_from: T,
        from: T,
        to: T,
        dst: T,
        edge: E,
        simple_path: &mut Vec<Vec<(T, T, E)>>,
        current_path: &mut Vec<(T, T, E)>,
        visited: &mut Vec<(T, T, E)>,
    ) {
        if visited.contains(&(from.clone(), dst.clone(), edge.clone())) {
            return;
        }
        visited.push((from.clone(), dst.clone(), edge.clone()));
        current_path.push((previous_from.clone(), dst.clone(), edge.clone()));
        if from == to {
            simple_path.push(current_path.clone());
            if visited.contains(&(from.clone(), dst.clone(), edge.clone())) {
                let index = visited
                    .iter()
                    .position(|x| {
                        x.0.clone() == from.clone()
                            && x.1.clone() == dst.clone()
                            && x.2.clone() == edge.clone()
                    })
                    .unwrap();
                visited.remove(index);
                current_path.pop();
                return;
            }
        }

        let neighbors = self.get_neighbors(dst.clone());
        for n in neighbors.iter() {
            self.dfs(
                dst.clone(),
                n.0.clone(),
                to.clone(),
                n.0.clone(),
                n.1.clone(),
                simple_path,
                current_path,
                visited,
            );
        }

        current_path.pop();
        if visited.contains(&(from.clone(), dst.clone(), edge.clone())) {
            let index = visited
                .iter()
                .position(|x| {
                    x.0.clone() == from.clone()
                        && x.1.clone() == dst.clone()
                        && x.2.clone() == edge.clone()
                })
                .unwrap();
            visited.remove(index);
        }
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
            for m in n.neighbors.borrow().iter() {
                s = s + &n.elem.to_string();
                s = s
                    + &String::from(" -> ")
                    + &m.node.elem.to_string()
                    + &String::from(" [label=\"")
                    + &m.edge.to_string()
                    + &String::from("\"];\n");
            }
        }
        s = s + &String::from("}\n");
        return s;
    }
}

impl<T, E> Drop for MultiDiGraph<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    fn drop(&mut self) {
        self.nodes.borrow_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::MultiDiGraph;
    #[test]
    fn multidigraph_test1() {
        let mut graph = MultiDiGraph::<i32, i32>::new();
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
        assert_eq!(s, [(3, 0), (4, 0), (5, 1)]);

        let ret = graph.is_connected(1, 7);
        assert_eq!(ret, true);

        let ret = graph.is_connected(1, 6);
        assert_eq!(ret, false);
    }

    #[test]
    fn multidigraph_generics() {
        let mut graph = MultiDiGraph::<String, String>::new();
        graph.add_node("a".to_string());
        graph.add_node("b".to_string());
        graph.add_node("c".to_string());
        graph.add_node("d".to_string());
        graph.add_edge("a".to_string(), "b".to_string(), "ab".to_string());
        graph.add_edge("b".to_string(), "c".to_string(), "bc".to_string());
        graph.add_edge("c".to_string(), "d".to_string(), "cd".to_string());
        graph.add_edge("a".to_string(), "d".to_string(), "ad".to_string());

        println!("From a to d");
        let paths = graph.all_simple_paths("a".to_string(), "d".to_string());
        println!("{:?}", paths);
        //
        assert_eq!(
            paths,
            vec![
                vec![
                    ("a".to_string(), "b".to_string(), "ab".to_string()),
                    ("b".to_string(), "c".to_string(), "bc".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string())
                ],
                vec![("a".to_string(), "d".to_string(), "ad".to_string())]
            ]
        );

        let s = graph.to_dot_string(&String::from("to_dot_multidigraph_test"));
        println!("Dot:\n{}", s);
        assert_eq!(s.is_empty(), false);
    }
}
