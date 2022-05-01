use std::sync::atomic::AtomicBool;
use std::time::Duration;

use nannou::prelude::*;
use nannou::wgpu::LoadOp;
use smooth_value::InterpolatedValue;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_analysis::{
    fft_processor::FftProcessor,
    running_rms_processor::{RunningRMSProcessor, RunningRMSProcessorHandle},
};
use audio_processor_file::AudioFileProcessor;
use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{StandaloneAudioOnlyProcessor, StandaloneHandles};
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
            "./input.mp3",
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

struct Particle {
    x: f32,
    y: f32,
    size: f32,
    interpolated_size: InterpolatedValue,
}

struct Model {
    handle: Shared<Handle>,
    particles: Vec<Particle>,
    wgpu_model: WGPUModel,
    _audio_handles: StandaloneHandles,
}

fn model(app: &App) -> Model {
    let processor = Processor::new();
    let handle = processor.handle.clone();
    let audio_handles = audio_processor_standalone::standalone_start(
        StandaloneAudioOnlyProcessor::new(
            processor,
            StandaloneOptions {
                accepts_input: false,
                ..StandaloneOptions::default()
            },
        ),
        None,
    );

    let particles = (0..100)
        .map(|i| {
            (0..100)
                .map(|j| Particle {
                    x: -app.window_rect().w() / 2.0 + app.window_rect().w() * (i as f32) / 100.0,
                    y: -app.window_rect().h() / 2.0 + app.window_rect().h() * (j as f32) / 100.0,
                    interpolated_size: InterpolatedValue::new(60.0, Duration::from_millis(5), 10.0),
                    size: 1.0,
                })
                .collect::<Vec<Particle>>()
        })
        .flatten()
        .collect();

    Model {
        handle,
        particles,
        wgpu_model: wgpu_model(app),
        _audio_handles: audio_handles,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rms = { model.handle.rms.calculate_rms(0) + model.handle.rms.calculate_rms(1) };
    if model.handle.fft_buffer_changed.get() {
        let interpolated_fft_magnitudes = (0..model.particles.len())
            .map(|i| {
                let index = (i as f32 / model.particles.len() as f32)
                    * model.handle.fft_buffer.num_samples() as f32;

                model.handle.fft_buffer.get(0, index as usize).get()
                    + model.handle.fft_buffer.get(1, index as usize).get()
            })
            .collect::<Vec<f32>>();
        for (particle, magnitude) in model.particles.iter_mut().zip(interpolated_fft_magnitudes) {
            particle.interpolated_size.set(1.0 * magnitude.sqrt());
            particle.size = particle.interpolated_size.next_sample();
        }

        let mut fft_slice = [0.0_f32; 4096];
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
    {
        let mut encoder = frame.command_encoder();
        let wgpu_model = &model.wgpu_model;

        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(frame.texture_view(), |color| {
                color.load_op(LoadOp::Load).store_op(true)
            })
            .begin(&mut encoder);
        render_pass.set_bind_group(0, &wgpu_model.bind_group, &[]);
        render_pass.set_bind_group(1, &wgpu_model.uniform_bind_group, &[]);
        render_pass.set_pipeline(&wgpu_model.render_pipeline);
        render_pass.set_vertex_buffer(0, wgpu_model.vertex_buffer.slice(..));

        let vertex_range = 0..VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
    }
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
}

fn wgpu_model(app: &App) -> WGPUModel {
    // The gpu device associated with the window's swapchain
    let window = app.main_window();
    let device = window.device();
    let format = Frame::TEXTURE_FORMAT;
    let sample_count = window.msaa_samples();

    // Load shader modules.
    let vs_desc = wgpu::include_wgsl!("shaders/vs.wgsl");
    let fs_desc = wgpu::include_wgsl!("shaders/fs.wgsl");
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

    let pipeline_layout = wgpu::create_pipeline_layout(
        device,
        Some("Pipeline layout"),
        &[&bind_group_layout, &uniform_bind_group_layout],
        &[],
    );

    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .build(device);

    WGPUModel {
        bind_group,
        uniform_bind_group,
        uniform_buffer,
        storage_buffer,
        vertex_buffer,
        render_pipeline,
    }
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}