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
use augmented_adsr_envelope::Envelope;
use std::time::Duration;

pub const DEFAULT_CLICK_ATTACK_MS: u64 = 1;
pub const DEFAULT_CLICK_DECAY_RELEASE_MS: u64 = 10;
pub const DEFAULT_SAMPLE_RATE: f32 = 44100.0;
pub const DEFAULT_TEMPO: f32 = 120.0;

pub fn build_envelope() -> Envelope {
    let envelope = Envelope::new();
    envelope.set_attack(Duration::from_millis(DEFAULT_CLICK_ATTACK_MS));
    envelope.set_decay(Duration::from_millis(DEFAULT_CLICK_DECAY_RELEASE_MS));
    envelope.set_sustain(0.0);
    envelope.set_release(Duration::from_millis(DEFAULT_CLICK_DECAY_RELEASE_MS));
    envelope
}
