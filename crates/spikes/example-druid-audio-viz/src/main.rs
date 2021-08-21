// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of sending commands from another thread.
//! This is useful when you want to have some kind of
//! generated content (like here), or some task that just
//! takes a long time but don't want to block the main thread
//! (waiting on an http request, some cpu intensive work etc.)

use std::thread;
use std::time::Duration;

use druid::widget::prelude::*;
use druid::{AppLauncher, Color, Point, Selector, Target, WidgetExt, WindowDesc};

use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::audio_processor_start;

use crate::buffer_analyser::BufferAnalyserProcessor;
use atomic_queue::Queue;
use basedrop::Shared;
use circular_data_structures::CircularVec;
use druid::kurbo::BezPath;

mod buffer_analyser;

// If you want to submit commands to an event sink you have to give it some kind
// of ID. The selector is that, it also assures the accompanying data-type is correct.
// look at the docs for `Selector` for more detail.
const DRAW_AUDIO: Selector<Vec<f32>> = Selector::new("event-example.draw_audio");

pub fn main() {
    let window = WindowDesc::new(|| make_ui()).title("External Event Demo");

    let launcher = AppLauncher::with_window(window);

    // If we want to create commands from another thread `launcher.get_external_handle()`
    // should be used. For sending commands from within widgets you can always call
    // `ctx.submit_command`
    let event_sink = launcher.get_external_handle();

    let garbage_collector = GarbageCollector::default();
    let processor = BufferAnalyserProcessor::new(garbage_collector.handle());
    let queue_handle = processor.queue();
    let _audio_streams = audio_processor_start(processor);
    thread::spawn(move || generate_audio_updates(event_sink, queue_handle));

    launcher
        .launch(AudioData(Vec::new()))
        .expect("launch failed");
}

fn generate_audio_updates(event_sink: druid::ExtEventSink, queue_handle: Shared<Queue<f32>>) {
    let mut buffer = CircularVec::with_size(5 * 4410, 0.0);
    let mut position = 0;

    loop {
        while let Some(sample) = queue_handle.pop() {
            buffer[position] = sample;
            position += 1;
        }

        if event_sink
            .submit_command(DRAW_AUDIO, buffer.inner().clone(), Target::Auto)
            .is_err()
        {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

#[derive(Clone)]
struct AudioData(Vec<f32>);

impl Data for AudioData {
    fn same(&self, _other: &Self) -> bool {
        false
    }
}

/// A widget that displays a color.
struct AudioWave {}

impl Widget<AudioData> for AudioWave {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut AudioData, _env: &Env) {
        match event {
            // This is where we handle our command.
            Event::Command(cmd) if cmd.is(DRAW_AUDIO) => {
                // We don't do much data processing in the `event` method.
                // All we really do is just set the data. This causes a call
                // to `update` which requests a paint. You can also request a paint
                // during the event, but this should be reserved for changes to self.
                // For changes to `Data` always make `update` do the paint requesting.
                *data = AudioData(cmd.get_unchecked(DRAW_AUDIO).clone());
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AudioData,
        _: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AudioData, _data: &AudioData, _: &Env) {
        ctx.request_paint()
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &AudioData, _: &Env) -> Size {
        bc.max()
    }

    // This is of course super slow due to using CoreGraphics
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AudioData, _env: &Env) {
        ctx.clear(Color::BLACK);
        let AudioData(data) = data;
        if data.is_empty() {
            return;
        }

        let size = ctx.size();
        let mut prev = data[0];
        let mut index = 0;

        let mut shape = BezPath::new();
        let num_points = data.len();
        let width = size.width;
        let step = ((num_points as f64) / width) as usize;
        while index < data.len() {
            let item = data[index];
            let f_index = index as f64;
            let x_coord = (f_index / data.len() as f64) * size.width;
            let y_coord = (prev as f64) * size.height / 2.0 + size.height / 2.0;
            shape.move_to(Point::new(x_coord, y_coord));

            let mut draw = |item| {
                let x2_coord = ((f_index + 1.0) / data.len() as f64) * size.width;
                let y2_coord = (item as f64) * size.height / 2.0 + size.height / 2.0;
                shape.line_to(Point::new(x2_coord, y2_coord));
            };

            draw(item);
            draw(-item);

            prev = item;
            index += step;
        }
        ctx.stroke(shape, &Color::RED, 3.0);
    }
}

fn make_ui() -> impl Widget<AudioData> {
    AudioWave {}.expand().padding(10.0).center()
}
