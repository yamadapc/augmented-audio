use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Instant;

use audio_garbage_collector::{make_shared, Shared};

use crate::file_io;

pub struct StreamingServer {}

impl Default for StreamingServer {
    fn default() -> Self {
        Self {}
    }
}

impl StreamingServer {
    pub fn open_file(
        &self,
        path: &str,
    ) -> (
        Shared<AtomicBool>,
        ringbuf::Consumer<symphonia::core::audio::AudioBuffer<f32>>,
    ) {
        let mut file = file_io::default_read_audio_file(path).unwrap();

        let audio_file_stream = file
            .format
            .default_track()
            .ok_or(file_io::AudioFileError::OpenStreamError)
            .unwrap();
        let mut decoder = symphonia::default::get_codecs()
            .make(&audio_file_stream.codec_params, &Default::default())
            .unwrap();
        let audio_file_stream_id = audio_file_stream.id;

        let channel = ringbuf::RingBuffer::new(20);
        let (mut tx, rx) = channel.split();

        let is_running = make_shared(AtomicBool::new(true));
        thread::spawn({
            let is_running = is_running.clone();
            move || {
                loop {
                    let mut pending_packets = std::collections::VecDeque::new();
                    let packet_read = Instant::now();

                    // On each iteration, we dequeue previously enqueued packets and try to push
                    // them in order. If any of them fails we requeue them and try again (no packets
                    // are written until writting to the queue is successful). Writing to the queue
                    // should fail on:
                    // * queue being full, consumer is slower than producer
                    let mut failed = false;
                    while let Some(packet) = pending_packets.pop_back() {
                        if let Err(packet) = tx.push(packet) {
                            pending_packets.push_back(packet);
                            failed = true;
                            break; // TODO - backoff
                        }
                    }

                    if failed {
                        continue; // TODO - backoff should be here;
                    }

                    // Read packet from file
                    let packet = file.format.next_packet().ok();
                    if packet.is_none() {
                        break;
                    }
                    let packet = packet.unwrap();
                    if packet.track_id() != audio_file_stream_id {
                        break;
                    }
                    let decoded = decoder.decode(&packet).ok();
                    if decoded.is_none() {
                        break;
                    }

                    // Push packet to consumer, on failure enqueue it on a thread local retry queue
                    let audio_buffer = decoded.unwrap();
                    let destination = file_io::convert_audio_buffer_sample_type(audio_buffer);

                    if let Err(buffer) = tx.push(destination) {
                        pending_packets.push_front(buffer);
                    }

                    log::debug!("Read packet in {:#?}", packet_read.elapsed());
                }

                is_running.store(false, Ordering::Relaxed);
            }
        });

        (is_running, rx)
    }
}

#[cfg(test)]
mod test {
    use audio_processor_traits::AudioProcessorSettings;

    use super::*;

    #[test]
    fn test_streaming_server_open_file() {
        wisual_logger::init_from_env();
        let path = format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../input-files/bass.mp3"
        );
        let server = StreamingServer::default();

        let (is_running, mut rx) = server.open_file(&path);
        let output_rate = AudioProcessorSettings::default().sample_rate();

        while is_running.load(Ordering::Relaxed) {
            if let Some(buffer) = rx.pop() {
                let _converted_channels: Vec<Vec<f32>> = (0..buffer.spec().channels.count())
                    .into_iter()
                    .map(|channel_number| {
                        file_io::convert_audio_file_sample_rate(
                            &buffer,
                            output_rate,
                            channel_number,
                        )
                    })
                    .collect();
            }
        }
    }
}
