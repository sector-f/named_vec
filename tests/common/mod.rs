extern crate named_vec;
use named_vec::Named;

#[derive(Debug, PartialEq)]
pub struct NamedNumber {
    name: String,
    num: i32,
}

impl NamedNumber {
    pub fn new(name: &str, num: i32) -> Self {
        NamedNumber {
            name: name.to_owned(),
            num: num,
        }
    }

    #[allow(dead_code)]
    pub fn num(&self) -> i32 {
        self.num
    }
}

impl Named for NamedNumber {
    fn name(&self) -> &str {
        &self.name
    }
}

