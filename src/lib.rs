mod tests;

///A drop in replacement for `std::collections::HashMap`
#[derive(Default, Clone, PartialEq)]
pub struct VecMap<K, V>
where
    K: PartialEq,
    K: Eq,
{
    pub(self) vec: Vec<(K, V)>,
}
pub struct VaccantEntrty<'a, K: std::cmp::Eq, V> {
    key: K,
    table: &'a mut VecMap<K, V>,
}
impl<'a, K, V> VaccantEntrty<'a, K, V>
where
    K: std::cmp::Eq,
{
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, value: V) -> &'a mut V {
        _ = self.table.insert(self.key, value).unwrap();
        //When we insert a new value it is always last in the vec so this SHOULD be fine
        &mut self.table.vec.last_mut().unwrap().1
    }
}
impl<K: std::fmt::Debug + std::cmp::Eq, V> std::fmt::Debug for VaccantEntrty<'_, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

//Sigh, can use RustcOccupied entry, gotta make shit up myself
pub struct OccupiedEntrty<'a, K: std::cmp::Eq, V> {
    ///Index of the entry, only useful in here
    index: usize,
    key: &'a K,
    value: &'a mut V,
    table: &'a mut VecMap<K, V>,
}

impl<'a, K, V> OccupiedEntrty<'a, K, V>
where
    K: std::cmp::Eq,
{
    pub fn get(&self) -> &V {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut V {
        self.value
    }

    pub fn insert(&mut self, value: V) -> V {
        let old = self.table.vec.remove(self.index);
        self.table.vec.push((old.0, value));
        old.1
    }

    pub fn into_mut(self) -> &'a mut V {
        self.value
    }

    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn remove(self) -> V {
        self.table.remove(&self.key).unwrap()
    }

    pub fn remove_entry(self) -> (K, V) {
        self.table.vec.remove(self.index)
    }
}

pub enum Entry<'a, K: std::cmp::Eq, V> {
    Occupied(OccupiedEntrty<'a, K, V>),
    Vaccant(VaccantEntrty<'a, K, V>),
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
        let mut ind = None;
        for (index, i) in self.vec.iter().enumerate() {
            if i.0 == *k {
                ind = Some(index);
            }
        }
        if let Some(ind) = ind {
            return Some(self.vec.remove(ind).1);
        }
        None
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        for (key, v) in self.vec.iter() {
            if key == k {
                return Some(&v);
            }
        }
        None
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        for (key, v) in self.vec.iter_mut() {
            if key == k {
                return Some(v);
            }
        }
        None
    }

    pub fn contains_key(&self, k: &K) -> bool {
        for (key, _) in self.vec.iter() {
            if key == k {
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
