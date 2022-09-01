#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};
    use crate::{scan_list, LinkedList, Node};

    fn test_list(src: &LinkedList, expect: Expect) {
        let actual: String = scan_list(src).map(|node| format!("{:?}\n", node)).collect();
        expect.assert_eq(&actual)
    }

    #[test]
    fn test_empty_list() {
        let list = LinkedList { head: None, tail: None };
        test_list(
            &list,
            expect![r#""#],
        )
    }

    #[test]
    fn test_list_with_single_node() {
        let list = LinkedList { head: Some(Box::new(Node::new(5))), tail: None };
        test_list(
            &list,
            expect![r#"
                 Node { value: 5, next: None }
            "#],
        )
    }

    #[test]
    fn test_list_with_two_nodes() {
        let mut list = LinkedList { head: None, tail: None };
        list.push_back(5);
        list.push_back(3);
        test_list(
            &list,
            expect![r#"
                 Node { value: 5, next: Node { value: 3, next: None } }
                 Node { value: 3, next: None }
            "#],
        )
    }
}
