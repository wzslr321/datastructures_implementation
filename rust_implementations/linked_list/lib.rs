mod tests;

#[derive(Debug)]
pub struct Node<'a> {
    value: i32,
    next: Option<&'a Node<'a>>,
}

pub fn get_iterator() -> impl Iterator<Item=Node<'static>> + 'static {
    let mut naive_count = 0;
    std::iter::from_fn(move || {
        naive_count += 1;

        if naive_count < 4 {
            Some(Node { value: naive_count, next: None })
        } else {
            None
        }
    })
}

fn main() {}
