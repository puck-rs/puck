use std::collections::HashMap;

use malvolio::prelude::BodyNode;

use crate::dom::{element::Element, listener::ListenerRef};

use self::id::IdGen;

pub mod id;

#[derive(Debug)]
#[must_use]
pub struct WrappedBodyNode {
    node: BodyNode,
    listeners: Vec<ListenerRef>,
    children: Vec<WrappedBodyNode>,
}

macro_rules! map_heading_to_element {
    ($self:ident, $id:expr, $h:ident) => {{
        let $h = $h.into_pub_fields();
        Element {
            id: $id,
            name: std::borrow::Cow::Borrowed(stringify!($h)),
            attributes: $h.attrs,
            listeners: $self.listeners,
            // headings currently can't have children; this will be rectified in the future
            children: vec![],
            text: Some($h.text),
            key: None,
        }
    }};
}

impl WrappedBodyNode {
    pub fn into_element(self, id_gen: &mut IdGen) -> Element {
        match self.node {
            BodyNode::H1(h1) => {
                map_heading_to_element!(self, id_gen.new_id(), h1)
            }
            BodyNode::H2(h2) => {
                map_heading_to_element!(self, id_gen.new_id(), h2)
            }
            BodyNode::H3(h3) => {
                map_heading_to_element!(self, id_gen.new_id(), h3)
            }
            BodyNode::H4(h4) => {
                map_heading_to_element!(self, id_gen.new_id(), h4)
            }
            BodyNode::H5(h5) => {
                map_heading_to_element!(self, id_gen.new_id(), h5)
            }
            BodyNode::H6(h6) => {
                map_heading_to_element!(self, id_gen.new_id(), h6)
            }
            BodyNode::P(p) => {
                let p = p.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("p"),
                    attributes: p.attrs,
                    listeners: self.listeners,
                    children: self
                        .children
                        .into_iter()
                        .map(|child| child.into_element(id_gen))
                        .collect(),
                    text: Some(p.text),
                    key: None,
                }
            }
            BodyNode::Form(form) => {
                let form = form.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("form"),
                    attributes: form.attrs,
                    listeners: self.listeners,
                    children: self
                        .children
                        .into_iter()
                        .map(|child| child.into_element(id_gen))
                        .collect(),
                    text: None,
                    key: None,
                }
            }
            BodyNode::Br(_) => Element {
                id: id_gen.new_id(),
                name: std::borrow::Cow::Borrowed("br"),
                attributes: HashMap::new(),
                listeners: vec![],
                children: vec![],
                text: None,
                key: None,
            },
            BodyNode::Div(div) => {
                let div = div.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("div"),
                    attributes: div.attrs,
                    listeners: self.listeners,
                    children: self
                        .children
                        .into_iter()
                        .map(|child| child.into_element(id_gen))
                        .collect(),
                    text: None,
                    key: None,
                }
            }
            BodyNode::A(a) => {
                let a = a.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("a"),
                    attributes: a.attrs,
                    listeners: self.listeners,
                    children: vec![],
                    text: Some(a.text),
                    key: None,
                }
            }
            BodyNode::Input(input) => {
                let input = input.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("input"),
                    attributes: input.attrs,
                    listeners: self.listeners,
                    children: self
                        .children
                        .into_iter()
                        .map(|child| child.into_element(id_gen))
                        .collect(),
                    text: None,
                    key: None,
                }
            }
            BodyNode::Label(label) => {
                map_heading_to_element!(self, id_gen.new_id(), label)
            }
            BodyNode::Select(select) => {
                let select = select.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("div"),
                    attributes: select.attrs,
                    listeners: self.listeners,
                    children: self
                        .children
                        .into_iter()
                        .map(|child| child.into_element(id_gen))
                        .collect(),
                    text: None,
                    key: None,
                }
            }
            // not very useful given that Puck is entirely dependent on Javascript, but hey.
            BodyNode::NoScript(noscript) => {
                let noscript = noscript.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("div"),
                    attributes: HashMap::new(),
                    listeners: vec![],
                    children: vec![],
                    text: Some(noscript.text),
                    key: None,
                }
            }
            // todo: this should be fixed
            BodyNode::Text(_) => panic!(""),
            BodyNode::Img(img) => {
                let img = img.into_pub_fields();
                Element {
                    id: id_gen.new_id(),
                    name: std::borrow::Cow::Borrowed("img"),
                    attributes: img.attrs,
                    listeners: self.listeners,
                    children: vec![],
                    text: None,
                    key: None,
                }
            }
        }
    }

    pub fn listener(mut self, listener: impl Into<ListenerRef>) -> WrappedBodyNode {
        self.listeners.push(listener.into());
        self
    }

    pub fn child(mut self, child: impl Into<WrappedBodyNode>) -> WrappedBodyNode {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = WrappedBodyNode>,
    ) -> WrappedBodyNode {
        self.children.extend(children);
        self
    }
}

pub trait IntoWrappedBodyNode {
    fn wrap(self) -> WrappedBodyNode;
}

impl<T> IntoWrappedBodyNode for T
where
    T: Into<BodyNode>,
{
    fn wrap(self) -> WrappedBodyNode {
        WrappedBodyNode {
            node: self.into(),
            listeners: vec![],
            children: vec![],
        }
    }
}

#[cfg(test)]
#[lunatic::test]
fn test_html_conversion() {
    use malvolio::prelude::*;

    let tree = Div::new();
    let output = tree.wrap().into_element(&mut IdGen::new());
    assert_eq!(&format!("{:?}", output), include_str!("tree"));

    let bigger_tree = Div::new().wrap().child(H1::new("Heading 1").wrap()).child(
        Input::new()
            .attribute(Type::Submit)
            .wrap()
            .listener(ListenerRef::new("a_listener", "click")),
    );
    let output = bigger_tree.into_element(&mut IdGen::new());
    assert_eq!(&format!("{:?}", output), include_str!("bigger_tree"));

    let id_not_starting_from_zero = Form::new()
        .wrap()
        .child(Input::new().attribute(Type::Text).wrap())
        .child(Input::new().attribute(Type::Submit).wrap());
    let mut idgen = IdGen::new();
    idgen.new_id();
    let output = id_not_starting_from_zero.into_element(&mut idgen);
    assert_eq!(
        &format!("{:?}", output),
        include_str!("id_not_starting_from_zero")
    );
}
