// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
pub use hadamard::HadamardMatrix;
pub use householder::apply_householder;

mod householder {
    pub fn apply_householder(frame: &mut [f32]) {
        let multiplier = -2.0 / (frame.len() as f32);
        let sum: f32 = frame.iter().sum::<f32>() * multiplier;
        for x in frame {
            *x += sum;
        }
    }
}

mod hadamard {
    use nalgebra::{ArrayStorage, Const, Matrix, U1};

    /// A `Hadamard` matrix - https://en.wikipedia.org/wiki/Hadamard_matrix
    pub struct HadamardMatrix<const D: usize> {
        inner: Matrix<f32, Const<D>, Const<D>, ArrayStorage<f32, D, D>>,
    }

    impl<const D: usize> Default for HadamardMatrix<D>
    where
        [[f32; D]; D]: Default,
    {
        /// Same as `HadamardMatrix::new()`
        fn default() -> Self {
            Self::new()
        }
    }

    impl<const D: usize> HadamardMatrix<D>
    where
        [[f32; D]; D]: Default,
    {
        /// Construct a new matrix
        pub fn new() -> Self {
            let mut inner = build_hadamard_matrix();
            let scaling = (1.0 / D as f32).sqrt();
            for sample in inner.iter_mut() {
                *sample = *sample * scaling;
            }

            Self { inner }
        }

        /// Apply the matrix against a frame of audio. The frame must be `D` channels otherwise this
        /// will fail.
        pub fn apply(&self, frame: &mut [f32]) {
            let target = Matrix::<f32, U1, Const<D>, ArrayStorage<f32, 1, D>>::from_iterator(
                frame.iter().cloned(),
            );
            let result = target * self.inner;
            for (r, slot) in result.iter().zip(frame) {
                *slot = *r;
            }
        }
    }

    /// Build a Hadamard matrix of given dimension. For example, if `D` is 4, this will return
    /// a 4x4 matrix, which can be used with a 4 channel input.
    fn build_hadamard_matrix<const D: usize>(
    ) -> Matrix<f32, Const<D>, Const<D>, ArrayStorage<f32, D, D>>
    where
        [[f32; D]; D]: Default,
    {
        let mut storage = vec![];
        storage.resize(D * D, 0.0);
        storage[0] = 1.0;

        let mut x = 1;
        while x < D {
            for i in 0..x {
                for j in 0..x {
                    storage[(i + x) * D + j] = storage[i * D + j];
                    storage[i * D + (j + x)] = storage[i * D + j];
                    storage[(i + x) * D + (j + x)] = -storage[i * D + j];
                }
            }
            x = 2 * x;
        }

        let matrix: Matrix<f32, Const<D>, Const<D>, ArrayStorage<f32, D, D>> =
            Matrix::<f32, Const<D>, Const<D>, ArrayStorage<f32, D, D>>::from_iterator(
                storage.into_iter(),
            );

        matrix
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_build_hadamard_matrix() {
            let result = build_hadamard_matrix::<4>();
            let sample = nalgebra::Matrix4::new(
                1.0, 1.0, 1.0, 1.0, // \n
                1.0, -1.0, 1.0, -1.0, // \n
                1.0, 1.0, -1.0, -1.0, // \n
                1.0, -1.0, -1.0, 1.0,
            );
            assert_eq!(result, sample)
        }
    }
}
