extern crate named_vec;
use named_vec::*;

#[derive(Debug, PartialEq)]
struct Item {
    name: String,
}

impl Item {
    fn new(n: &str) -> Self {
        Item {
            name: n.to_owned(),
        }
    }
}

impl Named for Item {
    fn name(&self) -> &str {
        &self.name
    }
}

#[test]
fn equality_with_two_items() {
    let mut first = NamedVec::new();
    first.push(Item::new("foo"));
    first.push(Item::new("bar"));

    let mut second = NamedVec::new();
    second.push(Item::new("foo"));
    second.push(Item::new("bar"));

    assert_eq!(first, second);
}

#[test]
#[should_panic]
fn equality_with_two_different_items() {
    let mut first = NamedVec::new();
    first.push(Item::new("foo"));
    first.push(Item::new("bar"));

    let mut second = NamedVec::new();
    second.push(Item::new("foo"));
    second.push(Item::new("quux"));

    assert_eq!(first, second);
}

#[test]
fn swap_two_items_by_index() {
    let mut first = NamedVec::new();
    first.push(Item::new("bar"));
    first.push(Item::new("foo"));

    let mut second = NamedVec::new();
    second.push(Item::new("foo"));
    second.push(Item::new("bar"));

    first.swap_by_index(0, 1);
    assert_eq!(first, second);
}

#[test]
#[should_panic]
fn swap_with_invalid_index() {
    let mut named_vec = NamedVec::new();
    named_vec.push(Item::new("bar"));
    named_vec.push(Item::new("foo"));

    named_vec.swap_by_index(0, 2);
}

#[test]
fn swap_two_items_by_name() {
    let mut first = NamedVec::new();
    first.push(Item::new("bar"));
    first.push(Item::new("foo"));

    let mut second = NamedVec::new();
    second.push(Item::new("foo"));
    second.push(Item::new("bar"));

    first.swap_by_name("foo", "bar");
    assert_eq!(first, second);
}

#[test]
#[should_panic]
fn swap_with_invalid_name() {
    let mut named_vec = NamedVec::new();
    named_vec.push(Item::new("bar"));
    named_vec.push(Item::new("foo"));

    named_vec.swap_by_name("foo", "quux");
}
