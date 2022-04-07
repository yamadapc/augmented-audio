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
/// This is `T-` and `T+` in the paper. Equations 3 and 4.
///
/// For each frame:
/// * `frame_deltas.minus` is the delta with the previous frame
/// * `frame_deltas.plus` is the delta with the next frame
pub struct DeltaFrames {
    pub minus: Vec<Vec<f32>>,
    pub plus: Vec<Vec<f32>>,
}

impl DeltaFrames {
    pub fn len(&self) -> usize {
        self.minus.len()
    }

    pub fn bins(&self) -> usize {
        if self.minus.is_empty() {
            0
        } else {
            self.minus[0].len()
        }
    }
}

/// Calculates `T-` and `T+`
pub fn calculate_deltas(magnitudes: &[Vec<f32>]) -> DeltaFrames {
    if magnitudes.is_empty() {
        return DeltaFrames {
            minus: vec![],
            plus: vec![],
        };
    }

    let empty_frame: Vec<f32> = magnitudes[0].iter().map(|_| 0.0).collect();
    let mut minus = vec![];
    let mut plus = vec![];

    for i in 0..magnitudes.len() {
        let frame = &magnitudes[i];
        let prev_frame = if i > 0 {
            &magnitudes[i - 1]
        } else {
            &empty_frame
        };
        let next_frame = if i < magnitudes.len() - 1 {
            &magnitudes[i + 1]
        } else {
            &empty_frame
        };

        let t_minus = frame.iter().zip(prev_frame).map(|(c, p)| c - p).collect();
        let t_plus = frame.iter().zip(next_frame).map(|(c, p)| c - p).collect();
        minus.push(t_minus);
        plus.push(t_plus)
    }

    DeltaFrames { minus, plus }
}
