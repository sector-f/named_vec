use std::collections::hash_map::HashMap;
use std::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo};

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

    pub fn with_capacity(capacity: usize) -> Self {
        NamedVec {
            map: HashMap::with_capacity(capacity),
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        let name = item.name().to_owned();
        self.items.push(item);
        self.map.insert(name, self.items.len() - 1);
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
        self.map.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
        self.map.shrink_to_fit();
    }

    pub fn truncate(&mut self, len: usize) {
        let max = self.map.values().max().map(|n| n.clone()).unwrap();
        for item in self.items[len..max+1].iter() {
            let name = item.name();
            self.map.remove(name);
        }
        self.items.truncate(len);
    }

    pub fn get<'a, A: 'a>(&self, lookup: A) -> Option<&T> where A: Into<Lookup<'a>> {
        self.index_from_lookup(lookup.into()).and_then(|i| self.items.get(i))
    }

    pub fn get_mut <'a, A: 'a>(&mut self, lookup: A) -> Option<&mut T> where A: Into<Lookup<'a>> {
        self.index_from_lookup(lookup.into()).and_then(move |i| self.items.get_mut(i))
    }

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

    pub fn pop(&mut self) -> Option<T> {
        if self.items.len() == 0 {
            None
        } else {
            let last_item = self.items.pop().unwrap();
            self.map.remove(last_item.name());
            Some(last_item)
        }
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

///////////
// Index //
///////////

impl<'a, T: Named> Index<&'a str> for NamedVec<T> {
    type Output = T;

    fn index(&self, index: &str) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Named> Index<usize> for NamedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.items[index]
    }
}

impl<T: Named> Index<Range<usize>> for NamedVec<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &[T] {
        &self.items[index]
    }
}

impl<T: Named> Index<RangeTo<usize>> for NamedVec<T> {
    type Output = [T];

    fn index(&self, index: RangeTo<usize>) -> &[T] {
        &self.items[index]
    }
}

impl<T: Named> Index<RangeFrom<usize>> for NamedVec<T> {
    type Output = [T];

    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        &self.items[index]
    }
}

impl<T: Named> Index<RangeFull> for NamedVec<T> {
    type Output = [T];

    fn index(&self, _index: RangeFull) -> &[T] {
        &self.items
    }
}

//////////////
// IndexMut //
//////////////

impl<'a, T: Named> IndexMut<&'a str> for NamedVec<T> {
    fn index_mut(&mut self, index: &str) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: Named> IndexMut<usize> for NamedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.items[index]
    }
}

impl<T: Named> IndexMut<Range<usize>> for NamedVec<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        &mut self.items[index]
    }
}

impl<T: Named> IndexMut<RangeTo<usize>> for NamedVec<T> {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        &mut self.items[index]
    }
}

impl<T: Named> IndexMut<RangeFrom<usize>> for NamedVec<T> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        &mut self.items[index]
    }
}

impl<T: Named> IndexMut<RangeFull> for NamedVec<T> {
    fn index_mut(&mut self, _index: RangeFull) -> &mut [T] {
        &mut self.items
    }
}

pub trait Named {
    fn name(&self) -> &str;
}

//////////////////
// Lookup //
//////////////////

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

/////////////////
// MultiLookup //
/////////////////

// This won't be useful until std::slice::SliceIndex is stable
enum MultiLookup<'a> {
    Name(&'a str),
    Index(usize),
    Range(Range<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeTo(RangeTo<usize>),
    RangeFull(RangeFull),
}

impl<'a> From<&'a str> for MultiLookup<'a> {
    fn from(s: &'a str) -> Self {
        MultiLookup::Name(s)
    }
}

impl<'a> From<usize> for MultiLookup<'a> {
    fn from(i: usize) -> Self {
        MultiLookup::Index(i)
    }
}

impl<'a> From<Range<usize>> for MultiLookup<'a> {
    fn from(i: Range<usize>) -> Self {
        MultiLookup::Range(i)
    }
}

impl<'a> From<RangeFrom<usize>> for MultiLookup<'a> {
    fn from(i: RangeFrom<usize>) -> Self {
        MultiLookup::RangeFrom(i)
    }
}

impl<'a> From<RangeTo<usize>> for MultiLookup<'a> {
    fn from(i: RangeTo<usize>) -> Self {
        MultiLookup::RangeTo(i)
    }
}

impl<'a> From<RangeFull> for MultiLookup<'a> {
    fn from(i: RangeFull) -> Self {
        MultiLookup::RangeFull(i)
    }
}
