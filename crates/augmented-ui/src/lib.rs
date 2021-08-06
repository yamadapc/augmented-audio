#![feature(type_name_of_val)]

use std::any::{Any, TypeId};
use std::io::BufWriter;
use xml::writer::XmlEvent;

#[derive(Debug)]
pub enum Tag {
    TypeName(&'static str),
    Group,
    Text,
}

impl Tag {
    fn to_string(&self) -> String {
        match self {
            Tag::TypeName(str) => String::from(*str),
            Tag::Text => "Text".into(),
            Tag::Group => "Group".into(),
        }
    }
}

pub struct RenderContext<'a> {
    props: &'a Box<dyn Props>,
}

impl<'a> RenderContext<'a> {
    pub fn new(props: &'a Box<dyn Props>) -> Self {
        RenderContext { props }
    }
}

impl<'a> RenderContext<'a> {
    pub fn props<T: 'static + Clone>(&self) -> T {
        let t_ref: &T = self.props.any_props().downcast_ref().unwrap();
        t_ref.clone()
    }
}

pub trait Component {
    fn tag(&self) -> Tag;
    fn render(&self, ctx: Box<RenderContext>) -> Node;
}

impl<F: 'static> Component for F
where
    F: Fn(Box<RenderContext>) -> Node,
{
    fn tag(&self) -> Tag {
        Tag::TypeName(std::any::type_name_of_val(self))
    }

    fn render(&self, ctx: Box<RenderContext>) -> Node {
        self(ctx)
    }
}

pub trait Props {
    fn any_props(&self) -> &dyn Any;
}

