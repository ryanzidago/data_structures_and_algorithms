use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::mem;

const MAX_LOAD_FACTOR: usize = 1;
const INIT_N_BUCKETS: usize = 1;

#[derive(Debug, PartialEq)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            len: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.maybe_resize();
        let bucket_index = calculate_hash(&key, self.buckets.len());
        let bucket = &mut self.buckets[bucket_index];

        for &mut (ref existing_key, ref mut existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                return Some(mem::replace(existing_value, value));
            }
        }

        self.buckets[bucket_index].push((key, value));
        self.len += 1;
        None
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let bucket_index = calculate_hash(&key, self.buckets.len());
        let bucket = &self.buckets[bucket_index];

        for (ref existing_key, ref existing_value) in bucket.iter() {
            if &key == existing_key {
                return Some(existing_value);
            }
        }

        None
    }

    pub fn get_key_value(&self, key: K) -> Option<(&K, &V)> {
        let bucket_index = calculate_hash(&key, self.buckets.len());
        let bucket = &self.buckets[bucket_index];

        for (ref existing_key, ref existing_value) in bucket.iter() {
            if &key == existing_key {
                return Some((existing_key, existing_value));
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let bucket_index = calculate_hash(&key, self.buckets.len());
        for (ref existing_key, ref mut existing_value) in self.buckets[bucket_index].iter_mut() {
            if key == existing_key {
                return Some(existing_value);
            }
        }

        None
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            hashmap: &self,
            bucket_index: 0,
            at: 0,
        }
    }

    fn maybe_resize(&mut self) {
        if self.buckets.is_empty() || self.load_factor() > MAX_LOAD_FACTOR {
            let target_size = match self.buckets.len() {
                0 => INIT_N_BUCKETS,
                n_buckets => n_buckets * 2,
            };

            let mut resized_buckets: Vec<Vec<(K, V)>> = Vec::with_capacity(target_size);
            resized_buckets.extend((0..target_size).map(|_| Vec::new()));

            for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
                let hash = calculate_hash(&key, resized_buckets.len());
                let bucket_index = hash % resized_buckets.len();
                resized_buckets[bucket_index].push((key, value));
            }

            let _old_buckets = mem::replace(&mut self.buckets, resized_buckets);
        }
    }

    /// load_factor is the number of items N divided by the number of buckets M
    /// it represents how full the hashmap is
    fn load_factor(&self) -> usize {
        self.len / self.buckets.len()
    }
}

#[derive(Debug)]
pub struct Iter<'a, K, V> {
    hashmap: &'a HashMap<K, V>,
    bucket_index: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Debug,
    V: Debug,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.hashmap.buckets.get(self.bucket_index) {
                Some(bucket) => match bucket.get(self.at) {
                    Some((key, value)) => {
                        self.at += 1;
                        return Some((key, value));
                    }
                    None => {
                        self.bucket_index += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => return None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: Debug,
    V: Debug,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            hashmap: self,
            bucket_index: 0,
            at: 0,
        }
    }
}

fn calculate_hash<K: Hash + Eq>(key: &K, bucket_len: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash = hasher.finish();
    (hash % bucket_len as u64) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_test() {
        let hashmap: HashMap<&str, i32> = HashMap::new();
        assert_eq!(
            hashmap,
            HashMap {
                buckets: Vec::new(),
                len: 0,
            }
        );
    }
    #[test]
    fn insert_test() {
        // insert into an empty map
        let mut hashmap = HashMap::new();
        assert_eq!(hashmap.insert("Julie", 34), None);
        assert_eq!(hashmap.len(), 1);
        assert_eq!(hashmap.buckets.len(), 1);

        // insert replacing an existing key-value pair within the map
        // hashmap is not resized unless the MAX_LOAD_FACTOR is exceeded
        assert_eq!(hashmap.insert("Julie", 23), Some(34));
        assert_eq!(hashmap.len(), 1);
        assert_eq!(hashmap.buckets.len(), 1);

        // if the MAX_LOAD_FACTOR is exceeded, then the hashmap is resized
        hashmap.insert("Jérémy", 56);
        assert_eq!(hashmap.len(), 2);
        assert_eq!(hashmap.buckets.len(), 1);
        hashmap.insert("Martine", 12);
        assert_eq!(hashmap.buckets.len(), 2);
    }

    #[test]
    fn get_test() {
        let mut hashmap = HashMap::new();
        hashmap.insert("Hello", "world");
        hashmap.insert("Bonjour", "monde");

        assert_eq!(hashmap.get("Hello"), Some(&"world"));
        assert_eq!(hashmap.get("Bonjour"), Some(&"monde"));
        assert_eq!(hashmap.get("Bom dia"), None);
    }

    #[test]
    fn get_key_value_test() {
        let mut hashmap = HashMap::new();
        hashmap.insert("Hello", "world");
        hashmap.insert("Bonjour", "monde");

        assert_eq!(hashmap.get_key_value("Hello"), Some((&"Hello", &"world")));
        assert_eq!(
            hashmap.get_key_value("Bonjour"),
            Some((&"Bonjour", &"monde"))
        );
        assert_eq!(hashmap.get_key_value("Bom dia"), None);
    }

    #[test]
    fn get_mut_test() {
        let mut hashmap = HashMap::new();
        hashmap.insert("Hello".to_string(), "world".to_string());
        let value = hashmap.get_mut(&"Hello".to_string()).unwrap();
        value.push_str(" from the computer!");
        assert_eq!(value, "world from the computer!");

        assert_eq!(hashmap.get_mut(&"Bonjour".to_string()), None);
    }

    #[test]
    fn contains_key_test() {
        let mut hashmap = HashMap::new();
        hashmap.insert("Hello", "world");
        hashmap.insert("Bonjour", "monde");

        assert!(hashmap.contains_key("Hello"));
        assert!(hashmap.contains_key("Bonjour"));
        assert!(!hashmap.contains_key("Bom dia"));
    }

    #[test]
    fn is_empty_test() {
        let mut hashmap = HashMap::new();

        assert!(hashmap.is_empty());
        hashmap.insert(4, 2);
        assert!(!hashmap.is_empty());
    }

    #[test]
    fn iter_test() {
        let mut hashmap = HashMap::new();
        hashmap.insert("Jean", 100);
        hashmap.insert("Jeanne", 200);
        hashmap.insert("Julie", 50);
        hashmap.insert("Marc", 1);

        let mut iter = hashmap.iter();

        assert_eq!(Some((&"Jean", &100)), iter.next());
        assert_eq!(Some((&"Jeanne", &200)), iter.next());
        assert_eq!(Some((&"Julie", &50)), iter.next());
        assert_eq!(Some((&"Marc", &1)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iter_test() {
        let mut hashmap = HashMap::new();
        hashmap.insert("a", 1);
        hashmap.insert("b", 2);
        hashmap.insert("c", 3);

        for (&key, &value) in &hashmap {
            match key {
                "a" => assert_eq!(value, 1),
                "b" => assert_eq!(value, 2),
                "c" => assert_eq!(value, 3),
                _ => unreachable!(),
            }
        }

        assert_eq!(hashmap.into_iter().count(), 3);
    }
}
