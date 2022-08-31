mod tests;

use std::fmt;
use std::mem::swap;

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}

impl<'a> LinkedList {
    fn get_head(&self) -> &Option<Box<Node>> {
        match &self.head {
            Some(node) => {
                println!("Head: {:?}", node);
            }
            None => {
                println!("Linked list is empty");
            }
        }
        &self.head
    }

    fn get_tail(&self) -> &Option<Box<Node>> {
        match &self.tail {
            Some(node) => {
                println!("Tail: {:?}", node);
            }
            None => {
                println!("There ain't no tail bruh");
            }
        }
        &self.tail
    }

    pub fn push_back(&mut self, value: i32) {
        let mut new_node = Node::new(value);

        match self.head {
            Some(_) => {
                swap(&mut Some(Box::new(new_node)), &mut self.tail);
            }
            None => {
                self.head = Some(Box::new(new_node.clone()));
                self.tail = Some(Box::new(new_node.clone()));
            }
        }
    }
}


fn main() {
    let mut ll: LinkedList = LinkedList { head: None, tail: None };
    ll.get_head();
    ll.push_back(5);
    ll.get_head();
    ll.push_back(3);
    ll.get_head();
    ll.get_tail();
}
