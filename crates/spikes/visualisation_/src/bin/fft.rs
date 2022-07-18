use std::ops::Deref;
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
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use nannou::{
    prelude::*,
    wgpu::{Device, LoadOp, TextureBuilder, TextureFormat, TextureView},
};

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_analysis::{
    fft_processor::FftProcessor,
    running_rms_processor::{RunningRMSProcessor, RunningRMSProcessorHandle},
};
use audio_processor_file::AudioFileProcessor;
use audio_processor_standalone::{
    standalone_processor::StandaloneOptions, StandaloneAudioOnlyProcessor, StandaloneHandles,
};
use audio_processor_traits::{
    audio_buffer, simple_processor, AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings,
    SimpleAudioProcessor, VecAudioBuffer,
};
use augmented_atomics::AtomicValue;

struct Handle {
    rms: Shared<RunningRMSProcessorHandle>,
    fft_buffer: Shared<VecAudioBuffer<AtomicF32>>,
    fft_buffer_changed: AtomicBool,
}

struct Processor {
    input_file: AudioFileProcessor,
    rms: RunningRMSProcessor,
    ffts: [FftProcessor; 2],
    handle: Shared<Handle>,
}

impl Processor {
    fn new() -> Self {
        let rms = RunningRMSProcessor::new_with_duration(
            audio_garbage_collector::handle(),
            Duration::from_millis(8),
        );

        let rms_handle = rms.handle().clone();
        let input_file = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            Default::default(),
            "./input.wav",
        )
        .unwrap();
        let fft = FftProcessor::default();
        let fft_size = fft.size();
        Self {
            input_file,
            ffts: [fft, FftProcessor::default()],
            rms,
            handle: make_shared(Handle {
                rms: rms_handle,
                fft_buffer: make_shared(VecAudioBuffer::empty_with(
                    2,
                    fft_size,
                    AtomicF32::new(0.0),
                )),
                fft_buffer_changed: AtomicBool::new(true),
            }),
        }
    }
}

impl AudioProcessor for Processor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.rms.s_prepare(settings);
        self.input_file.prepare(settings);
        for fft in &mut self.ffts {
            fft.s_prepare(settings);
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        audio_buffer::clear(data);
        self.input_file.process(data);
        simple_processor::process_buffer(&mut self.rms, data);
        for frame in data.frames_mut() {
            for (channel, sample) in frame.iter().enumerate() {
                let fft = &mut self.ffts[channel];

                fft.s_process(*sample);
                if fft.has_changed() {
                    for (fft_sample, output_frame) in
                        fft.buffer().iter().zip(self.handle.fft_buffer.frames())
                    {
                        output_frame[channel].set(fft_sample.norm());
                    }
                    self.handle.fft_buffer_changed.set(true);
                }
            }
        }
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    handle: Shared<Handle>,
    wgpu_model: WGPUModel,
    _audio_handles: StandaloneHandles,
}

