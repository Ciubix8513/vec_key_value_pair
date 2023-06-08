mod tests;

///A drop in replacement for `std::collections::HashMap`
#[derive(Default, Clone, PartialEq)]
pub struct VecMap<K, V>
where
    K: PartialEq,
    K: Eq,
{
    vec: Vec<(K, V)>,
}

pub struct IntoIter<K, V> {
    iter: std::vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoIter<K, V>
where
    K: PartialEq,
    K: Eq,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
impl<K, V> IntoIterator for VecMap<K, V>
where
    K: PartialEq,
    K: Eq,
{
    type Item = (K, V);

    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.vec.into_iter(),
        }
    }
}

impl<K, V> VecMap<K, V>
where
    K: PartialEq,
    K: Eq,
{
    ///Creates an empty `VecMap`
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    //Creates an empty `VecMap` with at least the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }

    ///Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old = self.remove(&k);
        self.vec.push((k, v));

        match old {
            Some(old) => Some(old),
            None => None,
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let old_value = self.get(k);
        if old_value.is_some() {
            self.vec.retain_mut(|v| &v.0 != k)
        }
        old_value.map(|i| *i)
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        for (key, v) in self.vec {
            if &key == k {
                return Some(&v);
            }
        }
        None
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        for (key, mut v) in self.vec {
            if &key == k {
                return Some(&mut v);
            }
        }
        None
    }

    pub fn contains_key(&self, k: &K) -> bool {
        for (key, _) in self.vec {
            if &key == k {
                return true;
            }
        }
        false
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }

    pub fn entry(&mut self, key: &K) -> Entry<'_, K, V> {
        todo!()
    }

    //An iterator over all the keys of the map in order they were added, has time complexity of O(len)
    // pub fn keys(&self) -> {
    // self.vec.iter().
    // }
}
