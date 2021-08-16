# audio-garbage-collector-v2
Experiments with a background queue based ref-counting GC to offload deallocations from audio-thread.

Same as https://github.com/glowcoil/basedrop. Maybe will replace it in this repository in the future.

Lock-free queue ported from https://github.com/max0x7ba/atomic_queue