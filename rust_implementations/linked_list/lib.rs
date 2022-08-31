mod tests;

use std::fmt;

#[derive(Debug, Clone)]
struct Node<'a> {
    value: i32,
    next: Option<&'a Node<'a>>,
}
impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Node<'_> {
    fn new(value: i32) -> Node<'static> {
        Node {
            value,
            next: None,
        }
    }
}

struct LinkedList<'a> {
    head: Option<Node<'a>>,
    tail: Option<Node<'a>>,
}

impl<'a> LinkedList<'a> {
    fn get_head(&self) -> &Option<Node<'_>> {
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

    pub fn push_back(&mut self, value: i32) {
        let new_node = Node::new(value);

        match &self.head {
            Some(node) => {
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
    }
}


fn main() {
    let mut ll: LinkedList<'_> = LinkedList { head: None, tail: None };
    ll.get_head();
    ll.push_back(5);
    ll.get_head();
}
