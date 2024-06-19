use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::vec::Vec;

use crate::rugraph::IGraph;
use crate::rugraph::IMultiDiGraph;

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

    fn get_index_by_node_id(&self, from: T) -> Result<usize, &'static str> {
        let nodes = self.nodes.borrow();
        let idx_from = nodes.iter().position(|r| r.elem == from);
        match idx_from {
            None => Err("Element not found"),
            Some(value) => Ok(value),
        }
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
        visited: &mut Vec<T>,
    ) {
        if visited.contains(&previous_from.clone()) {
            return;
        }
        visited.push(previous_from.clone());
        current_path.push((previous_from.clone(), dst.clone(), edge.clone()));
        if from == to {
            simple_path.push(current_path.clone());
            if visited.contains(&previous_from.clone()) {
                let index = visited
                    .iter()
                    .position(|x| x.clone() == previous_from.clone())
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
        if visited.contains(&previous_from.clone()) {
            let index = visited
                .iter()
                .position(|x| x.clone() == previous_from.clone())
                .unwrap();
            visited.remove(index);
        }
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

impl<T, E> IGraph<T> for MultiDiGraph<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    /// Adds a new node `elem` to the graph
    fn add_node(&mut self, elem: T) {
        if self.node_exists(elem.clone()) {
            return;
        }

        let mut nodes = self.nodes.borrow_mut();
        let n = Rc::new(MultiNode::<T, E>::new(elem));

        nodes.push(n);
    }

    fn node_exists(&self, from: T) -> bool {
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
    fn is_directly_connected(&self, from: T, to: T) -> bool {
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

    /// Returns if a node `from` is connected to a node `to`
    fn is_connected(&self, from: T, to: T) -> bool {
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

    /// Exports the graph to a dot file. `file` must be a valid
    /// file ready to be written.
    /// `graph_name` is the name of the graph
    fn to_dot_file(&self, file: &mut File, graph_name: &str) {
        let s = self.to_dot_string(graph_name);
        file.write_all(s.as_bytes()).expect("Error writing file!");
    }

    /// Returns an `String` with a dot file representation of the graph
    fn to_dot_string(&self, graph_name: &str) -> String {
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

    fn is_empty(&self) -> bool {
        return self.nodes.borrow().is_empty();
    }

    fn count_nodes(&self) -> usize {
        return self.nodes.borrow().len();
    }
    fn get_nodes(&self) -> Vec<T> {
        let mut ret = Vec::<T>::new();
        for n in self.nodes.borrow().iter() {
            ret.push(n.elem.clone());
        }
        return ret;
    }
}

/// Returns a multidirected string graph `MultiDiGraph<String, String>` from a dot file content
pub fn multidigraph_from_dot_string(
    content: &String,
) -> Result<MultiDiGraph<String, String>, &'static str> {
    let mut graph = MultiDiGraph::<String, String>::new();
    let idx1: usize;
    let idx2: usize;
    match content.chars().position(|c| c == '{') {
        None => {
            return Err("Dot file not correct. { not found.");
        }
        Some(i) => {
            idx1 = i + 1;
        }
    }

    match content.chars().position(|c| c == '}') {
        None => {
            return Err("Dot file not correct. } not found.");
        }
        Some(i) => {
            idx2 = i - 1;
        }
    }

    if idx2 < idx1 {
        return Err("Dot file not correct. } before {");
    }

    let c = &content[idx1..idx2];
    //println!("Content {}",c);
    let v_c: Vec<&str> = c.split(';').collect();

    for line in v_c.iter() {
        if line.is_empty() {
            continue;
        }
        //println!("Line {}", line);
        // [
        let idx3 = match line.chars().position(|c| c == '[') {
            None => {
                return Err("Dot file not correct. [ not found.");
            }
            Some(i) => i - 1,
        };

        let l_nodes = line[0..idx3].trim();

        let v_nodes: Vec<&str> = l_nodes.split("->").collect();

        let n_from;
        let n_to;
        if v_nodes.len() == 2 {
            n_from = v_nodes[0].trim().to_string();
            n_to = v_nodes[1].trim().to_string();
            graph.add_node(n_from.clone());
            graph.add_node(n_to.clone());
        } else {
            return Err("Dot file not correct.");
        }

        let label = line[idx3..]
            .replace("[label=\"", "")
            .replace("\"]", "")
            .trim()
            .to_string();
        //println!("LAbel {}",label.clone());
        graph.add_edge(n_from, n_to, label.to_string())
    }

    Ok(graph)
}

impl<T, E> IMultiDiGraph<T, E> for MultiDiGraph<T, E>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
    E: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    ///Creates a new edge from node `from` to node `to`
    ///nodes `from` and `to` must be previously added to the graph
    fn add_edge(&mut self, from: T, to: T, edge: E) {
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

    /// Returns if node `to` is a neighbord of `from` by edge `edge`
    fn is_directly_connected_by(&self, from: T, to: T, edge: E) -> bool {
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

    /// Returns a vector `Vec<Vec<(T, T, E)>>` containing all the simple paths
    /// from node `from` to node `to` in a vector of tuples `(from,to,edge)`
    fn all_simple_paths(&self, from: T, to: T) -> Vec<Vec<(T, T, E)>> {
        let mut ret = Vec::<Vec<(T, T, E)>>::new();
        let mut current_path = Vec::<(T, T, E)>::new();
        let mut visited = Vec::<T>::new();
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

    fn get_neighbors(&self, from: T) -> Vec<(T, E)> {
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
}

#[cfg(test)]
mod tests {
    use super::MultiDiGraph;
    use crate::multidigraph::multidigraph_from_dot_string;
    use crate::multidigraph::File;
    use crate::rugraph::IGraph;
    use crate::rugraph::IMultiDiGraph;

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

    #[test]
    fn multidigraph_from_dot_str() {
        let content =
            String::from("digraph multidigraph_from_dot_str{\na -> b [label=\"ab\"];\na -> d [label=\"ad\"];\nb -> c [label=\"bc\"];\nc -> d [label=\"cd\"];\n}");

        let graph = match multidigraph_from_dot_string(&content) {
            Ok(v) => v,
            Err(e) => {
                println!("Error {}", e);
                MultiDiGraph::<String, String>::new()
            }
        };

        assert_eq!(graph.count_nodes(), 4);
        let s = graph.to_dot_string(&String::from("multidigraph_from_dot_str"));
        println!("{}", s);
        //assert_eq!(s,content);
    }

    #[test]
    fn all_paths() {
        let mut graph = MultiDiGraph::<String, String>::new();
        graph.add_node("a".to_string());
        graph.add_node("b".to_string());
        graph.add_node("c".to_string());
        graph.add_node("d".to_string());
        graph.add_node("e".to_string());
        graph.add_node("f".to_string());

        graph.add_edge("a".to_string(), "b".to_string(), "ab0".to_string());
        graph.add_edge("a".to_string(), "b".to_string(), "ab1".to_string());
        graph.add_edge("b".to_string(), "c".to_string(), "bc0".to_string());
        graph.add_edge("b".to_string(), "c".to_string(), "bc1".to_string());
        graph.add_edge("c".to_string(), "d".to_string(), "cd".to_string());
        graph.add_edge("d".to_string(), "e".to_string(), "de".to_string());
        graph.add_edge("d".to_string(), "a".to_string(), "da".to_string());
        graph.add_edge("c".to_string(), "e".to_string(), "ce".to_string());
        graph.add_edge("e".to_string(), "f".to_string(), "ef".to_string());
        graph.add_edge("a".to_string(), "d".to_string(), "ad".to_string());

        println!("From a to d");
        let paths = graph.all_simple_paths("a".to_string(), "f".to_string());
        println!("Len {} {:?}", paths.len(), paths);

        assert_eq!(
            paths,
            vec![
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "d".to_string(), "ad".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ]
            ]
        );

        //
        /*assert_eq!(
            paths,
            vec![
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "d".to_string(), "cd".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "d".to_string(), "ad".to_string()),
                    ("d".to_string(), "e".to_string(), "de".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "d".to_string(), "ad".to_string()),
                    ("d".to_string(), "a".to_string(), "da".to_string()),
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "d".to_string(), "ad".to_string()),
                    ("d".to_string(), "a".to_string(), "da".to_string()),
                    ("a".to_string(), "b".to_string(), "ab0".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "d".to_string(), "ad".to_string()),
                    ("d".to_string(), "a".to_string(), "da".to_string()),
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc0".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ],
                vec![
                    ("a".to_string(), "d".to_string(), "ad".to_string()),
                    ("d".to_string(), "a".to_string(), "da".to_string()),
                    ("a".to_string(), "b".to_string(), "ab1".to_string()),
                    ("b".to_string(), "c".to_string(), "bc1".to_string()),
                    ("c".to_string(), "e".to_string(), "ce".to_string()),
                    ("e".to_string(), "f".to_string(), "ef".to_string())
                ]
            ]
        );*/

        println!("-----");
        for p in paths {
            println!("{:?}", p)
        }

        let s = graph.to_dot_string(&String::from("to_dot_multidigraph_test"));
        println!("Dot:\n{}", s);
        assert_eq!(s.is_empty(), false);
        let mut fd = File::create("test_multidirected.dot").expect("error creating file");
        graph.to_dot_file(&mut fd, &String::from("paths_test"));
    }
}
