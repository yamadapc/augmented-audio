pub struct Cache<K, T> {
    value: Option<(K, T)>,
}

impl<K, T> Default for Cache<K, T> {
    fn default() -> Self {
        Cache::new(None)
    }
}

impl<K, T> Cache<K, T> {
    pub fn new(value: Option<(K, T)>) -> Self {
        Cache { value }
    }
}

impl<K, T> Cache<K, T>
where
    K: PartialEq,
{
    pub fn get<F>(&mut self, key: K, builder: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self.value.take() {
            Some((cached_key, cached_value)) if cached_key == key => cached_value,
            _ => builder(),
        }
    }

    pub fn save(&mut self, key: K, value: T) {
        self.value = Some((key, value));
    }
}
