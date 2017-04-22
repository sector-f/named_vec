use std::collections::hash_map::HashMap;

#[derive(Debug, PartialEq)]
pub struct NamedVec<T: Named> {
    map: HashMap<String, usize>,
    items: Vec<T>,
}

impl<T: Named> NamedVec<T> {
    pub fn new() -> Self {
        NamedVec {
            map: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        let name = item.name().to_owned();
        self.items.push(item);
        self.map.insert(name, self.items.len() - 1);
    }

    pub fn get<'a, A: 'a>(&self, lookup: A) -> Option<&T> where A: Into<Lookup<'a>> {
        // self.items.get(
        self.index_from_lookup(lookup.into()).and_then(|i| self.items.get(i))
    }

    // pub fn get_by_name(&self, name: &str) -> Option<&T> {
    //     match self.map.get(name) {
    //         Some(index) => { self.items.get(index.clone()) }
    //         None => { None },
    //     }
    // }

    // pub fn get_by_index(&self, index: usize) -> Option<&T> {
    //     self.items.get(index)
    // }

    pub fn swap<'a, 'b, A: 'a, B: 'b>(&mut self, first: A, second: B)
    where A: Into<Lookup<'a>> + Copy, B: Into<Lookup<'b>> + Copy {
        let old_i1 = self.index_from_lookup(first.into()).unwrap();
        let old_i2 = self.index_from_lookup(second.into()).unwrap();

        // Don't bother swapping (and allocating Strings!) if the two items are the same
        if old_i1 == old_i2 {
            return;
        }

        let old_s1 = self.name_from_lookup(first.into()).unwrap();
        let old_s2 = self.name_from_lookup(second.into()).unwrap();

        self.map.insert(old_s1, old_i2);
        self.map.insert(old_s2, old_i1);
        self.items.swap(old_i1, old_i2);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    fn index_from_lookup(&self, lookup: Lookup) -> Option<usize> {
        match lookup {
            Lookup::Name(name) => {
                self.map.get(name).cloned()
            },
            Lookup::Index(index) => {
                Some(index)
            },
        }
    }

    fn name_from_lookup(&self, lookup: Lookup) -> Option<String> {
        match lookup {
            Lookup::Name(name) => {
                Some(name.to_owned())
            },
            Lookup::Index(index) => {
                self.items.get(index).and_then(|s| Some(String::from(s.name())))
            },
        }
    }
}

pub enum Lookup<'a> {
    Name(&'a str),
    Index(usize),
}

impl<'a> From<&'a str> for Lookup<'a> {
    fn from(s: &'a str) -> Self {
        Lookup::Name(s)
    }
}

impl<'a> From<usize> for Lookup<'a> {
    fn from(i: usize) -> Self {
        Lookup::Index(i)
    }
}

pub trait Named {
    fn name(&self) -> &str;
}
