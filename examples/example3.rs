use rugraph::graph::Graph;
use rugraph::rugraph::IDiGraph;
use rugraph::rugraph::IGraph;

fn main() {
    println!("Example of undirected graph\n\n");

    let mut graph = Graph::<String>::new();
    graph.add_node("a".to_string());
    graph.add_node("b".to_string());
    graph.add_node("c".to_string());
    graph.add_node("d".to_string());
    graph.add_edge("a".to_string(), "b".to_string());
    graph.add_edge("b".to_string(), "c".to_string());
    graph.add_edge("c".to_string(), "d".to_string());
    graph.add_edge("a".to_string(), "d".to_string());

    println!("Number of nodes:{}", graph.count_nodes());
}
