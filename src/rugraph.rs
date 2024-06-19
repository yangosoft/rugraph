use std::fs::File;
use std::vec::Vec;

/// This trait is contains the basic behaviour of a `Graph`
pub trait IGraph<T> {
    /// Adds a new node `elem` to the graph
    fn add_node(&mut self, elem: T);
    /// Returns `true` if node `node` exists
    fn node_exists(&self, node: T) -> bool;
    /// Returns if a node `from` is connected to a node `to`
    fn is_connected(&self, from: T, to: T) -> bool;
    /// Returns if node `to` is a neighbord of `from`
    fn is_directly_connected(&self, from: T, to: T) -> bool;
    /// Returns an `String` with a dot file representation of the graph
    fn to_dot_string(&self, graph_name: &str) -> String;
    /// Exports the graph to a dot file. `file` must be a valid
    /// file ready to be written.
    /// `graph_name` is the name of the graph
    fn to_dot_file(&self, file: &mut File, graph_name: &str);
    /// Returns if a graph doesn't contain nodes
    fn is_empty(&self) -> bool;
    /// Returns how many nodes are in the graph
    fn count_nodes(&self) -> usize;
    /// Returns a vector of the elements
    fn get_nodes(&self) -> Vec<T>;
}

/// This trait is contains the basic behaviour of a `directed graph`
pub trait IDiGraph<T> {
    ///Creates a new edge from node `from` to node `to`
    ///nodes `from` and `to` must be previously added to the graph
    fn add_edge(&mut self, from: T, to: T);
    /// Returns a `Vec<Vec<T>>` containing all the simple paths
    /// from node `from` to node `to`
    fn all_simple_paths(&self, from: T, to: T) -> Vec<Vec<T>>;
    /// Returns a vector containing the `neighbors` of node `from`
    fn get_neighbors(&self, from: T) -> Vec<T>;
}

/// This trait is contains the basic behaviour of a `multi directed graph`
pub trait IMultiDiGraph<T, E> {
    ///Creates a new edge from node `from` to node `to`
    ///nodes `from` and `to` must be previously added to the graph
    fn add_edge(&mut self, from: T, to: T, edge: E);

    /// Returns if node `to` is a neighbord of `from` by edge `edge`
    fn is_directly_connected_by(&self, from: T, to: T, edge: E) -> bool;

    /// Returns a vector `Vec<Vec<(T, T, E)>>` containing all the simple paths
    /// from node `from` to node `to` in a vector of tuples `(from,to,edge)`
    fn all_simple_paths(&self, from: T, to: T) -> Vec<Vec<(T, T, E)>>;

    /// Returns a vector containing the `neighbors` of node `from`
    fn get_neighbors(&self, from: T) -> Vec<(T, E)>;
}
