# audio-processor-bitcrusher

Implements a simple bitcrusher based on "sample-and-hold".

[`BitCrusherProcessor`] is the [`audio_processor_traits::AudioProcessor`] implementation.

[`BitCrusherHandle`] is the handle with which to change parameters from any thread. A generic
handle is implemented to generate generic GUIs.

License: MIT
