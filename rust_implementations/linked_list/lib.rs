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

impl LinkedList {
    pub fn push_back(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));

        match &self.tail {
            Some(_) => {
                // swap(&mut Some(Box::new(new_node_2)), &mut self.tail);
                {
                    // tail.next = Some(new_node);
                }
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
    }
}

fn main() {
    let mut ll: LinkedList = LinkedList { head: None, tail: None };
    ll.push_back(5);
    let actual: String = scan_list(&ll).map(|node| format!("{:?}\n", node)).collect();
    println!("{:?}", actual);
}

fn scan_list(list: &LinkedList) -> impl Iterator<Item=Node> {
    let mut current_node = list.head.clone();

    std::iter::from_fn(move || {
        match current_node.clone() {
            None => {
                None
            }
            Some(node) => {
                current_node = node.clone().next;
                Some(node.as_ref().clone())
            }
        }
    })
}
