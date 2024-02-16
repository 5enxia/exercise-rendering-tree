use crate::{
    css,
    dom::{Node, NodeType},
    javascript::{JavaScriptRuntime},
    layout::to_layout_box,
    render::{to_element_container, ElementContainer},
    style::to_styled_node,
};

use cursive::{
    direction::Orientation,
    event::{AnyCb, Event, EventResult},
    view::{Selector, View, ViewNotFound},
    Rect,
    Vec2,
    CbSink,
};

use std::{
    cell::RefCell,
    rc::Rc
};

pub struct Renderer {
    view: ElementContainer, // 表示中のView
    document_element: Rc<RefCell<Box<Node>>>, // DOMツリー
    js_runtime_instance: JavaScriptRuntime, // JavaScriptのランタイム
}