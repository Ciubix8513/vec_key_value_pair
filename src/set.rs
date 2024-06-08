use std::{
    borrow::Borrow,
    collections::TryReserveError,
    fmt::Debug,
    iter::{Chain, FusedIterator},
    ops::{BitAnd, BitOr, BitXor, Sub},
    vec,
};

#[cfg(test)]
mod tests;

pub struct VecSet<T> {
    inner: Vec<T>,
}

pub struct Iter<'a, T> {
    inner: core::slice::Iter<'a, T>,
}

impl<K> Clone for Iter<'_, K> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K: Debug> Debug for Iter<'_, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K> ExactSizeIterator for Iter<'_, K> {
    fn len(&self) -> usize {
        let (lower, upper) = self.inner.size_hint();
        assert_eq!(upper, Some(lower));
        lower
    }
}

impl<K> FusedIterator for Iter<'_, K> {}

impl<'a, K> Iterator for Iter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Drain<'a, T> {
    inner: std::vec::Drain<'a, T>,
}

impl<K: Debug> Debug for Drain<'_, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<K> ExactSizeIterator for Drain<'_, K> {
    fn len(&self) -> usize {
        let (lower, upper) = self.inner.size_hint();
        // Note: This assertion is overly defensive, but it checks the invariant
        // guaranteed by the trait. If this trait were rust-internal,
        // we could use debug_assert!; assert_eq! will check all Rust user
        // implementations too.
        assert_eq!(upper, Some(lower));
        lower
    }
}

impl<K> FusedIterator for Drain<'_, K> {}

impl<'a, K> Iterator for Drain<'a, K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Difference<'a, T> {
    iter: Iter<'a, T>,
    other: &'a VecSet<T>,
}

impl<T> Clone for Difference<'_, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            other: self.other,
        }
    }
}

impl<T: Debug> Debug for Difference<'_, T>
where
    T: Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T> ExactSizeIterator for Difference<'_, T>
where
    T: Eq,
{
    fn len(&self) -> usize {
        let (lower, upper) = self.iter.size_hint();
        // Note: This assertion is overly defensive, but it checks the invariant
        // guaranteed by the trait. If this trait were rust-internal,
        // we could use debug_assert!; assert_eq! will check all Rust user
        // implementations too.
        assert_eq!(upper, Some(lower));
        lower
    }
}

impl<T> FusedIterator for Difference<'_, T> where T: Eq {}

impl<'a, T> Iterator for Difference<'a, T>
where
    T: Eq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.iter.next() {
            if !self.other.contains(n) {
                return Some(n);
            }
        }
        None
    }
}

pub struct SymmetricDifference<'a, T> {
    iter: Chain<Difference<'a, T>, Difference<'a, T>>,
}

impl<T> Clone for SymmetricDifference<'_, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<T: Debug> Debug for SymmetricDifference<'_, T>
where
    T: Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T> FusedIterator for SymmetricDifference<'_, T> where T: Eq {}

impl<'a, T> Iterator for SymmetricDifference<'a, T>
where
    T: Eq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Intersection<'a, T> {
    iter: Iter<'a, T>,
    other: &'a VecSet<T>,
}

impl<T> Clone for Intersection<'_, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            other: self.other,
        }
    }
}

impl<T: Debug> Debug for Intersection<'_, T>
where
    T: Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T> FusedIterator for Intersection<'_, T> where T: Eq {}

impl<'a, T> Iterator for Intersection<'a, T>
where
    T: Eq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.iter.next() {
            if self.other.contains(n) {
                return Some(n);
            }
        }
        None
    }
}

pub struct Union<'a, T> {
    iter: Chain<Iter<'a, T>, Difference<'a, T>>,
}

impl<T> Clone for Union<'_, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<T: Debug> Debug for Union<'_, T>
where
    T: Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T> FusedIterator for Union<'_, T> where T: Eq {}

