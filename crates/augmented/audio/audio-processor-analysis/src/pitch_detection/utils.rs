use std::cmp::Ordering;

/// Given a bin, estimate its frequency
pub fn frequency_from_location(sample_rate: f32, location: f32, bin_count: usize) -> f32 {
    let ratio: f32 = location / bin_count as f32;
    sample_rate * ratio
}

pub fn maximum_index(iterator: impl ExactSizeIterator<Item = f32>) -> Option<usize> {
    iterator
        .enumerate()
        .max_by(|(_, f1): &(usize, f32), (_, f2): &(usize, f32)| {
            f1.abs().partial_cmp(&f2.abs()).unwrap_or(Ordering::Equal)
        })
        .map(|(i, _f)| i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_index_when_exists() {
        let iterator: Vec<f32> = vec![10.0, 30.0, 20.0, 5.0];
        let index = maximum_index(iterator.iter().cloned()).unwrap();
        assert_eq!(index, 1);
    }

    #[test]
    fn test_maximum_index_when_does_not_exist() {
        let iterator: Vec<f32> = vec![];
        let index = maximum_index(iterator.iter().cloned());
        assert_eq!(index, None);
    }
}
