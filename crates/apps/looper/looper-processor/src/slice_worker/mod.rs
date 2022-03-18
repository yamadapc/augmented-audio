use atomic_queue::Queue;
use audio_garbage_collector::make_shared;
use basedrop::Shared;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use audio_processor_analysis::transient_detection::stft::{
    find_transients, IterativeTransientDetectionParams,
};
use audio_processor_traits::{AudioBuffer, OwnedAudioBuffer, VecAudioBuffer};

use crate::processor::handle::LooperClipRef;

struct SliceResult {
    id: usize,
}

struct SliceJob {
    id: usize,
    buffer: LooperClipRef,
}

struct SliceProcessorThread {
    job_queue: Shared<Queue<SliceJob>>,
    result_queue: Shared<Queue<SliceResult>>,
    is_running: Shared<AtomicBool>,
}

impl SliceProcessorThread {
    fn new(
        job_queue: Shared<Queue<SliceJob>>,
        result_queue: Shared<Queue<SliceResult>>,
        is_running: Shared<AtomicBool>,
    ) -> Self {
        SliceProcessorThread {
            job_queue,
            result_queue,
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
        let _result = find_transients(
            IterativeTransientDetectionParams::default(),
            &mut work_buffer,
        );

        log::info!(
            "Finished slicing background job on loop buffer job_id={}",
            job.id
        );
        self.result_queue.push(SliceResult { id: job.id });
    }
}

struct SliceWorker {
    job_queue: Shared<Queue<SliceJob>>,
    result_queue: Shared<Queue<SliceResult>>,
    is_running: Shared<AtomicBool>,
}

impl SliceWorker {
    pub fn new() -> Self {
        let result_queue = make_shared(Queue::new(10));
        let job_queue = make_shared(Queue::new(10));
        let is_running = make_shared(AtomicBool::new(true));

        let s = Self {
            result_queue,
            job_queue,
            is_running,
        };
        s.start();
        s
    }

    fn start(&self) {
        {
            let result_queue = self.result_queue.clone();
            let job_queue = self.job_queue.clone();
            let is_running = self.is_running.clone();
            std::thread::spawn(move || {
                let processor = SliceProcessorThread {
                    result_queue,
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

    pub fn slice_result(&self) -> Option<SliceResult> {
        self.result_queue.pop()
    }
}

impl Drop for SliceWorker {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}
