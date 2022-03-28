use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

use std::time::Duration;

use basedrop::Shared;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;
use audio_processor_analysis::transient_detection::stft::markers::{
    build_markers, AudioFileMarker,
};
use audio_processor_analysis::transient_detection::stft::IterativeTransientDetectionParams;
use audio_processor_traits::{
    AudioBuffer, AudioProcessorSettings, OwnedAudioBuffer, VecAudioBuffer,
};

use crate::processor::handle::LooperClipRef;

#[derive(Clone)]
pub struct SliceResult {
    result: Shared<Vec<AudioFileMarker>>,
}

impl SliceResult {
    pub fn markers(&self) -> &Shared<Vec<AudioFileMarker>> {
        &self.result
    }
}

#[derive(Clone)]
struct SliceJob {
    id: usize,
    settings: AudioProcessorSettings,
    buffer: LooperClipRef,
}

struct SliceProcessorThread {
    job_queue: Shared<Queue<SliceJob>>,
    results: Shared<lockfree::map::Map<usize, SliceResult>>,
    is_running: Shared<AtomicBool>,
}

impl SliceProcessorThread {
    fn new(
        job_queue: Shared<Queue<SliceJob>>,
        results: Shared<lockfree::map::Map<usize, SliceResult>>,
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
        work_buffer.resize(1, buffer.num_samples(), 0.0);
        for (loop_frame, work_sample) in buffer.frames().zip(work_buffer.slice_mut()) {
            for loop_sample in loop_frame {
                *work_sample += loop_sample.get();
            }
        }

        // Run transient detection
        let params = IterativeTransientDetectionParams {
            fft_size: 2048,
            threshold_time_spread: 2,
            threshold_time_spread_factor: 1.5,
            power_of_change_spectral_spread: 2,
            iteration_count: 10,
            ..IterativeTransientDetectionParams::default()
        };
        let result = build_markers(&job.settings, work_buffer.slice_mut(), params, 0.05);
        let marker_count = result.len();
        let result = SliceResult {
            result: make_shared(result),
        };

        log::info!(
            "Finished slicing background job on loop buffer job_id={} marker_count={}",
            job.id,
            marker_count
        );
        let results = &self.results;
        results.insert(job.id, result);
    }
}

pub struct SliceWorker {
    job_queue: Shared<Queue<SliceJob>>,
    results: Shared<lockfree::map::Map<usize, SliceResult>>,
    is_running: Shared<AtomicBool>,
}

impl SliceWorker {
    pub fn new() -> Self {
        let job_queue = make_shared(Queue::new(10));
        let results = make_shared(lockfree::map::Map::new());
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
            std::thread::Builder::new()
                .name(String::from("looper_slice_worker"))
                .spawn(move || {
                    log::info!("Slice processor thread started");
                    let processor = SliceProcessorThread::new(job_queue, results, is_running);
                    log::info!("Slice processor thread exiting");
                    processor.run();
                })
                .unwrap();
        }
    }

    pub fn add_job(&self, id: usize, settings: AudioProcessorSettings, clip: LooperClipRef) {
        self.job_queue.push(SliceJob {
            id,
            settings,
            buffer: clip,
        });
    }

    pub fn result(&self, id: usize) -> Option<SliceResult> {
        self.results.get(&id).map(|entry| entry.val().clone())
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}

impl Drop for SliceWorker {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use atomic_refcell::AtomicRefCell;
    use augmented_atomics::AtomicF32;
    use itertools::Itertools;
    use std::time::Instant;

    #[test]
    fn test_we_can_start_the_worker() {
        let _ = wisual_logger::try_init_from_env();
        let mut worker = SliceWorker::new();
        worker.start();
    }

    #[test]
    fn test_we_can_post_jobs_into_the_worker() {
        let _ = wisual_logger::try_init_from_env();
        let mut worker = SliceWorker::new();
        worker.start();

        let buffer = VecAudioBuffer::from(
            vec![1.0, 0.0, 1.0, 0.0]
                .iter()
                .map(|f| AtomicF32::from(*f))
                .collect_vec(),
        );
        let buffer = AtomicRefCell::new(buffer);
        let buffer = make_shared(buffer);

        worker.add_job(0, AudioProcessorSettings::default(), buffer);

        let start = Instant::now();
        loop {
            let result = worker.result(0);
            if result.is_some() {
                break;
            }

            if start.elapsed().as_secs() > 2 {
                assert!(false, "Timed-out waiting for result");
                break;
            }
        }
    }
}
