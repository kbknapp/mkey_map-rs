type Key = (&'static str, usize);
type Value = &'static str;

#[derive(Default)]
pub struct MKeyMap {
    keys: Vec<Key>,
    values: Vec<Value>
}

impl MKeyMap {
    pub fn new() -> Self {
        MKeyMap::default()
    }

    pub fn insert(&mut self, key: &'static str, value: Value) -> Result<(), ()> {
        if self.keys.iter().find(|k| k.0 == key).is_some() {
            return Err(());
        }
        let idx = self.values.iter().enumerate().find(|vt| vt.1 == &value).map(|vt| vt.0);
        if let Some(i) = idx {
            self.keys.push((key, i));
        } else {
            self.values.push(value);
            self.keys.push((key, self.values.len()-1));
        }
        Ok(())
    }

    pub fn get(&self, key: &'static str) -> Option<Value> {
        self.keys.iter().find(|k| k.0 == key).map(|k| self.values[k.1])
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty() && self.values.is_empty()
    }

    pub fn remove(&mut self, key: &'static str) -> Option<Value> {
        let idx = self.keys.iter().find(|k| k.0 == key).map(|k| k.1);
        if let Some(idx) = idx {
            let value = self.values.swap_remove(idx);
            let indexs: Vec<usize> = self.keys.iter().enumerate().filter_map(|(i, kt)| if i == kt.1 { Some(i) }else {None}).collect::<Vec<_>>();
            for i in indexs {
                self.keys.remove(i);
            }
            Some(value)
        } else {
            None
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

    #[test]
    fn insert_delete_value() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        assert_eq!(map.remove("One"), Some("Value1"));
        assert!(map.is_empty());
    }

    #[test]
    fn insert_duplicate_key() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        assert!(map.insert("One", "Value2").is_err());
    }

    #[test]
    fn insert_duplicate_value() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        let orig_len = map.values.len();
        map.insert("Two", "Value1");
        assert_eq!(map.values.len(), orig_len);
    }

    #[test]
    fn insert_delete_none() {
        let mut map = MKeyMap::new();
        map.insert("One", "Value1");
        assert_eq!(map.remove("Two"), None);
        assert!(!map.is_empty());
        assert_eq!(map.get("One"), Some("Value1"));
    }
}
