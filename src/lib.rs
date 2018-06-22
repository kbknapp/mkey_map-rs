use std::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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
    value_index: Vec<&'a V>,
    values: HashSet<V>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyType<T1, T2, T3> {
    Key1(T1),
    Key2(T2),
    Key3(T3),
}

// macro_rules! existing_key {
//     ($key1:ident) => {
//         if  let Some(key1) = $key1.into() {
//             self
//                 .keys
//                 .iter()
//                 .enumerate()
//                 .find(|(i, k)| k.0 == key1)
//                 .map(|(i, k)| i)
//         } else {  None};
//     }
// }

// macro_rules! insert_key {
//     ($key: ident, $storage:expr, $index:ident) => {
//         if let Some(existing_key) = $key {
//                 $storage[existing_key].1 = $index;
//             } else {
//                 $storage.push((key, $index));
//             }
//     }
// }

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

    // pub fn insert(&mut self, key1: impl Into<Option<K1>>, key2: impl Into<Option<K2>>, key3: impl Into<Option<K3>>, value: V) -> usize {
    //     let idx = self
    //         .values
    //         .iter()
    //         .enumerate()
    //         .find(|vt| vt.1 == &value)
    //         .map(|vt| vt.0);


    //     let  existing_key1 = existing_key!(key1);
    //     let  existing_key2 = existing_key!(key2);
    //     let  existing_key3 = existing_key!(key3);

    //     if let Some(i) = idx {
    //         insert_key!(key1, self.keys1, i);
    //         insert_key!(key2, self.keys2, i);
    //         insert_key!(key3, self.keys3, i);
    //         return i;
    //     } else {
    //         self.values.push(value);
    //         let index = self.values.len() - 1;
    //         insert_key!(key1, self.keys1, index);
    //         insert_key!(key2, self.keys2, index);
    //         insert_key!(key3, self.keys3, index);
    //         return index;
    //     }
    // }
    pub fn insert(&mut self, key: KeyType<K1, K2, K3>, value: V) -> usize {        
        unimplemented!()
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

    // pub fn get(&self, key: &'static str) -> Option<Value> {
    //     self.keys
    //         .iter()
    //         .find(|k| k.0 == key)
    //         .map(|k| self.values[k.1])
    // }
    pub fn get(&self, key: KeyType<K1, K2, K3>) -> Option<&V> {
        unimplemented!()
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
        map.insert(Key1("One"), "Value1");
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
