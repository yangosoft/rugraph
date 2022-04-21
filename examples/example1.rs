use rugraph::rugraph::Graph;
use rugraph::rugraph::IGraph;
use rugraph::rugraph::IDiGraph;
use rugraph::rugraph::graph_from_dot_string;
use std::fs::File;

fn main() {
    println!("Example of dot file creation. Check test1.dot file.\nTo create a picture install graphivz.\n\n$ dot -Tpng example1.dot -o example1.png\n\n");

    let mut fd = File::create("example1.dot").expect("error creating file");
    let mut graph = Graph::<String>::new();
    graph.add_node("a".to_string());
    graph.add_node("b".to_string());
    graph.add_node("c".to_string());
    graph.add_node("d".to_string());
    graph.add_edge("a".to_string(), "b".to_string());
    graph.add_edge("b".to_string(), "c".to_string());
    graph.add_edge("c".to_string(), "d".to_string());
    graph.add_edge("a".to_string(), "d".to_string());
    println!("Number of nodes of graph {}", graph.count_nodes());

    graph.to_dot_file(&mut fd, &String::from("to_dot_test"));
    let s = graph.to_dot_string(&String::from("to_dot_test"));
    println!("File content:\n{}", s);

    println!("Creating a new graph from the file content.");

    let graph2 = match graph_from_dot_string(&s)
    {
        Err(e) => { println!("Error {e}"); return },
        Ok(v) => v
    };
    println!("Number of nodes of new graph {}", graph2.count_nodes());
}
