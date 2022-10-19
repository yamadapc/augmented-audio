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

use skia_safe::{Canvas, Color4f, Paint, Rect, Size};
use taffy::prelude::Dimension;
use taffy::style::{FlexDirection, FlexboxLayout};

pub struct DrawContext<'a> {
    pub(crate) canvas: &'a mut Canvas,
}

pub fn render(canvas: &mut Canvas, size: Size, root: Element) {
    let mut layout_tree = taffy::Taffy::new();

    let node = layout_tree
        .new_leaf(FlexboxLayout {
            min_size: taffy::geometry::Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            max_size: taffy::geometry::Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            ..FlexboxLayout::DEFAULT
        })
        .unwrap();

    let root_node = layout_tree
        .new_with_children(
            FlexboxLayout {
                padding: taffy::geometry::Rect::from_points(80.0, 80.0, 80.0, 80.0),
                size: taffy::geometry::Size {
                    width: Dimension::Points(size.width),
                    height: Dimension::Points(size.height),
                },
                flex_direction: FlexDirection::Row,
                ..FlexboxLayout::DEFAULT
            },
            &[node],
        )
        .unwrap();
    layout_tree
        .compute_layout(
            root_node,
            taffy::geometry::Size {
                width: Some(size.width),
                height: Some(size.height),
            },
        )
        .unwrap();

    let layout = layout_tree.layout(node).unwrap();
    let origin = (layout.location.x, layout.location.y);
    let bounds = Rect::new(
        origin.0,
        origin.1,
        origin.0 + layout.size.width,
        origin.1 + layout.size.height,
    );

    let mut draw_context = DrawContext { canvas };
    root.widget.draw(&mut draw_context, bounds);
}

pub struct Element {
    widget: Box<dyn Widget>,
    _children: Vec<Element>,
}

impl<W: Widget + 'static> From<W> for Element {
    fn from(w: W) -> Self {
        Element {
            widget: Box::new(w),
            _children: vec![],
        }
    }
}

pub trait Widget {
    fn layout_style(&self) -> FlexboxLayout;
    fn draw(&self, ctx: &mut DrawContext, bounds: Rect);
}

#[derive(Default)]
pub struct Rectangle {}

impl Widget for Rectangle {
    fn layout_style(&self) -> FlexboxLayout {
        FlexboxLayout::DEFAULT
    }

    fn draw(&self, ctx: &mut DrawContext, bounds: Rect) {
        ctx.canvas
            .draw_rect(bounds, &Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None));
    }
}
