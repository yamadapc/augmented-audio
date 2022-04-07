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
use cocoa::appkit::{NSColor, NSView, NSWindow};
use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSRect, NSString};
use objc::msg_send;
use stretch::Stretch;

use crate::color::Color;
use crate::component::{Component, Node, Props, RenderContext};

pub struct Renderer {
    target_view: id,
    stretch: Stretch,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            target_view: nil,
            stretch: Stretch::new(),
        }
    }

    pub fn new_with_window(window: id) -> Self {
        Self {
            target_view: unsafe { window.contentView() },
            stretch: Stretch::new(),
        }
    }

    pub fn render<C: Component>(&mut self, mut root: C) {
        let empty_props: Box<dyn Props + 'static> = Box::new(());
        let mut ctx = RenderContext::new(&empty_props).into();
        let node = root.render(ctx);

        self.render_node(&node)
    }

    fn render_node(&mut self, node: &Node) {
        match node {
            Node::Text { inner } => {
                self.render_text(inner);
            }
            Node::Box { color } => {
                self.render_box(color);
            }
            Node::Group { inner } => {
                self.render_children(inner);
            }
            _ => {}
        }
    }

    fn render_children(&mut self, children: &Vec<Box<Node>>) {
        for child in children {
            self.render_node(&child);
        }
    }

    fn render_box(&mut self, color: &Color) {
        log::info!("Rendering box {:?}", color);
        let view = self.target_view;
        unsafe {
            let frame: NSRect = msg_send![view, frame];
            let child: id = msg_send![class!(NSBox), alloc];
            let child: id = msg_send![child, initWithFrame: frame];
            let ns_color = NSColor::colorWithRed_green_blue_alpha_(
                nil,
                color.r as f64 / 255.,
                color.g as f64 / 255.,
                color.b as f64 / 255.,
                color.a as f64,
            );
            let _: id = msg_send![child, setBoxType: 4]; // NSBoxCustom
            let _: id = msg_send![child, setFillColor: ns_color];
            view.addSubview_(child);
        }
    }

    fn render_text(&mut self, text: &str) {
        let view = self.target_view;
        unsafe {
            let frame: NSRect = msg_send![view, frame];
            let instance: id = msg_send![class!(NSTextView), alloc];
            let text_view: id = msg_send![instance, initWithFrame: frame];
            let ns_text = NSString::alloc(nil).init_str(text);
            let _: id = msg_send![text_view, setString: ns_text];
            view.addSubview_(text_view);
        }
    }
}
