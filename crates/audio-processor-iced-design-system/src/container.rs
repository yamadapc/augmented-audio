pub use hover_container::HoverContainer;

/// Modified `iced_native::container::Container` to have styles on hover/pressed
pub mod hover_container {
    use iced::canvas::event::Status;
    use iced::{Align, Length, Point, Rectangle};
    use iced_native::layout::{Limits, Node};
    use iced_native::{
        layout, overlay, Clipboard, Element, Event, Hasher, Layout, Padding, Widget,
    };
    use std::hash::Hash;

    pub struct HoverContainer<'a, Message, Renderer: self::Renderer> {
        padding: Padding,
        content: Element<'a, Message, Renderer>,
        width: Length,
        height: Length,
        max_width: u32,
        max_height: u32,
        horizontal_alignment: Align,
        vertical_alignment: Align,
        style: Renderer::Style,
    }

    impl<'a, Message, Renderer> HoverContainer<'a, Message, Renderer>
    where
        Renderer: self::Renderer,
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
                horizontal_alignment: Align::Start,
                vertical_alignment: Align::Start,
                style: Renderer::Style::default(),
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
        pub fn align_x(mut self, alignment: Align) -> Self {
            self.horizontal_alignment = alignment;
            self
        }

        /// Sets the content alignment for the vertical axis of the [`Container`].
        pub fn align_y(mut self, alignment: Align) -> Self {
            self.vertical_alignment = alignment;
            self
        }

        /// Centers the contents in the horizontal axis of the [`Container`].
        pub fn center_x(mut self) -> Self {
            self.horizontal_alignment = Align::Center;
            self
        }

        /// Centers the contents in the vertical axis of the [`Container`].
        pub fn center_y(mut self) -> Self {
            self.vertical_alignment = Align::Center;
            self
        }

        /// Sets the style of the [`Container`].
        pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
            self.style = style.into();
            self
        }
    }

    impl<'a, Message, Renderer> Widget<Message, Renderer> for HoverContainer<'a, Message, Renderer>
    where
        Renderer: self::Renderer,
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

            layout::Node::with_children(size.pad(self.padding), vec![content])
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            defaults: &Renderer::Defaults,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
        ) -> Renderer::Output {
            renderer.draw(
                defaults,
                layout.bounds(),
                cursor_position,
                viewport,
                &self.style,
                &self.content,
                layout.children().next().unwrap(),
            )
        }

        fn hash_layout(&self, state: &mut Hasher) {
            struct Marker;
            std::any::TypeId::of::<Marker>().hash(state);

            self.padding.hash(state);
            self.width.hash(state);
            self.height.hash(state);
            self.max_width.hash(state);
            self.max_height.hash(state);

            self.content.hash_layout(state);
        }

        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            messages: &mut Vec<Message>,
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
        ) -> Option<overlay::Element<'_, Message, Renderer>> {
            self.content.overlay(layout.children().next().unwrap())
        }
    }

    pub trait Renderer: iced_native::Renderer {
        /// The style supported by this renderer.
        type Style: Default;

        /// Draws a [`Container`].
        fn draw<Message>(
            &mut self,
            defaults: &Self::Defaults,
            bounds: Rectangle,
            cursor_position: Point,
            viewport: &Rectangle,
            style: &Self::Style,
            content: &Element<'_, Message, Self>,
            content_layout: Layout<'_>,
        ) -> Self::Output;
    }

    impl<'a, Message, Renderer> From<HoverContainer<'a, Message, Renderer>>
        for Element<'a, Message, Renderer>
    where
        Renderer: 'a + self::Renderer,
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

        #[derive(Debug, Clone, Copy)]
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

    pub mod renderer {
        use iced::{Background, Color, Point, Rectangle};
        use iced_graphics::defaults::{self, Defaults};
        use iced_graphics::{Backend, Primitive};
        use iced_native::{Element, Layout};

        impl<B> super::Renderer for iced_graphics::Renderer<B>
        where
            B: Backend,
        {
            type Style = Box<dyn super::style::StyleSheet>;

            fn draw<Message>(
                &mut self,
                defaults: &Defaults,
                bounds: Rectangle,
                cursor_position: Point,
                viewport: &Rectangle,
                style_sheet: &Self::Style,
                content: &Element<'_, Message, Self>,
                content_layout: Layout<'_>,
            ) -> Self::Output {
                let style = if bounds.contains(cursor_position) {
                    style_sheet.hovered()
                } else {
                    style_sheet.style()
                };

                let defaults = Defaults {
                    text: defaults::Text {
                        color: style.text_color.unwrap_or(defaults.text.color),
                    },
                };

                let (content, mouse_interaction) =
                    content.draw(self, &defaults, content_layout, cursor_position, viewport);

                if let Some(background) = background(bounds, &style) {
                    (
                        Primitive::Group {
                            primitives: vec![background, content],
                        },
                        mouse_interaction,
                    )
                } else {
                    (content, mouse_interaction)
                }
            }
        }

        pub fn background(bounds: Rectangle, style: &super::style::Style) -> Option<Primitive> {
            if style.background.is_some() || style.border_width > 0.0 {
                Some(Primitive::Quad {
                    bounds,
                    background: style
                        .background
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                })
            } else {
                None
            }
        }
    }
}
