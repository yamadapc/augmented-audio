# audio-garbage-collector

Batteries included solution to using reference counted values on the audio-thread.

Wraps `basedrop` so that smart pointers are dropped on a background thread. Exposes a default
global GC thread and helpers to create pointers attached to it.

## Collection frequency
Collection is based on polling the queue. If references are created and dropped very frequently
this will not be adequate.

License: MIT
