#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::unwrap_or_default
)]

use std::{borrow::Borrow, collections::HashMap, ops::Index};

mod tests;

///A drop in replacement for `std::collections::HashMap`
#[derive(Default, Clone, Eq)]
pub struct VecMap<K, V> {
    vec: Vec<(K, V)>,
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for VecMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("VecMap").field("vec", &self.vec).finish()
        f.debug_map()
            .entries(self.vec.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}

impl<K: PartialEq + Eq, V: PartialEq> PartialEq for VecMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .map(|i| other.get(i.0).map(|j| j == i.1).unwrap_or_default())
            .fold(true, |a, i| i && a)
    }
}
pub struct VaccantEntrty<'a, K: std::cmp::Eq, V> {
    key: K,
    table: &'a mut Vec<(K, V)>,
}
impl<'a, K, V> VaccantEntrty<'a, K, V>
where
    K: std::cmp::Eq,
{
    pub const fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, value: V) -> &'a mut V {
        let key = self.key;
        self.table.push((key, value));
        //When we insert a new value it is always last in the vec so this SHOULD be fine
        &mut self.table.last_mut().unwrap().1
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
    // key: &'a K,
    table: &'a mut Vec<(K, V)>,
}

impl<'a, K, V> OccupiedEntrty<'a, K, V>
where
    K: std::cmp::Eq,
{
    pub fn get(&self) -> &V {
        &self.table.get(self.index).unwrap().1
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.table.get_mut(self.index).unwrap().1
    }

    pub fn insert(&mut self, value: V) -> V {
        let old = self.table.remove(self.index);
        self.table.push((old.0, value));
        old.1
    }

    pub fn into_mut(self) -> &'a mut V {
        &mut self.table.get_mut(self.index).unwrap().1
    }

    pub fn key(&self) -> &K {
        &self.table.get(self.index).unwrap().0
    }
    pub fn remove(self) -> V {
        self.table.swap_remove(self.index).1
    }

    pub fn remove_entry(self) -> (K, V) {
        self.table.remove(self.index)
    }
}

pub enum Entry<'a, K, V>
where
    K: std::cmp::Eq,
{
    Occupied(OccupiedEntrty<'a, K, V>),
    Vacant(VaccantEntrty<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: std::cmp::Eq,
{
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default()),
        }
    }
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let value = default(&e.key);
                e.insert(value)
            }
        }
    }
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(e) => e.key(),
            Entry::Vacant(e) => e.key(),
        }
    }
    #[must_use]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut e) => {
                let borrow = e.get_mut();
                f(borrow);
                Entry::Occupied(e)
            }
            Entry::Vacant(_) => self,
        }
    }
}
impl<'a, K, V> Entry<'a, K, V>
where
    K: std::cmp::Eq,
    V: Default,
{
    pub fn or_default(self) -> &'a mut V {
        self.or_insert(V::default())
    }
}

pub struct IntoIter<K, V> {
    iter: std::vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoIter<K, V>
where
    K: Eq,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V>
where
    K: Eq,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> IntoIterator for VecMap<K, V>
where
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

#[derive(Clone, Debug)]
pub struct Keys<'a, K, V> {
    inner: core::slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }
}

impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug)]
pub struct IntoKeys<K, V> {
    inner: std::vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| i.0)
    }
}

impl<K, V> ExactSizeIterator for IntoKeys<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug)]
pub struct Values<'a, K, V> {
    inner: core::slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
}

impl<K, V> ExactSizeIterator for Values<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug)]
pub struct ValuesMut<'a, K, V> {
    inner: core::slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug)]
pub struct IntoValues<K, V> {
    inner: std::vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| i.1)
    }
}

impl<K, V> ExactSizeIterator for IntoValues<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, K, V> {
    inner: core::slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    inner: core::slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (&(*k), v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug)]
pub struct Drain<'a, K, V> {
    inner: std::vec::Drain<'a, (K, V)>,
}

impl<K, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<K, V> ExactSizeIterator for Drain<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> VecMap<K, V>
where
    K: Eq,
{
    ///Creates an empty `VecMap`
    pub const fn new() -> Self {
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
        self.vec.reserve(additional);
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old = self.remove(&k);
        self.vec.push((k, v));

        old
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
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

    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        let mut ind = None;
        for (index, i) in self.vec.iter().enumerate() {
            if i.0 == *k {
                ind = Some(index);
            }
        }
        if let Some(ind) = ind {
            return Some(self.vec.remove(ind));
        }
        None
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        for (key, v) in &self.vec {
            if k == key.borrow() {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let mut ind = None;
        for (index, (key, _)) in self.vec.iter().enumerate() {
            if k == key.borrow() {
                ind = Some(index);
            }
        }
        Some(&mut self.vec.get_mut(ind?)?.1)
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        for (key, _) in &self.vec {
            if key == k {
                return true;
            }
        }
        false
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.vec.shrink_to(min_capacity);
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match {
            let mut val = None;
            for (index, (map_key, _)) in self.vec.iter().enumerate() {
                if &key == map_key {
                    val = Some(index);
                    break;
                }
            }
            val
        } {
            Some(e) => Entry::Occupied(OccupiedEntrty {
                index: e,
                table: &mut self.vec,
            }),
            None => Entry::Vacant(VaccantEntrty {
                key,
                table: &mut self.vec,
            }),
        }
    }
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Eq + ?Sized,
    {
        for i in &self.vec {
            if &i.0 == k {
                return Some((&i.0, &i.1));
            }
        }
        None
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            inner: self.vec.iter(),
        }
    }

    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys {
            inner: self.vec.into_iter(),
        }
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            inner: self.vec.iter(),
        }
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            inner: self.vec.iter_mut(),
        }
    }

    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues {
            inner: self.vec.into_iter(),
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.vec.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            inner: self.vec.iter_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn drain(&mut self) -> Drain<'_, K, V> {
        Drain {
            inner: self.vec.drain(..),
        }
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        let mut f = f;
        self.vec.retain_mut(|i| f(&i.0, &mut i.1));
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn hasher<S>(&self) -> &S
    where
        S: std::hash::BuildHasher,
    {
        unimplemented!("Hasher is not implemented for VecMap");
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for VecMap<K, V>
where
    K: Eq + Copy,
    V: Copy,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(*k, *v);
        }
    }
}

impl<K, V> Extend<(K, V)> for VecMap<K, V>
where
    K: Eq,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for VecMap<K, V>
where
    K: Eq,
{
    fn from(value: [(K, V); N]) -> Self {
        let mut o = Self::new();
        for (k, v) in value {
            o.insert(k, v);
        }
        o
    }
}

impl<K, V> FromIterator<(K, V)> for VecMap<K, V>
where
    K: Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut o = Self::new();
        for (k, v) in iter {
            o.insert(k, v);
        }
        o
    }
}

impl<K, Q, V> Index<&Q> for VecMap<K, V>
where
    K: Eq + Borrow<Q>,
    Q: Eq + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, K, V> IntoIterator for &'a VecMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.vec.iter(),
        }
    }
}

impl<'a, K, V> IntoIterator for &'a mut VecMap<K, V> {
    type Item = (&'a K, &'a mut V);

    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            inner: self.vec.iter_mut(),
        }
    }
}

impl<K, V> PartialEq<HashMap<K, V>> for VecMap<K, V>
where
    K: Eq,
    V: PartialEq,
{
    fn eq(&self, other: &HashMap<K, V>) -> bool {
        other.iter().eq(self.iter())
    }
}
