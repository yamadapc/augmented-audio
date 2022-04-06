use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AtomicF32, AudioBuffer, VecAudioBuffer};

use super::scratch_pad;

pub struct CopyLoopClipParams<'a> {
    pub scratch_pad: &'a scratch_pad::ScratchPad,
    pub start_cursor: usize,
    pub length: usize,
}

pub fn copy_looped_clip(params: CopyLoopClipParams, result_buffer: &mut VecAudioBuffer<AtomicF32>) {
    let buffer = params.scratch_pad.buffer();

    result_buffer.resize(buffer.num_channels(), params.length, AtomicF32::new(0.0));

    for channel in 0..buffer.num_channels() {
        for i in 0..params.length {
            let index = (i + params.start_cursor) % buffer.num_samples();
            let sample = buffer.get(channel, index).clone();
            result_buffer.set(channel, i, sample);
        }
    }
}

pub fn empty_buffer(channels: usize, samples: usize) -> VecAudioBuffer<AtomicF32> {
    let mut b = VecAudioBuffer::new();
    b.resize(channels, samples, AtomicF32::new(0.0));
    b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buffer = empty_buffer(2, 10);
        assert_eq!(buffer.num_channels(), 2);
        assert_eq!(buffer.num_samples(), 10);
        for sample in buffer.slice() {
            assert_eq!(sample.get(), 0.0)
        }
    }
}
