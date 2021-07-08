use std::ops::{Index, IndexMut, Rem};
use std::slice::SliceIndex;

#[derive(Debug)]
pub struct CircularVec<T> {
    inner: Vec<T>,
}

impl<T> Default for CircularVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CircularVec<T> {
    pub fn new() -> Self {
        CircularVec { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        CircularVec {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn with_vec(vec: Vec<T>) -> Self {
        CircularVec { inner: vec }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> CircularVec<T> {
    #[inline]
    fn circular_index<I>(&self, index: I) -> I
    where
        I: SliceIndex<[T]> + Rem<Output = I> + From<usize>,
    {
        let length = I::from(self.inner.len());
        index % length
    }
}

impl<T: Clone> CircularVec<T> {
    pub fn with_size(size: usize, value: T) -> Self {
        let mut v = CircularVec::new();
        v.resize(size, value);
        v
    }

    pub fn resize(&mut self, new_len: usize, value: T) {
        self.inner.resize(new_len, value);
    }
}

impl<T> IntoIterator for CircularVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T, I: SliceIndex<[T]> + Rem<Output = I> + From<usize>> Index<I> for CircularVec<T> {
    type Output = <Vec<T> as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        let index = self.circular_index(index);
        &self.inner[index]
    }
}

impl<T, I: SliceIndex<[T]> + Rem<Output = I> + From<usize>> IndexMut<I> for CircularVec<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = self.circular_index(index);
        &mut self.inner[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::CircularVec;

    #[test]
    fn test_we_can_create_a_circular_vec() {
        let mut vec = CircularVec::new();
        vec.resize(10, 0);
        assert_eq!(vec.len(), 10);
        for i in 0..10 {
            vec[i] = i;
        }
        for i in 0..100 {
            assert_eq!(vec[i], i % 10);
        }
    }

    #[test]
    fn test_we_can_use_the_vec_as_normal() {
        let mut vec = CircularVec::new();
        vec.resize(10, 0);
        assert_eq!(vec.len(), 10);
        vec[0] = 10;
        vec[1] = 20;
        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 0);
    }

    #[test]
    fn test_we_can_set_elements_past_the_length_into_the_vec() {
        let mut vec = CircularVec::new();
        vec.resize(10, 0);
        assert_eq!(vec.len(), 10);
        vec[100] = 10;
        assert_eq!(vec[0], 10);
    }
}