impl<'a, T> Iterator for Union<'a, T>
where
    T: Eq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T: Debug> Debug for IntoIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

impl<'a, T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> VecSet<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        Drain {
            inner: self.inner.drain(..),
        }
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.retain(f);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

impl<T> VecSet<T>
where
    T: Eq,
{
    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T> {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    pub fn symmetric_difference<'a>(&'a self, other: &'a VecSet<T>) -> SymmetricDifference<'a, T> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }

    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T> {
        if self.len() <= other.len() {
            Intersection {
                iter: self.iter(),
                other,
            }
        } else {
            Intersection {
                iter: other.iter(),
                other: self,
            }
        }
    }

    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T> {
        if self.len() >= other.len() {
            Union {
                iter: self.iter().chain(other.difference(self)),
            }
        } else {
            Union {
                iter: other.iter().chain(self.difference(other)),
            }
        }
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        for i in &self.inner {
            if i.borrow() == value {
                return true;
            }
        }
        false
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        for i in &self.inner {
            if i.borrow() == value {
                return Some(i);
            }
        }
        None
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for i in &self.inner {
            if other.contains(i) {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for i in &self.inner {
            if !other.contains(i) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    pub fn insert(&mut self, value: T) -> bool {
        if self.contains(&value) {
            return false;
        }
        self.inner.push(value);

        true
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        let mut r_index = None;
        for (index, v) in self.inner.iter().enumerate() {
            if v == &value {
                r_index = Some(index);
                break;
            }
        }
        if let Some(i) = r_index {
            let old = self.inner.remove(i);
            self.inner.push(value);
            return Some(old);
        }

        self.inner.push(value);

        None
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let mut r_index = None;
        for (index, v) in self.inner.iter().enumerate() {
            if v.borrow() == value {
                r_index = Some(index);
                break;
            }
        }

        if let Some(i) = r_index {
            _ = self.inner.remove(i);
            return true;
        }

        false
    }
}

impl<T> BitAnd<&VecSet<T>> for &VecSet<T>
where
    T: Eq + Clone,
{
    type Output = VecSet<T>;

    fn bitand(self, rhs: &VecSet<T>) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T> BitOr<&VecSet<T>> for &VecSet<T>
where
    T: Eq + Clone,
{
    type Output = VecSet<T>;

    fn bitor(self, rhs: &VecSet<T>) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T> BitXor<&VecSet<T>> for &VecSet<T>
where
    T: Eq + Clone,
{
    type Output = VecSet<T>;

    fn bitxor(self, rhs: &VecSet<T>) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T> Clone for VecSet<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Debug for VecSet<T>
where
    T: Debug + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self).finish()
    }
}

impl<T> Default for VecSet<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<'a, T> Extend<&'a T> for VecSet<T>
where
    T: 'a + Eq + Copy,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for i in iter {
            _ = self.insert(*i);
        }
    }
}

impl<T> Extend<T> for VecSet<T>
where
    T: Eq,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            _ = self.insert(i);
        }
    }
}

impl<T, const N: usize> From<[T; N]> for VecSet<T>
where
    T: Eq,
{
    fn from(value: [T; N]) -> Self {
        let mut o = VecSet::new();
        for i in value {
            o.insert(i);
        }

        o
    }
}

impl<T> FromIterator<T> for VecSet<T>
where
    T: Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut o = VecSet::new();
        for i in iter {
            o.insert(i);
        }

        o
    }
}

impl<'a, T> IntoIterator for &'a VecSet<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

impl<T> IntoIterator for VecSet<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl<T> PartialEq for VecSet<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && self.is_superset(other)
    }
}

impl<T> Eq for VecSet<T> where T: Eq {}

impl<T> Sub<&VecSet<T>> for &VecSet<T>
where
    T: Eq + Clone,
{
    type Output = VecSet<T>;

    fn sub(self, rhs: &VecSet<T>) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}
