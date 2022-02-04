use crate::LooperProcessorHandle;
use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{
    AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings, VecAudioBuffer,
};
use basedrop::{Shared, SharedCell};
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

#[derive(Clone)]
pub struct LoopSequencerParams {
    pub num_slices: usize,
    pub sequence_length: usize,
    pub num_samples: usize,
}

#[derive(Clone)]
pub struct VirtualLoopSection {
    start: usize,
    end: usize,
}

pub struct LoopSequencerOutput {
    sequence: Vec<VirtualLoopSection>,
    sequence_step_size: usize,
}

pub struct LoopSequencerProcessorHandle {
    looped_clip: SharedCell<VecAudioBuffer<f32>>,
    looper_handle: Shared<LooperProcessorHandle>,
    params: SharedCell<Option<LoopSequencerParams>>,
    playhead: AtomicUsize,
    sequencer_output: SharedCell<Option<LoopSequencerOutput>>,
}

impl LoopSequencerProcessorHandle {
    fn set_clip(&self, clip: VecAudioBuffer<f32>) {
        self.looped_clip.set(make_shared(clip));
    }

    pub fn params(&self) -> Option<LoopSequencerParams> {
        let o = self.params.get();
        (*o).clone()
    }

    pub fn playhead(&self) -> Option<usize> {
        self.params
            .get()
            .as_ref()
            .map(|_| self.playhead.load(Ordering::Relaxed))
    }

    pub fn set_params(&self, params: LoopSequencerParams) {
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
}

pub struct LoopSequencerProcessor {
    cursor: usize,
    handle: Shared<LoopSequencerProcessorHandle>,
}

impl LoopSequencerProcessor {
    pub fn new(looper_handle: Shared<LooperProcessorHandle>) -> Self {
        Self {
            cursor: 0,
            handle: make_shared(LoopSequencerProcessorHandle {
                looped_clip: make_shared_cell(VecAudioBuffer::new()),
                looper_handle,
                params: make_shared_cell(None),
                playhead: AtomicUsize::new(0),
                sequencer_output: make_shared_cell(None),
            }),
        }
    }

    pub fn handle(&self) -> &Shared<LoopSequencerProcessorHandle> {
        &self.handle
    }
}

impl AudioProcessor for LoopSequencerProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        if let Some(LoopSequencerOutput {
            sequence,
            sequence_step_size,
        }) = self.handle.sequencer_output.get().deref()
        {
            let clip = self.handle.looped_clip.get();
            let num_samples = clip.deref().num_samples();
            let clip = clip.deref();

            if num_samples == 0 {
                return;
            }

            for frame in data.frames_mut() {
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
}

pub fn run_sequencer(params: &LoopSequencerParams) -> LoopSequencerOutput {
    if params.sequence_length == 0 || params.num_slices == 0 {
        return LoopSequencerOutput {
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
        .map(|step_num| {
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

    LoopSequencerOutput {
        sequence,
        sequence_step_size: max_step_len,
    }
}
