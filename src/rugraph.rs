
use std::rc::Rc;
use std::vec::Vec;
use std::rc::Weak;
use std::cell::RefCell;

pub struct Graph {
    nodes: RefCell<Vec<Weak<Node>>>
}


struct Node {
    elem: i32,
    siblings: Vec<Rc<Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: RefCell::new(vec![]) }
    }

    pub fn add_node(&mut self, elem: i32)
    {
        let mut n = Rc::new(Node{elem: elem, siblings : Vec::new()});
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(Rc::downgrade(&n));
    }

    pub fn add_edge(&mut self, from: i32, to: i32 )
    {
        let mut nodes = self.nodes.borrow_mut();
        let idx_from = nodes.iter().position(|r| r.upgrade().unwrap().elem == from).unwrap();
        let idx_to = nodes.iter().position(|r| r.upgrade().unwrap().elem == from).unwrap();
        println!("Index from {} -> {} index to {} -> {}", idx_from,from, idx_to,to);

        //let mut n = &self.nodes[idx_from];
        //let m = self.nodes[idx_to].clone();
        //n.siblings.push(m);
    }

   
    /*pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }*/
}

impl Drop for Graph {
    fn drop(&mut self) {
        
    }
}


#[cfg(test)]
mod tests {
    use super::Graph;
    #[test]
    fn it_works() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_edge(1,2);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