fn model(app: &App) -> Model {
    let processor = Processor::new();
    let handle = processor.handle.clone();
    let audio_handles =
        audio_processor_standalone::standalone_start(StandaloneAudioOnlyProcessor::new(
            processor,
            StandaloneOptions {
                accepts_input: false,
                ..StandaloneOptions::default()
            },
        ));

    Model {
        handle,
        wgpu_model: wgpu_model(app),
        _audio_handles: audio_handles,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rms = { model.handle.rms.calculate_rms(0) + model.handle.rms.calculate_rms(1) };
    if model.handle.fft_buffer_changed.get() {
        let mut fft_slice = [0.0_f32; 4096];
        #[allow(clippy::needless_range_loop)]
        for i in 0..4096 {
            let f =
                model.handle.fft_buffer.get(0, i).get() + model.handle.fft_buffer.get(1, i).get();
            fft_slice[i] = f;
        }
        let storage = Storage { fft: fft_slice };
        app.main_window()
            .queue()
            .write_buffer(&model.wgpu_model.storage_buffer, 0, unsafe {
                wgpu::bytes::from_slice(&[storage])
            });
    }

    let uniform = Uniform {
        size: rms,
        dimensions: [app.window_rect().w(), app.window_rect().h()],
    };
    app.main_window()
        .queue()
        .write_buffer(&model.wgpu_model.uniform_buffer, 0, unsafe {
            wgpu::bytes::from_slice(&[uniform])
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let wgpu_model = &model.wgpu_model;
    {
        let mut encoder = frame.command_encoder();

        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(frame.texture_view(), |color| color.load_op(LoadOp::Load))
            .begin(&mut encoder);
        render_pass.set_bind_group(0, &wgpu_model.bind_group, &[]);
        render_pass.set_bind_group(1, &wgpu_model.uniform_bind_group, &[]);
        render_pass.set_bind_group(2, &wgpu_model.texture_bind_group, &[]);
        render_pass.set_pipeline(&wgpu_model.render_pipeline);
        render_pass.set_vertex_buffer(0, wgpu_model.vertex_buffer.slice(..));
        let vertex_range = 0..VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
    }

    let rms = { model.handle.rms.calculate_rms(0) + model.handle.rms.calculate_rms(1) };
    let draw = app.draw();
    draw.texture(&wgpu_model.texture)
        .x(rms * app.window_rect().w())
        .w(app.window_rect().w())
        .h(app.window_rect().h())
        .finish();
    draw.to_frame(app, &frame).unwrap();
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

const VERTICES: [Vertex; 6] = [
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniform {
    size: f32,
    dimensions: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Storage {
    fft: [f32; 4096],
}

struct WGPUModel {
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    storage_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    texture_bind_group: wgpu::BindGroup,
    texture: TextureView,
    _texture_sampler: wgpu::Sampler,
}

fn wgpu_model(app: &App) -> WGPUModel {
    // The gpu device associated with the window's swapchain
    let window = app.main_window();
    let device = window.device();
    let format = Frame::TEXTURE_FORMAT;
    let sample_count = 1; // window.msaa_samples();

    // Load shader modules.
    let vs_desc = wgpu::include_wgsl!("../shaders/vs.wgsl");
    let fs_desc = wgpu::include_wgsl!("../shaders/fs.wgsl");
    let vs_mod = device.create_shader_module(&vs_desc);
    let fs_mod = device.create_shader_module(&fs_desc);

    // Create the vertex buffer.
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsages::VERTEX;
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage,
    });

    let uniform = Uniform {
        size: 10.0,
        dimensions: [app.window_rect().w(), app.window_rect().h()],
    };
    let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: unsafe { wgpu::bytes::from_slice(&[uniform]) },
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let storage = Storage { fft: [0.0; 4096] };
    let storage_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: unsafe { wgpu::bytes::from_slice(&[storage]) },
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("uniform_bind_group_layout"),
        });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: storage_buffer.as_entire_binding(),
            },
        ],
        label: Some("uniform_bind_group"),
    });

    let bind_group_layout = wgpu::BindGroupLayoutBuilder::new().build(device);
    let bind_group = wgpu::BindGroupBuilder::new().build(device, &bind_group_layout);

    let (texture_bind_group, texture_bind_group_layout, texture, texture_sampler) =
        build_texture(&window, device, format, sample_count);

    let pipeline_layout = wgpu::create_pipeline_layout(
        device,
        Some("Pipeline layout"),
        &[
            &bind_group_layout,
            &uniform_bind_group_layout,
            &texture_bind_group_layout,
        ],
        &[],
    );

    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(4)
        .build(device);

    WGPUModel {
        bind_group,
        uniform_bind_group,
        uniform_buffer,
        storage_buffer,
        vertex_buffer,
        render_pipeline,
        texture_bind_group,
        texture,
        _texture_sampler: texture_sampler,
    }
}

fn build_texture(
    window: &Window,
    device: &Device,
    format: TextureFormat,
    sample_count: u32,
) -> (
    wgpu::BindGroup,
    wgpu::BindGroupLayout,
    wgpu::TextureView,
    wgpu::Sampler,
) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    });
    let texture = TextureBuilder::new()
        .sample_count(sample_count)
        .format(format)
        .size([window.rect().w() as u32, window.rect().h() as u32])
        .build(device)
        .view()
        .build();

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("texture_bind_group"),
    });
    (bind_group, bind_group_layout, texture, sampler)
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
