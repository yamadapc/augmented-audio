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
pub use hover_container::HoverContainer;

/// Modified `iced_native::container::Container` to have styles on hover/pressed
pub mod hover_container {
    use iced::{Alignment, Color, Length, Point, Rectangle};
    use iced::canvas::event::Status;
    use iced_native::{Clipboard, Element, Event, Layout, overlay, Padding, Shell, Widget};
    use iced_native::layout::{Limits, Node};
    use iced_native::renderer::Quad;

    pub struct HoverContainer<'a, Message, Renderer: iced_native::Renderer> {
        padding: Padding,
        content: Element<'a, Message, Renderer>,
        width: Length,
        height: Length,
        max_width: u32,
        max_height: u32,
        horizontal_alignment: Alignment,
        vertical_alignment: Alignment,
        style: Box<dyn self::style::StyleSheet>,
    }

    impl<'a, Message, Renderer> HoverContainer<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
    {
        pub fn new<T>(content: T) -> Self
        where
            T: Into<Element<'a, Message, Renderer>>,
        {
            HoverContainer {
                padding: Padding::ZERO,
                width: Length::Shrink,
                height: Length::Shrink,
                max_width: u32::MAX,
                max_height: u32::MAX,
                horizontal_alignment: Alignment::Start,
                vertical_alignment: Alignment::Start,
                style: Box::new(crate::style::HoverContainer::default()),
                content: content.into(),
            }
        }

        /// Sets the [`Padding`] of the [`Container`].
        pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
            self.padding = padding.into();
            self
        }

        /// Sets the width of the [`Container`].
        pub fn width(mut self, width: Length) -> Self {
            self.width = width;
            self
        }

        /// Sets the height of the [`Container`].
        pub fn height(mut self, height: Length) -> Self {
            self.height = height;
            self
        }

        /// Sets the maximum width of the [`Container`].
        pub fn max_width(mut self, max_width: u32) -> Self {
            self.max_width = max_width;
            self
        }

        /// Sets the maximum height of the [`Container`] in pixels.
        pub fn max_height(mut self, max_height: u32) -> Self {
            self.max_height = max_height;
            self
        }

        /// Sets the content alignment for the horizontal axis of the [`Container`].
        pub fn align_x(mut self, alignment: Alignment) -> Self {
            self.horizontal_alignment = alignment;
            self
        }

        /// Sets the content alignment for the vertical axis of the [`Container`].
        pub fn align_y(mut self, alignment: Alignment) -> Self {
            self.vertical_alignment = alignment;
            self
        }

        /// Centers the contents in the horizontal axis of the [`Container`].
        pub fn center_x(mut self) -> Self {
            self.horizontal_alignment = Alignment::Center;
            self
        }

        /// Centers the contents in the vertical axis of the [`Container`].
        pub fn center_y(mut self) -> Self {
            self.vertical_alignment = Alignment::Center;
            self
        }

        /// Sets the stylesheet of the [`Container`].
        pub fn style(mut self, stylesheet: impl Into<Box<dyn self::style::StyleSheet>>) -> Self {
            self.style = stylesheet.into();
            self
        }
    }

    impl<'a, Message, Renderer> Widget<Message, Renderer> for HoverContainer<'a, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
    {
        fn width(&self) -> Length {
            self.width
        }

        fn height(&self) -> Length {
            self.height
        }

        fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
            let limits = limits
                .loose()
                .max_width(self.max_width)
                .max_height(self.max_height)
                .width(self.width)
                .height(self.height)
                .pad(self.padding);

            let mut content = self.content.layout(renderer, &limits.loose());
            let size = limits.resolve(content.size());

            content.move_to(Point::new(
                self.padding.left.into(),
                self.padding.top.into(),
            ));
            content.align(self.horizontal_alignment, self.vertical_alignment, size);

            Node::with_children(size.pad(self.padding), vec![content])
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            style: &iced_native::renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
        ) {
            let is_hovered = layout.bounds().contains(cursor_position);
            let container_style = if is_hovered {
                self.style.hovered()
            } else {
                self.style.style()
            };
            renderer.fill_quad(
                Quad {
                    bounds: layout.bounds(),
                    border_radius: container_style.border_radius,
                    border_color: container_style.border_color,
                    border_width: container_style.border_width,
                },
                container_style
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );
            self.content.draw(
                renderer,
                &iced_native::renderer::Style {
                    text_color: container_style.text_color.unwrap_or(style.text_color),
                },
                layout.children().next().unwrap(),
                cursor_position,
                viewport,
            );
        }

        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            messages: &mut Shell<Message>,
        ) -> Status {
            self.content.on_event(
                event,
                layout.children().next().unwrap(),
                cursor_position,
                renderer,
                clipboard,
                messages,
            )
        }

        fn overlay(
            &mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
        ) -> Option<overlay::Element<'_, Message, Renderer>> {
            self.content
                .overlay(layout.children().next().unwrap(), renderer)
        }
    }

    impl<'a, Message, Renderer> From<HoverContainer<'a, Message, Renderer>>
        for Element<'a, Message, Renderer>
    where
        Renderer: 'a + iced_native::Renderer,
        Message: 'a,
    {
        fn from(
            container: HoverContainer<'a, Message, Renderer>,
        ) -> Element<'a, Message, Renderer> {
            Element::new(container)
        }
    }

    pub mod style {
        use iced::{Background, Color};

        #[derive(Debug, Clone)]
        pub struct Style {
            pub text_color: Option<Color>,
            pub background: Option<Background>,
            pub border_radius: f32,
            pub border_width: f32,
            pub border_color: Color,
        }

        impl std::default::Default for Style {
            fn default() -> Self {
                Self {
                    text_color: None,
                    background: None,
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            }
        }

        /// A set of rules that dictate the style of a container.
        pub trait StyleSheet {
            /// Produces the style of a container.
            fn style(&self) -> Style;

            fn hovered(&self) -> Style;
        }

        struct Default;

        impl StyleSheet for Default {
            fn style(&self) -> Style {
                Style {
                    text_color: None,
                    background: None,
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            }

            fn hovered(&self) -> Style {
                self.style()
            }
        }

        impl std::default::Default for Box<dyn StyleSheet> {
            fn default() -> Self {
                Box::new(Default)
            }
        }

        impl<T> From<T> for Box<dyn StyleSheet>
        where
            T: 'static + StyleSheet,
        {
            fn from(style: T) -> Self {
                Box::new(style)
            }
        }
    }
}
