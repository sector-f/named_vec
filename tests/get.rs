extern crate named_vec;
use named_vec::*;

mod common;
use common::*;

#[test]
fn get_item_with_str() {
    let mut named_vec = NamedVec::new();
    named_vec.push(NamedNumber::new("bar", 1));
    named_vec.push(NamedNumber::new("foo", 0));

    assert_eq!(named_vec.get("bar").unwrap().num(), 1)
}

#[test]
fn get_item_with_index() {
    let mut named_vec = NamedVec::new();
    named_vec.push(NamedNumber::new("bar", 1));
    named_vec.push(NamedNumber::new("foo", 0));

    assert_eq!(named_vec.get(1).unwrap().num(), 0)
}

#[test]
fn index_trait() {
    let mut named_vec = NamedVec::new();
    named_vec.push(NamedNumber::new("bar", 1));
    named_vec.push(NamedNumber::new("foo", 0));

    assert_eq!(named_vec[1].num(), 0)
}

#[test]
#[should_panic]
fn invalid_index_trait() {
    let mut named_vec = NamedVec::new();
    named_vec.push(NamedNumber::new("bar", 1));
    named_vec.push(NamedNumber::new("foo", 0));

    let ref _number = named_vec[3];
}
