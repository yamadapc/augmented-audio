//! TODO - All of this could be replaced with a subscription :face-palm:
use iced::{Element, Length, Point, Rectangle};
use iced_native::event::Status;
use iced_native::layout::{Limits, Node};
use iced_native::widget::Widget;
use iced_native::{layout, Clipboard, Event, Hasher, Layout};

#[cfg(all(not(target_arch = "wasm32"), feature = "glow"))]
use iced_glow as renderer;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "glow"), feature = "wgpu"))]
use iced_wgpu as renderer;

type RendererOutput = <renderer::Renderer as iced_native::renderer::Renderer>::Output;
type RendererDefaults = <renderer::Renderer as iced_native::renderer::Renderer>::Defaults;

use super::Message;

pub struct FileDragAndDropHandler {}

impl FileDragAndDropHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(self) -> Element<'static, Message> {
        self.into()
    }
}

impl Widget<Message, renderer::Renderer> for FileDragAndDropHandler {
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &renderer::Renderer, _limits: &Limits) -> Node {
        layout::Node::default()
    }

    fn draw(
        &self,
        _renderer: &mut renderer::Renderer,
        _defaults: &RendererDefaults,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> RendererOutput {
        Default::default()
    }

    fn hash_layout(&self, _state: &mut Hasher) {}

    fn on_event(
        &mut self,
        event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _renderer: &renderer::Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> Status {
        if let Event::Window(window_event) = event {
            match window_event {
                iced_native::window::Event::FileHovered(path) => {
                    log::info!("Received file hovered {:?}", path);
                    messages.push(Message::FileHover(path));
                    Status::Captured
                }
                iced_native::window::Event::FileDropped(path) => {
                    log::info!("Received file drop {:?}", path);
                    messages.push(Message::FileDropped(path));
                    Status::Captured
                }
                _ => Status::Ignored,
            }
        } else {
            Status::Ignored
        }
    }
}

impl<'a> Into<Element<'a, Message>> for FileDragAndDropHandler {
    fn into(self) -> Element<'a, Message> {
        Element::new(self)
    }
}
