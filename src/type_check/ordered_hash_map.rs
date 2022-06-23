use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Iter;

pub(crate) struct OrderedHashMap<K: Clone + Eq + Hash, V> {
    insertion_order: Vec<K>,
    inner: HashMap<K, V>,
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            insertion_order: Default::default(),
            inner: Default::default(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(old_val) = self.inner.insert(k.clone(), v) {
            // find and remove the old reference before pushing a new one.
            let previous_pos = self
                .insertion_order
                .iter()
                .position(|existing_key| existing_key == &k);

            if let Some(old_idx) = previous_pos {
                self.insertion_order.remove(old_idx);
            }

            self.insertion_order.push(k);
            Some(old_val)
        } else {
            self.insertion_order.push(k);
            None
        }
    }

    pub fn keys(&self) -> Keys<'_, K> {
        Keys {
            inner: self.insertion_order.iter(),
        }
    }
}

impl<K, V> Default for OrderedHashMap<K, V>
where
    K: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct Keys<'a, K> {
    inner: Iter<'a, K>,
}

impl<'a, K: 'a> Iterator for Keys<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(k) => Some(k),
            None => None,
        }
    }
}

impl<'a, K: 'a> DoubleEndedIterator for Keys<'a, K> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner.next_back() {
            Some(k) => Some(k),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_iterated_results_in_same_order_of_insertion() {
        let mut map = OrderedHashMap::default();
        map.insert("2", 2);
        map.insert("0", 0);
        map.insert("1", 1);

        let mut keys = map.keys();

        assert_eq!(Some(&"2"), keys.next());
        assert_eq!(Some(&"0"), keys.next());
        assert_eq!(Some(&"1"), keys.next());
        assert_eq!(None, keys.next());
    }
}
