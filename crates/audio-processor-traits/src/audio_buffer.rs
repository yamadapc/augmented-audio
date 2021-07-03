/// Passing around `&mut [f32]` as audio buffers isn't good because:
///
/// * Some libraries / APIs will use interleaved buffers
/// * Some will not
/// * If you pick one all your processor code is bound to a buffer layout
/// * If there's an abstraction on top the processor code can work for any buffer layout while
///   still having the sample performance
/// * Currently `AudioProcessor` is made to work with cpal interleaved buffers; it then needs
///   conversion to work with VST.
/// * That's very unfortunate. I'd like to write a single processor that can work with both buffer
///   types with no overhead.
pub trait AudioBuffer {
    type SampleType: num::Float + Sync + Send;

    fn num_channels(&self) -> usize;
    fn num_samples(&self) -> usize;
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType;
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType;
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType);
}

pub struct InterleavedAudioBuffer<'a, SampleType> {
    num_channels: usize,
    inner: &'a mut [SampleType],
}

impl<'a, SampleType> InterleavedAudioBuffer<'a, SampleType> {
    pub fn new(num_channels: usize, inner: &'a mut [SampleType]) -> Self {
        Self {
            num_channels,
            inner,
        }
    }

    pub fn inner(&self) -> &[SampleType] {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut [SampleType] {
        &mut self.inner
    }
}

impl<'a, SampleType: num::Float + Sync + Send> AudioBuffer
    for InterleavedAudioBuffer<'a, SampleType>
{
    type SampleType = SampleType;

    fn num_channels(&self) -> usize {
        self.num_channels
    }

    fn num_samples(&self) -> usize {
        self.inner.len() / self.num_channels
    }

    fn get(&self, channel: usize, sample: usize) -> &SampleType {
        &self.inner[sample * self.num_channels + channel]
    }

    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut SampleType {
        &mut self.inner[sample * self.num_channels + channel]
    }

    fn set(&mut self, channel: usize, sample: usize, value: SampleType) {
        let sample_ref = self.get_mut(channel, sample);
        *sample_ref = value;
    }
}
