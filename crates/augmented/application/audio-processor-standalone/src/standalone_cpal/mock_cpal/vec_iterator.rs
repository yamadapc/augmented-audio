pub struct VecIterator<T> {
    inner: Vec<T>,
    cursor: usize,
}

impl<T: Clone> Iterator for VecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.inner.len() {
            None
        } else {
            let item = T::clone(&self.inner[self.cursor]);
            self.cursor += 1;
            Some(item)
        }
    }
}

impl<T> From<Vec<T>> for VecIterator<T> {
    fn from(v: Vec<T>) -> Self {
        VecIterator {
            inner: v,
            cursor: 0,
        }
    }
}
