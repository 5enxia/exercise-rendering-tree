use crate::{
    css,
    dom::{Node, NodeType},
    javascript::{JavaScriptRuntime},
    javascript::renderapi::RendererAPI,
    layout::to_layout_box,
    render::{to_element_container, ElementContainer},
    style::to_styled_node,
};

use cursive::{
    direction::{Direction, Orientation}, event::{AnyCb, Event, EventResult}, view::{CannotFocus, Selector, View, ViewNotFound}, CbSink, Rect, Vec2
};

use std::{
    cell::RefCell,
    rc::Rc
};

pub struct Renderer {
    pub view: ElementContainer, // 表示中のView
    document_element: Rc<RefCell<Box<Node>>>, // DOMツリー
    js_runtime_instance: JavaScriptRuntime, // JavaScriptのランタイム
}

const DEFAULT_STYLESHEET: &str = r#"
script, style {
    display: none;
}
p, div {
    display: block;
}
"#;

pub fn collect_tag_inners(node: &Box<Node>, tag_name: &str) -> Vec<String> {
    if let NodeType::Element(ref el) = node.node_type {
        if el.tag_name.as_str() == tag_name {
            return vec![node.inner_text()];
        }
    }

    node.children
        .iter()
        .map(|child| collect_tag_inners(child, tag_name))
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect()
}

impl Renderer {
    pub fn new(ui_cb_sink: Rc<CbSink>, document_element: Box<Node>) -> Renderer {
        let stylesheet = css::parse(&format!(
            "{}\n{}",
            DEFAULT_STYLESHEET,
            collect_tag_inners(&document_element, "style".into()).join("\n")
        ));

        let view = to_styled_node(&document_element, &stylesheet)
            .and_then(|styled_node| Some(to_layout_box(styled_node)))
            .and_then(|layout_box| Some(to_element_container(layout_box)))
            .unwrap();

        let document_element = Rc::new(RefCell::new(document_element));
        let document_element_ref = document_element.clone();
        Renderer {
            document_element,
            view,
            // js_runtime_instance: JavaScriptRuntime::new(),
            js_runtime_instance: JavaScriptRuntime::new(
                document_element_ref,
                Rc::new(RendererAPI::new(ui_cb_sink)),
            ),
        }
    }

    // 再描画
    pub fn rerender(&mut self) {
        let document_element = self.document_element.borrow();
        let stylesheet = css::parse(&format!(
            "{}\n{}",
            DEFAULT_STYLESHEET,
            collect_tag_inners(&document_element, "style".into()).join("\n")
        ));
        let view = to_styled_node(&document_element, &stylesheet)
            .and_then(|styled_node| Some(to_layout_box(styled_node)))
            .and_then(|layout_box| Some(to_element_container(layout_box)))
            .unwrap();
        self.view = view;
    }

    // inlineスクリプトを実行
    pub fn execute_inline_scripts(&mut self) {
        let scripts = {
            let document_element = self.document_element.borrow();
            collect_tag_inners(&document_element, "script".into()).join("\n")
        };
        self.js_runtime_instance
            .execute("(inline)", &scripts.as_str())
            .unwrap();
    }
}

impl View for Renderer {
    fn draw(&self, printer: &cursive::Printer) {
        self.view.draw(printer)
    }

    fn layout(&mut self, v: Vec2) {
        self.view.layout(v)
    }

    fn needs_relayout(&self) -> bool {
        self.view.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.view.required_size(constraint)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        self.view.on_event(e)
    }

    fn call_on_any<'a>(&mut self, s: &Selector<'_>, cb: AnyCb<'a>) {
        self.view.call_on_any(s, cb)
    }

    fn focus_view(&mut self, s: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        self.view.focus_view(s)
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus>{
        self.view.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.view.important_area(view_size)
    }

    fn type_name(&self) -> &'static str {
        self.view.type_name()
    }
}