impl Props for () {
    fn any_props(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl Props for String {
    fn any_props(&self) -> &dyn Any {
        self as &dyn Any
    }
}

pub enum Node {
    Text {
        inner: String,
    },
    Component {
        inner: Box<dyn Component>,
        props: Box<dyn Props>,
    },
    Group {
        inner: Vec<Box<Node>>,
    },
}

impl Node {
    pub fn tag(&self) -> Tag {
        match self {
            Node::Text { .. } => Tag::Text,
            Node::Component { inner, .. } => inner.tag(),
            Node::Group { .. } => Tag::Group,
        }
    }

    pub fn children(&self) -> Option<&Vec<Box<Node>>> {
        match self {
            Node::Group { inner } => Some(inner),
            _ => None,
        }
    }

    pub fn text<Str: Into<String>>(text: Str) -> Self {
        Self::Text { inner: text.into() }
    }

    pub fn group(children: Vec<Box<Node>>) -> Self {
        Self::Group { inner: children }
    }

    pub fn child<C: 'static + Component>(child: C) -> Self {
        Self::Component {
            inner: Box::new(child),
            props: Box::new(()),
        }
    }

    pub fn child_with<C: 'static + Component, P: 'static + Props>(child: C, props: P) -> Self {
        Self::Component {
            inner: Box::new(child),
            props: Box::new(props),
        }
    }
}

impl<Str: Into<String>> From<Str> for Node {
    fn from(str: Str) -> Node {
        Node::text(str)
    }
}

pub fn shallow_render_to_xml<C: Component>(mut root: C) -> String {
    use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

    let bytes = Vec::new();
    let buf_sink = BufWriter::new(bytes);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(buf_sink);

    let root_tag = root.tag();
    let root_tag_str = root_tag.to_string();

    let event: XmlEvent = XmlEvent::start_element(&*root_tag_str).into();
    writer.write(event);
    log::info!("Rendering {:?}", root_tag_str);

    {
        let empty_props: Box<dyn Props + 'static> = Box::new(());
        let mut ctx = RenderContext::new(&empty_props).into();
        let node = root.render(ctx);
        let node_tag_str = node.tag().to_string();
        let event: XmlEvent = XmlEvent::start_element(&*node_tag_str).into();
        writer.write(event);
    }

    let event: XmlEvent = XmlEvent::end_element().into();
    writer.write(event);
    let event: XmlEvent = XmlEvent::end_element().into();
    writer.write(event);

    String::from_utf8(writer.into_inner().buffer().to_owned()).unwrap()
}

pub fn do_xml_render<W: std::io::Write>(writer: &mut xml::writer::EventWriter<W>, node: &Node) {
    // Early exit for groups
    if let Some(children) = node.children() {
        for child in children {
            do_xml_render(writer, child);
        }
        return;
    }

    let tag_str = node.tag().to_string();
    let event: XmlEvent = XmlEvent::start_element(&*tag_str).into();
    writer.write(event);

    if let Node::Component { inner, props } = node {
        let mut ctx = RenderContext::new(props).into();
        let child = inner.render(ctx);
        do_xml_render(writer, &child);
    }

    let event: XmlEvent = XmlEvent::end_element().into();
    writer.write(event);
}

pub fn render_to_xml<C: Component>(mut root: C) -> String {
    use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

    let bytes = Vec::new();
    let buf_sink = BufWriter::new(bytes);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(buf_sink);

    let root_tag = root.tag();
    let root_tag_str = root_tag.to_string();

    let event: XmlEvent = XmlEvent::start_element(&*root_tag_str).into();
    writer.write(event);
    log::info!("Rendering {:?}", root_tag_str);

    {
        let empty_props: Box<dyn Props + 'static> = Box::new(());
        let mut ctx = RenderContext::new(&empty_props).into();
        let node = root.render(ctx);
        do_xml_render(&mut writer, &node);
    }

    let event: XmlEvent = XmlEvent::end_element().into();
    writer.write(event);

    String::from_utf8(writer.into_inner().buffer().to_owned()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_render_hello_world() {
        let _ = wisual_logger::try_init_from_env();

        fn app(_: Box<RenderContext>) -> Node {
            "Hello world".into()
        }

        let output = shallow_render_to_xml(app);
        log::info!(
            "Basic output ==================================================:\n\n{}\n\n",
            output
        );
    }

    #[test]
    fn it_can_shallow_render_nested_containers() {
        let _ = wisual_logger::try_init_from_env();

        fn container(_: Box<RenderContext>) -> Node {
            "Hello world".into()
        }

        fn app(_: Box<RenderContext>) -> Node {
            Node::child(container)
        }

        let output = shallow_render_to_xml(app);
        log::info!(
            "Shallow nested output =========================================:\n\n{}\n\n",
            output
        );
    }

    #[test]
    fn it_can_deep_render_nested_containers() {
        let _ = wisual_logger::try_init_from_env();

        fn container(_: Box<RenderContext>) -> Node {
            "Hello world".into()
        }

        fn app(_: Box<RenderContext>) -> Node {
            Node::child(container)
        }

        let output = render_to_xml(app);
        log::info!(
            "Deep nested output ============================================:\n\n{}\n\n",
            output
        );
    }

    #[test]
    fn it_can_deep_render_lists() {
        let _ = wisual_logger::try_init_from_env();

        fn text(_: Box<RenderContext>) -> Node {
            "Hello world".into()
        }

        fn container(_: Box<RenderContext>) -> Node {
            Node::group(vec![
                Node::child(text).into(),
                Node::text("Something").into(),
                Node::text("Other").into(),
                Node::child(text).into(),
            ])
        }

        fn app(_: Box<RenderContext>) -> Node {
            Node::child(container)
        }

        let output = render_to_xml(app);
        log::info!(
            "Deep nested output ============================================:\n\n{}\n\n",
            output
        );
    }

    #[test]
    fn children_can_have_props() {
        let _ = wisual_logger::try_init_from_env();

        fn text(ctx: Box<RenderContext>) -> Node {
            ctx.props::<String>().into()
        }

        fn container(ctx: Box<RenderContext>) -> Node {
            Node::group(vec![Node::child_with(text, ctx.props::<String>()).into()])
        }

        fn app(_: Box<RenderContext>) -> Node {
            Node::child_with(container, String::from("Hello world"))
        }

        let output = render_to_xml(app);
        log::info!(
            "Props output ==================================================:\n\n{}\n\n",
            output
        );
    }
}
