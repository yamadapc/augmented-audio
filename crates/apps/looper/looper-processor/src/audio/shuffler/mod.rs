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
//! This is an implementation of a "shuffling-looper" similar to the one on a certain
//! tiny multi-effects and modeling pedal.
//!
//! After recording a loop, this processor can be used to automatically shuffle playback.
//!
//! It'll randomly generate a N step sequence of M slices of the clip.
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

use basedrop::{Shared, SharedCell};

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, SimpleAudioProcessor, VecAudioBuffer,
};

use crate::LooperProcessorHandle;

#[derive(Clone)]
pub struct LoopShufflerParams {
    pub num_slices: usize,
    pub sequence_length: usize,
    pub num_samples: usize,
}

#[derive(Clone)]
pub struct VirtualLoopSection {
    start: usize,
    end: usize,
}

pub struct LoopShufflerOutput {
    sequence: Vec<VirtualLoopSection>,
    sequence_step_size: usize,
}

pub struct LoopShufflerProcessorHandle {
    looped_clip: SharedCell<VecAudioBuffer<f32>>,
    looper_handle: Shared<LooperProcessorHandle>,
    params: SharedCell<Option<LoopShufflerParams>>,
    playhead: AtomicUsize,
    sequencer_output: SharedCell<Option<LoopShufflerOutput>>,
}

impl LoopShufflerProcessorHandle {
    fn set_clip(&self, clip: VecAudioBuffer<f32>) {
        self.looped_clip.set(make_shared(clip));
    }

    pub fn params(&self) -> Option<LoopShufflerParams> {
        let o = self.params.get();
        (*o).clone()
    }

    pub fn playhead(&self) -> Option<usize> {
        self.params
            .get()
            .as_ref()
            .map(|_| self.playhead.load(Ordering::Relaxed))
    }

    pub fn set_params(&self, params: LoopShufflerParams) {
        let output = run_sequencer(&params);
        let clip = self.looper_handle.looper_clip();
        let clip = clip.deref().borrow();
        let num_samples = clip.num_samples();

        let mut own_buffer = VecAudioBuffer::new();
        own_buffer.resize(clip.num_channels(), num_samples, 0.0);

        for (sample_index, frame) in own_buffer.frames_mut().enumerate() {
            for (channel_index, sample) in frame.iter_mut().enumerate() {
                let sample_index = sample_index % num_samples;
                let output = clip.get(channel_index, sample_index);
                *sample = output.get();
            }
        }

        self.set_clip(own_buffer);
        self.sequencer_output.set(make_shared(Some(output)));
        self.params.set(make_shared(Some(params)));
    }

    pub fn clear(&self) {
        self.sequencer_output.set(make_shared(None));
        self.params.set(make_shared(None));
    }
}

pub struct LoopShufflerProcessor {
    cursor: usize,
    handle: Shared<LoopShufflerProcessorHandle>,
}

impl LoopShufflerProcessor {
    pub fn new(looper_handle: Shared<LooperProcessorHandle>) -> Self {
        Self {
            cursor: 0,
            handle: make_shared(LoopShufflerProcessorHandle {
                looped_clip: make_shared_cell(VecAudioBuffer::new()),
                looper_handle,
                params: make_shared_cell(None),
                playhead: AtomicUsize::new(0),
                sequencer_output: make_shared_cell(None),
            }),
        }
    }

    pub fn handle(&self) -> &Shared<LoopShufflerProcessorHandle> {
        &self.handle
    }
}

impl AudioProcessor for LoopShufflerProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            self.s_process_frame(frame);
        }
    }
}

impl SimpleAudioProcessor for LoopShufflerProcessor {
    type SampleType = f32;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        self.prepare(settings);
    }

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        if let Some(LoopShufflerOutput {
            sequence,
            sequence_step_size,
        }) = self.handle.sequencer_output.get().deref()
        {
            let clip = self.handle.looped_clip.get();
            let num_samples = clip.deref().num_samples();
            let clip = clip.deref();

            if num_samples == 0 || *sequence_step_size == 0 {
                return;
            }

            let cursor = self.cursor % num_samples;
            let sequence_step_index = (cursor / sequence_step_size) % sequence.len();
            let VirtualLoopSection { start, .. } = &sequence[sequence_step_index];
            let overflow = cursor % sequence_step_size;
            let index = (start + overflow) % num_samples;
            let start_volume = (overflow as f32 / 80.0).min(1.0);
            let end_volume = ((sequence_step_size - overflow) as f32 / 80.0).min(1.0);

            for (channel, sample) in frame.iter_mut().enumerate() {
                let output_sample = clip.get(channel, index);
                *sample = *output_sample * start_volume * end_volume;
            }

            self.handle.playhead.store(index, Ordering::Relaxed);
            self.cursor += 1;
            if self.cursor >= num_samples {
                self.cursor = 0;
            }
        }
    }
}

pub fn run_sequencer(params: &LoopShufflerParams) -> LoopShufflerOutput {
    if params.sequence_length == 0 || params.num_slices == 0 {
        return LoopShufflerOutput {
            sequence: vec![],
            sequence_step_size: 0,
        };
    }

    let slice_len = params.num_samples / params.num_slices;
    let max_step_len = params.num_samples / params.sequence_length;

    let slices: Vec<VirtualLoopSection> = (0..params.num_slices)
        .map(|slice_num| {
            let slice_start = slice_num * slice_len;
            let slice_end = slice_start + slice_len;

            VirtualLoopSection {
                start: slice_start,
                end: slice_end,
            }
        })
        .collect();
    let sequence = (0..params.sequence_length)
        .map(|_step_num| {
            let ratio: f32 = rand::random();
            let index = (ratio * slices.len() as f32) as usize;
            let slice = slices[index].clone();

            let end = slice.start + max_step_len;
            VirtualLoopSection {
                start: slice.start,
                end: slice.end.min(end),
            }
        })
        .collect();

    LoopShufflerOutput {
        sequence,
        sequence_step_size: slice_len.min(max_step_len),
    }
}
