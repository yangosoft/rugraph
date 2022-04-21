use std::fs::File;
use std::vec::Vec;
use std::io::Write;
use crate::digraph::DiGraph;
use crate::rugraph::IDiGraph;
use crate::rugraph::IGraph;

/// `Graph` is a `generic` undirected graph where each node of type `T`
///  must implement: `T: Ord + Clone + std::fmt::Display + std::fmt::Debug`
pub struct Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    digraph: DiGraph<T>,
}

impl<T> Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn new() -> Self {
        Graph::<T> {
            digraph: DiGraph::<T>::new(),
        }
    }
}

impl<T> IGraph<T> for Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    fn add_node(&mut self, elem: T) {
        self.digraph.add_node(elem);
    }

    fn node_exists(&self, node: T) -> bool {
        return self.digraph.node_exists(node);
    }

    fn is_connected(&self, from: T, to: T) -> bool {
        return self.digraph.is_connected(from.clone(), to.clone())
            | self.digraph.is_connected(to, from);
    }

    fn is_directly_connected(&self, from: T, to: T) -> bool {
        return self.digraph.is_directly_connected(from.clone(), to.clone())
            | self.digraph.is_directly_connected(to.clone(), from.clone());
    }

    /// TODO: not implemented yet
    fn to_dot_file(&self, file: &mut File, graph_name: &String) {
        let s = self.to_dot_string(&graph_name.clone());
        file.write_all(s.as_bytes()).expect("Error writing file!");
    }

    /// TODO: not implemented yet
    fn to_dot_string(&self, graph_name: &String) -> String {
        let mut s = self.digraph.to_dot_string(graph_name);
        s = s.replace("digraph","graph").replace("->", "--");
        //TODO detect a -- b .. b -- a cases
        return s;
    }

    fn is_empty(&self) -> bool {
        return self.digraph.is_empty();
    }

    fn count_nodes(&self) -> usize {
        return self.digraph.count_nodes();
    }

    fn get_nodes(&self) -> Vec<T> {
        return self.digraph.get_nodes();
    }
}

impl<T> IDiGraph<T> for Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    fn add_edge(&mut self, from: T, to: T) {
        self.digraph.add_edge(from.clone(), to.clone());
        self.digraph.add_edge(to, from);
    }

    fn all_simple_paths(&self, from: T, to: T) -> Vec<Vec<T>> {
        return self.digraph.all_simple_paths(from, to);
    }

    fn get_neighbors(&self, from: T) -> Vec<T> {
        return self.digraph.get_neighbors(from);
    }
}

impl<T> Drop for Graph<T>
where
    T: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    fn drop(&mut self) {}
}

/// Returns a directed string graph `Graph<String>` from a dot file content
pub fn graph_from_dot_string(content: &String) -> Result<Graph<String>, &'static str> {
    let mut graph = Graph::<String>::new();
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
    let v_c: Vec<&str> = c.split(';').collect();

    for line in v_c.iter() {
        let v_nodes: Vec<&str> = line.split("->").collect();
        let mut prev_node = String::new();
        for txt_node in v_nodes.iter() {
            let txt_n = txt_node.replace(";", "");
            let n = txt_n.trim().to_string();
            if !n.is_empty() {
                // println!("Adding node {}", n.clone());
                graph.add_node(n.clone());
            }
            if !prev_node.is_empty() {
                // println!("  |-> Edge {} to {}",prev_node, n);
                graph.add_edge(prev_node.clone(), n.clone());
            }
            prev_node = n.clone();
        }
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::Graph;
    //use crate::graph::graph_from_dot_string;
    use crate::rugraph::IDiGraph;
    use crate::rugraph::IGraph;
    use std::fs::File;
    #[test]
    fn graph_it_works() {
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
        assert_eq!(s, [1, 3, 4, 5]);

        let ret = graph.is_connected(1, 7);
        assert_eq!(ret, true);

        let ret = graph.is_connected(7, 1);
        assert_eq!(ret, true);

        let ret = graph.is_connected(1, 6);
        assert_eq!(ret, false);
    }

    #[test]
    fn graph_paths() {
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
        println!(
            "1 connected to 5?{} 5 to 1?{}",
            ret,
            graph.is_connected(5, 1)
        );
        assert_eq!(ret, true);

        /*let paths = graph.all_simple_paths(1, 5);
        println!("GPRA {:?}", paths);
        assert_eq!(
            paths,
            vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 9, 10, 5],
                vec![1, 2, 3, 7, 8, 5],
                vec![1, 5]
            ]
        );*/
    }

    #[test]
    fn graph_generics() {
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
    fn graph_to_dot() {
        let mut fd = File::create("test_undirected.dot").expect("error creating file");
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

    #[test]
    fn graph_from_dot_str() {
        /*let content =
            String::from("digraph from_dot_str{\na -> b -> d;\nb -> c;\nc -> d;\nd;\n};\n");

        let graph = match graph_from_dot_string(&content) {
            Ok(v) => v,
            Err(e) => {
                println!("Error {}", e);
                Graph::<String>::new()
            }
        };

        assert_eq!(graph.count_nodes(), 4);
        let s = graph.to_dot_string(&String::from("from_dot_str"));
        println!("{}", s);
        //assert_eq!(s,content);*/
    }
}
