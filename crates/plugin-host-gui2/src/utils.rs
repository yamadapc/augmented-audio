#[macro_export]
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()), *])
}
