extern crate named_vec;
use named_vec::*;

mod common;
use common::*;

#[test]
#[should_panic]
fn swap_with_invalid_index() {
    let mut named_vec = NamedVec::new();
    named_vec.push(NamedNumber::new("bar", 0));
    named_vec.push(NamedNumber::new("foo", 1));

    named_vec.swap(0, 2);
}

#[test]
#[should_panic]
fn swap_with_invalid_name() {
    let mut named_vec = NamedVec::new();
    named_vec.push(NamedNumber::new("bar", 0));
    named_vec.push(NamedNumber::new("foo", 1));

    named_vec.swap("foo", "quux");
}

#[test]
fn swap_two_items_by_name() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("bar", 0));
    first.push(NamedNumber::new("foo", 1));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 1));
    second.push(NamedNumber::new("bar", 0));

    first.swap("foo", "bar");
    assert_eq!(first, second);
}

#[test]
fn swap_two_items_by_index() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("bar", 0));
    first.push(NamedNumber::new("foo", 1));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 1));
    second.push(NamedNumber::new("bar", 0));

    first.swap(0, 1);
    assert_eq!(first, second);
}

#[test]
fn swap_by_index_and_str() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("bar", 0));
    first.push(NamedNumber::new("foo", 1));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 1));
    second.push(NamedNumber::new("bar", 0));

    first.swap("bar", 1);
    assert_eq!(first, second);
}
