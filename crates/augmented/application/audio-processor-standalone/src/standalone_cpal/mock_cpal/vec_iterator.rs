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
