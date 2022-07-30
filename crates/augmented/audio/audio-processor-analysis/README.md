# audio-processor-analysis

Provides implementations of some audio analysis processors.

* **Peak detector** - [`peak_detector`]
* **FFT (Windowed/Overlapped)** - [`fft_processor`]
* **Transient detection** (not real-time) - [`transient_detection::stft`]
* **Window functions** - [`window_functions`]

### RMS
Real-time safe, per-sample (ticked by UI thread) RMS calculation.

### Peak detector
Peak detector with adjustable attack/release times.

### FFT
`rustfft` audio-processor, forwards or backwards, real-time safe, FFT.

Applies a Hann window by default. Several window functions are exported by [`window_functions`].

![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/window_functions/windows--HannWindow.png)

Then performs FFT with N bins.

![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/fft_processor.png--FFT_sine_440Hz.png)

Overlap is configurable

![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/screen.png)

### Envelope follower

Envelope follower implementation with adjustable attack/release times.

![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/audio-envelope.png)

### Transient detection

Implements "[A Transient Detection Algorithm for Audio Using Iterative Analysis of STFT.](https://www.researchgate.net/profile/Balaji-Thoshkahna/publication/220723752_A_Transient_Detection_Algorithm_for_Audio_Using_Iterative_Analysis_of_STFT/links/0deec52e6331412aed000000/A-Transient-Detection-Algorithm-for-Audio-Using-Iterative-Analysis-of-STFT.pdf)".

Does polyphonic transient detection, able to output signal or markers

![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/transient_detection/stft.png)

### Window functions
Several window functions are implemented and configurable.

License: MIT
