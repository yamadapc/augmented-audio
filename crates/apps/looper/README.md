# looper
This is a **WIP** implementation of a live looper.

<p align="center">
  <img src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/looper/screenshot.png" width="400" />
</p>

I want it to improve on existing loopers in the following areas:

* **Better quantization**
* Built-in multi-track support
* Manipulation of recorded loops

## Better quantization
Some of the digital loopers don't do proper tempo quantization.

For a musician using a looper, quantization should aid in recording loops in sync with some given tempo.

Why would a musician need aid to sync? Usually for live looping, this will be because there's some other loop or music
that is playing back at a certain tempo.

For example, there may be a drum track playing back. Additionally, if a looper is multi-track, there might be multiple
loops recorded at different points, these loops should have flexible lengths, but be synced with one another.

For example, one may record a 4 bar harmony loop and a 1 bar percussion loop. These two should have their lengths synced
so they are exactly a ratio of another & don't drift out-of-sync over time.

### How software loopers do this less great
Most software loopers on the market do the above. But the way they do it is bad for performers and different to that of
the great looper pedals.

If there's a tempo track playing and a musician presses to start a recording at beat 3.7, the looper should wait until
the 1st beat of the next bar. All loopers do this as expected. This is correcting for when the musician is **early**.

However, if the musician instead is **early**, a lot of loopers have no threshold for correction and always wait until
the next bar! This means if I press 100ms after the beat, the looper will wait until the next bar.

Of course this can be overcome with practice, however it is really inconvenient if this happens during a performance.

### How this looper will do quantization better
This looper provides two modes of quantization:

* SnapNext
* SnapClosest

The first, always pushes the action to the _next bar_ (as configured).

The second, instead, does time-traveling :). If a musician is late for an operation, including **starting a recording**,
the looper will correct back to the start of the bar, with a certain configurable threshold tolerance.

### Starting recording Xms ago
In order to do this, the looper is always recording. Starting to record is simply a state change & sets the current
starting cursor.

Currently, the specific mechanisms here need to be reviewed. As the looper should ideally also support undo-ing layers.
