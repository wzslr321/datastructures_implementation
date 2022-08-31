#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};
    use crate::get_iterator;

    fn test_nodes(expect: Expect) {
        let actual: String = get_iterator().map(|node| format!("{:?}\n", node)).collect();
        expect.assert_eq(&actual)
    }

    #[test]
    fn test_linked_list() {
        test_nodes(
            expect![r#"
                Node { value: 1, next: None }
                Node { value: 2, next: None }
                Node { value: 3, next: None }
                "#]
        )
    }
}
