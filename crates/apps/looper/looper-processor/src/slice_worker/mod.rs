use atomic_queue::Queue;
use audio_garbage_collector::{make_shared, make_shared_cell};
use basedrop::{Shared, SharedCell};
use im::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use audio_processor_analysis::transient_detection::stft::{
    find_transients, IterativeTransientDetectionParams,
};
use audio_processor_traits::{AudioBuffer, OwnedAudioBuffer, VecAudioBuffer};

use crate::processor::handle::LooperClipRef;

#[derive(Clone)]
pub struct SliceResult {
    id: usize,
    result: Shared<Vec<f32>>,
}

#[derive(Clone)]
struct SliceJob {
    id: usize,
    buffer: LooperClipRef,
}

struct SliceProcessorThread {
    job_queue: Shared<Queue<SliceJob>>,
    results: Shared<SharedCell<im::HashMap<usize, SliceResult>>>,
    is_running: Shared<AtomicBool>,
}

impl SliceProcessorThread {
    fn new(
        job_queue: Shared<Queue<SliceJob>>,
        results: Shared<SharedCell<im::HashMap<usize, SliceResult>>>,
        is_running: Shared<AtomicBool>,
    ) -> Self {
        SliceProcessorThread {
            job_queue,
            results,
            is_running,
        }
    }

    fn run(&self) {
        while self.is_running.load(Ordering::Relaxed) {
            if let Some(slice) = self.job_queue.pop() {
                self.process_job(slice);
            }

            std::thread::sleep(Duration::from_millis(100))
        }
    }

    fn process_job(&self, job: SliceJob) {
        log::info!(
            "Running slicing background job on loop buffer job_id={}",
            job.id
        );
        // Copy buffer from shared reference onto working copy
        let buffer = job.buffer.deref();
        let buffer = buffer.borrow();
        let buffer = buffer.deref();

        let mut work_buffer = VecAudioBuffer::new();
        work_buffer.resize(buffer.num_channels(), buffer.num_samples(), 0.0);
        for (loop_frame, work_frame) in buffer.frames().zip(work_buffer.frames_mut()) {
            for (loop_sample, work_sample) in loop_frame.iter().zip(work_frame.iter_mut()) {
                *work_sample = loop_sample.get();
            }
        }

        // Run transient detection
        let result = find_transients(
            IterativeTransientDetectionParams::default(),
            &mut work_buffer,
        );
        let result = SliceResult {
            id: job.id,
            result: make_shared(result),
        };

        log::info!(
            "Finished slicing background job on loop buffer job_id={}",
            job.id
        );
        let results = self.results.get();
        let mut results = results.deref().clone();
        results.insert(job.id, result);
        self.results.set(make_shared(results));
    }
}

pub struct SliceWorker {
    job_queue: Shared<Queue<SliceJob>>,
    results: Shared<SharedCell<im::HashMap<usize, SliceResult>>>,
    is_running: Shared<AtomicBool>,
}

impl SliceWorker {
    pub fn new() -> Self {
        let job_queue = make_shared(Queue::new(10));
        let results = make_shared(make_shared_cell(HashMap::new()));
        let is_running = make_shared(AtomicBool::new(true));

        let s = Self {
            results,
            job_queue,
            is_running,
        };
        s.start();
        s
    }

    fn start(&self) {
        {
            let results = self.results.clone();
            let job_queue = self.job_queue.clone();
            let is_running = self.is_running.clone();
            std::thread::spawn(move || {
                let processor = SliceProcessorThread {
                    results,
                    job_queue,
                    is_running,
                };
                processor.run();
            })
        };
    }

    pub fn add_job(&self, id: usize, clip: LooperClipRef) {
        self.job_queue.push(SliceJob { id, buffer: clip });
    }

    pub fn result(&self, id: usize) -> Option<SliceResult> {
        let results = self.results.get();
        results.get(&id).cloned()
    }
}

impl Drop for SliceWorker {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}
