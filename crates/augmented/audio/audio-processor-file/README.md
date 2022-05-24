# audio-processor-file

[`audio_processor_traits::AudioProcessor`] implementations for audio file playback & writing.

Currently two processors are provided:

* [`AudioFileProcessor`] is an input file processor, its `prepare` method will *load the whole
  file onto memory*. Both `wav` and `mp3` are supported via [`symphonia`]
  - If streaming is a requirement this won't work
* [`OutputAudioFileProcessor`] writes `wav` files

License: MIT
