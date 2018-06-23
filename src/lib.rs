use std::fmt::Debug;
use std::collections::{hash_map::Entry::*, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;

// ! rustdoc

#[derive(Debug, Default, PartialEq)]
pub struct MKeyMap<'a, K1, K2, K3, V: 'a>
    where 
        K1: Eq + Hash, 
        K2: Eq + Hash,
        K3: Eq + Hash,
        V: Eq + Hash,
{
    keys: HashMap<KeyType<K1, K2, K3>, usize>,
    value_index: Vec<V>,
    values: HashMap<u64, usize>,
    _phantomData: PhantomData<&'a V>
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyType<T1, T2, T3> {
    Key1(T1),
    Key2(T2),
    Key3(T3),
}

impl<'a, K1, K2, K3, V> MKeyMap<'a, K1, K2, K3, V>
where
    K1: Default + PartialEq + Debug + Eq + Hash,
    K2: Default + PartialEq + Debug + Eq + Hash,
    K3: Default + PartialEq + Debug + Eq + Hash,
    V: Default + PartialEq + Debug + Eq + Hash,
{
    //type KT = KeyType<K1, K2, K3>;
    
    pub fn new() -> Self {
        MKeyMap::default()
    }
    //TODO ::from(x), ::with_capacity(n) etc
    //? set theory ops?

    pub fn insert(&mut self, key: KeyType<K1, K2, K3>, value: V) -> usize {        
        let index;
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(&idx) = self.values.get(&hash) {
            index = idx;      
        } else {
            self.value_index.push(value);
            index = self.value_index.len() - 1;
            self.values.insert(hash, index);
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
    pub fn insert_key(&mut self, key: KeyType<K1, K2, K3>, index: usize) -> Result<(), ()> {
        unimplemented!()
    }
    //TODO ::insert_keyset([Key1, Key2])

    pub fn get(&self, key: KeyType<K1, K2, K3>) -> Option<&V> {
        self.keys.get(&key).and_then(|&idx| self.value_index.get(idx))
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
    pub fn remove(&mut self, key: KeyType<K1, K2, K3>) -> Option<V> {
        unimplemented!()
    }
    //TODO ::remove_many([KeyA, KeyB])

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
    pub fn remove_key(&mut self, key: KeyType<K1, K2, K3>) {
        unimplemented!()
    }
    //TODO ::remove_keys([KeyA, KeyB])

    //    pub fn iter_keys<'a>(&'a self) -> std::iter::Map<std::slice::Iter<'a, Key>, Fn(Key) -> &'static str > {
    //        self.keys.iter().map(|(k, i)| k)
    //    }
    pub fn iter_keys(&'a self) -> impl Iterator + 'a {
        self.keys.keys()
    }

    //    pub fn iter_values(&self) -> impl Iterator {
    //        self.values.iter()
    //    }
    pub fn iter_values(&'a self) -> impl Iterator + 'a {
        self.values.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use KeyType::*;

    #[test]
    fn get_some_value() {
        let mut map: MKeyMap<&str, &str, &str, &str> = MKeyMap::new();
        {map.insert(Key1("One"), "Value1");}
        assert_eq!(map.get(Key1("One")), Some(&"Value1"));
    }

    #[test]
    fn get_none_value() {
        let mut map: MKeyMap<&str, &str, &str, &str> = MKeyMap::new();
        map.insert(Key1("One"), "Value1");
        assert_eq!(map.get(Key1("Two")), None);
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
        let mut map: MKeyMap<&str, &str, &str, &str> = MKeyMap::new();
        map.insert(Key1("One"), "Value1");
        assert_eq!(map.insert(Key1("One"), "Value2"), 1);
    }

    #[test]
    fn insert_duplicate_value() {
        let mut map: MKeyMap<&str, &str, &str, &str> = MKeyMap::new();
        map.insert(Key1("One"), "Value1");
        let orig_len = map.values.len();
        map.insert(Key1("Two"), "Value1");
        assert_eq!(map.values.len(), orig_len);
        assert_eq!(map.get(Key1("One")), map.get(Key1("Two")));
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
        let mut map: MKeyMap<&str, &str, &str, &str> = MKeyMap::new();
        let index = map.insert(Key1("One"), "Value1");
        map.insert_key(Key2("Two"), index).unwrap();
        assert_eq!(map.get(Key1("One")), map.get(Key2("Two")));
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn remove_key() {
        let mut map: MKeyMap<&str, &str, &str, &str> = MKeyMap::new();
        let index = map.insert(Key1("One"), "Value1");
        map.insert_key(Key2("Two"), index).unwrap();
        map.remove_key(Key1("One"));
        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.values.len(), 1);
    }

    //    #[test]
    //    fn iter_keys() {
    //        let mut map = MKeyMap::new();
    //        map.insert(Key1("One"), "Value1");
    //        map.insert(Key2("Two"), "Value2");
    //        map.insert(Key3("Three"), "Value1");
    //        let mut iter = map. iter_keys();
    //        assert_eq!(iter.next().unwrap(), Key1("One"));
    //        assert_eq!(iter.next().unwrap(), Key2("Two"));
    //        assert_eq!(iter.next().unwrap(), Key3("Three"));
    //        assert_eq!(iter.next(), None);
    //    }
}
