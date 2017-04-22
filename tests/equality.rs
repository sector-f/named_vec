extern crate named_vec;
use named_vec::*;

mod common;
use common::*;

#[test]
fn equality_with_two_items() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("foo", 1));
    first.push(NamedNumber::new("bar", 0));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 1));
    second.push(NamedNumber::new("bar", 0));

    assert_eq!(first, second);
}

#[test]
#[should_panic]
fn inequality_with_two_items() {
    let mut first = NamedVec::new();
    first.push(NamedNumber::new("foo", 1));
    first.push(NamedNumber::new("bar", 0));

    let mut second = NamedVec::new();
    second.push(NamedNumber::new("foo", 1));
    second.push(NamedNumber::new("quux", 0));

    assert_eq!(first, second);
}
