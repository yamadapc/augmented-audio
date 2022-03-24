static SMALL_AMOUNT: f32 = 9.8607615e-32;

#[inline]
pub fn undenormalize(mut sample: f32) -> f32 {
    sample += SMALL_AMOUNT;
    sample - SMALL_AMOUNT
}

pub fn make_vec(size: usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(size);
    v.resize(size, 0.0);
    v
}
