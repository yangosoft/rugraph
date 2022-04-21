use rugraph::multidigraph::MultiDiGraph;

use rugraph::rugraph::IGraph;
use rugraph::rugraph::IMultiDiGraph;
use std::fs::File;

fn main() {
    println!("Example of dot file creation. Check example2.dot file.\nTo create a picture install graphivz.\n\n$ dot -Tpng example2.dot -o example2.png\n\n");

    let mut fd = File::create("example2.dot").expect("error creating file");
    let mut graph = MultiDiGraph::<String, String>::new();
    graph.add_node("a".to_string());
    graph.add_node("b".to_string());
    graph.add_node("c".to_string());
    graph.add_node("d".to_string());
    graph.add_edge("a".to_string(), "b".to_string(), "ab".to_string());
    graph.add_edge("b".to_string(), "c".to_string(), "bc".to_string());
    graph.add_edge("c".to_string(), "d".to_string(), "cd".to_string());
    graph.add_edge("a".to_string(), "d".to_string(), "ad".to_string());
    graph.to_dot_file(&mut fd, &String::from("to_dot_multidigraph_test"));
    let s = graph.to_dot_string(&String::from("to_dot_multidigraph_test"));
    println!("File content:\n{}", s);
}
