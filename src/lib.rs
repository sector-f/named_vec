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

    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        match self.map.get(name) {
            Some(index) => { self.items.get(index.clone()) }
            None => { None },
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn swap_by_index(&mut self, i1: usize, i2: usize) {
        let old_s1 = self.items[i1].name().to_owned();
        let old_s2 = self.items[i2].name().to_owned();

        let old_i1 = self.map[&old_s1];
        let old_i2 = self.map[&old_s2];

        self.map.insert(old_s1, old_i2);
        self.map.insert(old_s2, old_i1);
        self.items.swap(i1, i2);
    }

    pub fn swap_by_name(&mut self, s1: &str, s2: &str) {
        let old_i1 = self.map[s1];
        let old_i2 = self.map[s2];

        let old_s1 = self.items[old_i1].name().to_owned();
        let old_s2 = self.items[old_i2].name().to_owned();

        self.map.insert(old_s1, old_i2);
        self.map.insert(old_s2, old_i1);
        self.items.swap(old_i1, old_i2);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

pub trait Named {
    fn name(&self) -> &str;
}
