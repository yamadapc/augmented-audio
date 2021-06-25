# background-garbage-collector
Experiments with a background queue based ref-counting GC to offload deallocations from audio-thread.

Same as https://github.com/glowcoil/basedrop

Lock-free queue ported from https://github.com/max0x7ba/atomic_queue