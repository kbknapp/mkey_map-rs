use std::collections::hash_map;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::slice;

// ! rustdoc

#[derive(Debug, Default, PartialEq)]
pub struct MKeyMap<'a, V: 'a>
where
    V: Eq + Hash,
{
    keys: HashMap<KeyType<'a>, usize>,
    value_index: Vec<V>,
    values: HashMap<u64, HashSet<usize>>,
    _phantomData: PhantomData<&'a V>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyType<'a> {
    Short([u8; 8]),
    Long(&'a [u8]),
    Position(usize),
}

impl<'a, V> MKeyMap<'a, V>
where
    V: Default + PartialEq + Debug + Eq + Hash,
{
    pub fn new() -> Self {
        MKeyMap::default()
    }
    //TODO ::from(x), ::with_capacity(n) etc
    //? set theory ops?

    pub fn insert(&mut self, key: KeyType<'a>, value: V) -> usize {
        let index;
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some((idx, _)) = self.values.get(&hash).and_then(|ids| {
            ids.iter()
                .map(|&x| (x, &self.value_index[x]))
                .find(|(i, x)| x == &&value)
        }) {
            index = idx;
        } else {
            self.value_index.push(value);
            index = self.value_index.len() - 1;
            self.values
                .entry(hash)
                .and_modify(|x| {
                    x.insert(index);
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(index);
                    set
                });
        }

        self.keys.insert(key, index);
        index
    }

    //TODO ::insert_many([x, y])

    // pub fn insert_key(&mut self, key: &'static str, index: usize) -> Result<(), ()> {
    //     if self.values.len() < index {
    //         return Err(());
    //     }
    //     let existing_key = self
    //         .keys
    //         .iter()
    //         .enumerate()
    //         .find(|(i, k)| k.0 == key)
    //         .map(|(i, k)| i);
    //     if let Some(existing_key) = existing_key {
    //         self.keys[existing_key].1 = index;
    //     } else {
    //         self.keys.push((key, index));
    //     }
    //     Ok(())
    // }
    pub fn insert_key(&mut self, key: KeyType<'a>, index: usize) {
        if index >= self.values.len() {
            panic!("Index out of bounds");
        }

        let idx = self.keys.insert(key, index);
    }
    //TODO ::insert_keyset([Long, Key2])

    pub fn get(&self, key: KeyType) -> Option<&V> {
        self.keys
            .get(&key)
            .and_then(|&idx| self.value_index.get(idx))
    }
    //TODO ::get_first([KeyA, KeyB])

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty() && self.values.is_empty()
    }

    //    pub fn remove(&mut self, key: &'static str) -> Option<Value> {
    //        let idx = self.keys.iter().find(|k| k.0 == key).map(|k| k.1);
    //        if let Some(idx) = idx {
    //            let value = self.values.swap_remove(idx);
    //            let indexs: Vec<usize> = self
    //                .keys
    //                .iter()
    //                .enumerate()
    //                .filter_map(|(i, kt)| if i == kt.1 { Some(i) } else { None })
    //                .collect::<Vec<_>>();
    //            for i in indexs {
    //                self.keys.remove(i);
    //            }
    //            Some(value)
    //        } else {
    //            None
    //        }
    //    }
    pub fn remove(&mut self, key: KeyType) -> Option<V> {
        unimplemented!()
    }
    //TODO ::remove_many([KeyA, KeyB])
    //? probably shouldn't add a possibility for removal?
    //? or remove by replacement by some dummy object, so the order is preserved

    // pub fn remove_key(&mut self, key: &'static str) {
    //     let existing_key = self
    //         .keys
    //         .iter()
    //         .enumerate()
    //         .find(|(i, k)| k.0 == key)
    //         .map(|(i, k)| i);

    //     if let Some(index) = existing_key {
    //         self.keys.swap_remove(index);
    //     }
    // }
    pub fn remove_key(&mut self, key: KeyType) {
        unimplemented!()
    }
    //TODO ::remove_keys([KeyA, KeyB])

    pub fn keys(&'a self) -> Keys<'a, usize> {
        Keys {
            iter: self.keys.keys(),
        }
    }

    pub fn values(&'a self) -> Values<'a, V> {
        Values {
            iter: self.value_index.iter(),
        }
    }
}

pub struct Keys<'a, V: 'a> {
    iter: hash_map::Keys<'a, KeyType<'a>, V>,
}

impl<'a,V> Iterator for Keys<'a, V> {
    type Item = &'a KeyType<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Values<'a, V: 'a> {
    iter: slice::Iter<'a, V>,
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use KeyType::*;
    use std::ffi::OsStr;


    #[test]
    fn get_some_value() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        {
            map.insert(Long("One"), "Value1");
        }
        assert_eq!(map.get(Long("One")), Some(&"Value1"));
    }

    #[test]
    fn get_none_value() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        map.insert(Long("One"), "Value1");
        assert_eq!(map.get(Long("Two")), None);
    }

    //    #[test]
    //    fn insert_delete_value() {
    //        let mut map = MKeyMap::new();
    //        map.insert("One", "Value1");
    //        assert_eq!(map.remove("One"), Some("Value1"));
    //        assert!(map.is_empty());
    //    }

    #[test]
    fn insert_duplicate_key() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        map.insert(Long(OsStr::new("One").as_bytes()), "Value1");
        assert_eq!(map.insert(Long(OsStr::new("One").as_bytes()), "Value2"), 1);
    }

    #[test]
    fn insert_duplicate_value() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        map.insert(Long("One"), "Value1");
        let orig_len = map.values.len();
        map.insert(Long("Two"), "Value1");
        assert_eq!(map.values.len(), orig_len);
        assert_eq!(map.get(Long("One")), map.get(Long("Two")));
    }

    //    #[test]
    //    fn insert_delete_none() {
    //        let mut map = MKeyMap::new();
    //        map.insert("One", "Value1");
    //        assert_eq!(map.remove("Two"), None);
    //        assert!(!map.is_empty());
    //        assert_eq!(map.get("One"), Some("Value1"));
    //    }

    #[test]
    fn insert_multiple_keys() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        let index = map.insert(Long("One"), "Value1");
        map.insert_key(Key2("Two"), index);
        assert_eq!(map.get(Long("One")), map.get(Key2("Two")));
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn remove_key() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        let index = map.insert(Long("One"), "Value1");
        map.insert_key(Key2("Two"), index);
        map.remove_key(Long("One"));
        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn iter_keys() {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        map.insert(Long("One"), "Value1");
        map.insert(Key2("Two"), "Value2");
        map.insert(Key3("Three"), "Value1");
        let mut iter = map.keys();
        let mut ground_truth = HashSet::new();
        ground_truth.insert(&Long("One"));
        ground_truth.insert(&Key2("Two"));
        ground_truth.insert(&Key3("Three"));
        assert_eq!(
            ground_truth.symmetric_difference(&iter.collect()).count(),
            0
        );
    }
}
*/
