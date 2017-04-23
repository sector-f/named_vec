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
