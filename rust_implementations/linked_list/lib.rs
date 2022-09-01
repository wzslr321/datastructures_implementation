mod tests;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
}


fn navigate(mut node: Option<&mut Box<Node>>, value: i32) {
    let ref mut n = node.unwrap();
    if n.clone().next != None {
        navigate(n.next.as_mut(), value)
    } else {
        n.next = Some(Box::new(Node::new(value)));
    }
}


impl LinkedList {
    pub fn push_back(&mut self, value: i32) {
        match self.head {
            Some(ref mut node) => {
                if node.next != None {
                    navigate(node.next.as_mut(), value)
                } else {
                    node.next = Some(Box::new(Node::new(value)));
                }
            }
            None => {
                self.head = Some(Box::new(Node::new(value)));
            }
        }
    }
}

fn main() {
    let mut ll: LinkedList = LinkedList { head: None };
    ll.push_back(5);
    ll.push_back(3);
    ll.push_back(2);
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
