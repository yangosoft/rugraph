use rugraph::rugraph::Graph;
use std::fs::File;


fn main() {

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
    graph.to_dot_file(&mut fd, &String::from("to_dot_test"))

    
}