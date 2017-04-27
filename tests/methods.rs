#[macro_use]
extern crate named_vec;
use named_vec::*;

mod common;
use common::*;

#[test]
fn truncate() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("foo", 0));
    first.push(NamedNumber::new("bar", 1));
    first.push(NamedNumber::new("baz", 2));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 0));
    second.push(NamedNumber::new("bar", 1));

    first.truncate(2);

    assert_eq!(first, second);
}

#[test]
fn truncate_empty() {
    let mut named_vec: NamedVec<NamedNumber> = NamedVec::new();

    named_vec.truncate(2);
    named_vec.truncate(1);
    named_vec.truncate(0);
}

#[test]
fn insert() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("bar", 1));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 0));
    second.push(NamedNumber::new("bar", 1));

    first.insert(0, NamedNumber::new("foo", 0));

    assert_eq!(first, second);
}

#[test]
#[should_panic]
fn insert_invalid_index() {
    let mut named_vec: NamedVec<NamedNumber> = NamedVec::new();
    named_vec.push(NamedNumber::new("foo", 0));
    named_vec.push(NamedNumber::new("bar", 1));

    named_vec.insert(5, NamedNumber::new("baz", 10));
}

#[test]
fn remove_item() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("foo", 0));
    first.push(NamedNumber::new("bar", 1));
    first.push(NamedNumber::new("baz", 2));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 0));
    second.push(NamedNumber::new("bar", 1));
    second.push(NamedNumber::new("baz", 2));

    let first_removed = first.remove(0);
    let second_removed = second.remove("foo");

    assert_eq!(first_removed, second_removed);
    assert_eq!(first, second);
}

#[test]
fn into_iterator() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("foo", 0));
    first.push(NamedNumber::new("bar", 1));
    first.push(NamedNumber::new("baz", 2));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 0));
    second.push(NamedNumber::new("bar", 1));
    second.push(NamedNumber::new("baz", 2));

    let mut second_iter = second.into_iter();

    for item in first {
        assert_eq!(item, second_iter.next().unwrap());
    }
}

#[test]
fn named_vec_macro() {
    let named_vec = {
        named_vec![
            NamedNumber::new("foo", 0),
            NamedNumber::new("bar", 1),
            NamedNumber::new("baz", 2),
        ]
    };

    assert_eq!(named_vec[0], NamedNumber::new("foo", 0));
    assert_eq!(named_vec[1], NamedNumber::new("bar", 1));
    assert_eq!(named_vec[2], NamedNumber::new("baz", 2));
}

