///A drop in replacement for `std::collections::HashMap`
pub struct VecMap<K, V>
where
    K: PartialEq,
{
    vec: Vec<(K, V)>,
}

impl<K, V> VecMap<K, V>
where
    K: PartialEq,
{
    ///Creates an empty `VecMap`
    pub fn new(vec: Vec<(K, V)>) -> Self {
        Self { vec }
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

    ///An iterator over all the keys of the map in order they were added, has time complexity of O(len)
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.vec.iter().map(|i| i.0).into_iter()
    }
}

#[cfg(test)]
mod tests {}
