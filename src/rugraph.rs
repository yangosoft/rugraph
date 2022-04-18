use std::vec::Vec;

pub struct Graph {
    head: Link,
}


type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    siblings: Vec<Link>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { head: None }
    }

    pub fn add_node(elem: i32)
    {
        let new_node = Box::new(Node {elem: elem, siblings: Vec::new()});
        
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
