use std::fmt::Debug;

#[derive(Debug, Default, PartialEq)]
pub struct MKeyMap<K1, K2, K3, V> {
    keys1: Vec<K1>,
    keys2: Vec<K2>,
    keys3: Vec<K3>,
    values: Vec<V>,
}

impl<K1, K2, K3, V> MKeyMap<K1, K2, K3, V>
where
    K1: Default + PartialEq + Debug,
    K2: Default + PartialEq + Debug,
    K3: Default + PartialEq + Debug,
    V: Default + PartialEq + Debug,
{
    pub fn new() -> Self {
        MKeyMap::default()
    }

    pub fn insert(&mut self, key1: impl Into<Option<K1>>, key2: impl Into<Option<K2>>, key3: impl Into<Option<K3>>, value: V) -> usize {
        let idx = self
            .values
            .iter()
            .enumerate()
            .find(|vt| vt.1 == &value)
            .map(|vt| vt.0);


        let  existing_key1 = existing_key!(key1);
        let  existing_key2 = existing_key!(key2);
        let  existing_key3 = existing_key!(key3);

        if let Some(i) = idx {
            insert_key!(key1, self.keys1, i);
            insert_key!(key2, self.keys2, i);
            insert_key!(key3, self.keys3, i);
            return i;
        } else {
            self.values.push(value);
            let index = self.values.len() - 1;
            insert_key!(key1, self.keys1, index);
            insert_key!(key2, self.keys2, index);
            insert_key!(key3, self.keys3, index);
            return index;
        }
    }

    pub fn insert_key(&mut self, key: &'static str, index: usize) -> Result<(), ()> {
        if self.values.len() < index {
            return Err(());
        }
        let existing_key = self
            .keys
            .iter()
            .enumerate()
            .find(|(i, k)| k.0 == key)
            .map(|(i, k)| i);
        if let Some(existing_key) = existing_key {
            self.keys[existing_key].1 = index;
        } else {
            self.keys.push((key, index));
        }
        Ok(())
    }

    pub fn get(&self, key: &'static str) -> Option<Value> {
        self.keys
            .iter()
            .find(|k| k.0 == key)
            .map(|k| self.values[k.1])
    }

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

    pub fn remove_key(&mut self, key: &'static str) {
        let existing_key = self
            .keys
            .iter()
            .enumerate()
            .find(|(i, k)| k.0 == key)
            .map(|(i, k)| i);

        if let Some(index) = existing_key {
            self.keys.swap_remove(index);
        }
    }

    //    pub fn iter_keys<'a>(&'a self) -> std::iter::Map<std::slice::Iter<'a, Key>, Fn(Key) -> &'static str > {
    //        self.keys.iter().map(|(k, i)| k)
    //    }

    //    pub fn iter_values(&self) -> impl Iterator {
    //        self.values.iter()
    //    }
}

macro_rules! existing_key {
    ($key1:ident) => {
        if  let Some(key1) = $key1.into() {
            self
                .keys
                .iter()
                .enumerate()
                .find(|(i, k)| k.0 == key1)
                .map(|(i, k)| i)
        } else {  None};
    }
}

macro_rules! insert_key {
    ($key: ident, $storage:expr, $index:ident) => {
        if let Some(existing_key) = $key {
                $storage[existing_key].1 = $index;
            } else {
                $storage.push((key, $index));
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_some_value() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        assert_eq!(map.get("One"), Some("Value1"));
    }

    #[test]
    fn get_none_value() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        assert_eq!(map.get("Two"), None);
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
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        assert_eq!(map.insert("One", "Value2"), 1);
    }

    #[test]
    fn insert_duplicate_value() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        let orig_len = map.values.len();
        map.insert("Two", "Value1");
        assert_eq!(map.values.len(), orig_len);
        assert_eq!(map.get("One"), map.get("Two"));
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
        let mut map = MKeyMap::new();
        let index = map.insert("One", "Value1");
        map.insert_key("Two", index);
        assert_eq!(map.get("One"), map.get("Two"));
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn remove_key() {
        let mut map = MKeyMap::new();
        let index = map.insert("One", "Value1");
        map.insert_key("Two", index);
        map.remove_key("One");
        assert_eq!(map.keys.len(), 1);
        assert_eq!(map.values.len(), 1);
    }

    //    #[test]
    //    fn iter_keys() {
    //        let mut map = MKeyMap::new();
    //        map.insert("One", "Value1");
    //        map.insert("Two", "Value2");
    //        map.insert("Three", "Value1");
    //        let mut iter = map. iter_keys();
    //        assert_eq!(iter.next(), Some("One"));
    //        assert_eq!(iter.next(), Some("Two"));
    //        assert_eq!(iter.next(), Some("Three"));
    //        assert_eq!(iter.next(), None);
    //    }
}
