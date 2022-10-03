// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::cell::{RefCell, RefMut};

use atomic_refcell::AtomicRefCell;
use basedrop::Shared;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;
use audio_processor_traits::{AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::AtomicF32;

use crate::audio::multi_track_looper::long_backoff::LongBackoff;
use crate::LooperId;

enum TrackEventsMessage {
    StoppedRecording {
        looper_id: LooperId,
        settings: Shared<AudioProcessorSettings>,
        looper_clip: Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
    },
}

pub struct TrackEventsWorker {
    tx: Shared<Queue<TrackEventsMessage>>,
}

impl TrackEventsWorker {
    pub fn new() -> Self {
        let queue = make_shared(Queue::new(10));
        let tx = queue.clone();
        std::thread::spawn(move || {
            let mut backoff = LongBackoff::new();
            loop {
                if let Some(msg) = queue.pop() {
                    println!("StoppedRecording message");
                    backoff.reset();
                } else {
                    backoff.snooze();
                }
            }
        });
        Self { tx }
    }

    pub fn on_stopped_recording(
        &self,
        looper_id: LooperId,
        settings: Shared<AudioProcessorSettings>,
        looper_clip: Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
    ) {
        let _ = self.tx.push(TrackEventsMessage::StoppedRecording {
            looper_id,
            looper_clip,
            settings,
        });
    }
}
