use crate::{
    dom::NodeType,
    layout::{BoxProps, BoxType, LayoutBox},
};
use cursive::{
    view::{IntoBoxedView, View, ViewWrapper},
    views::{DummyView, LinearLayout, Panel, TextView},
};

pub type ElementContainer = Box<dyn View>;

pub fn new_element_container() -> ElementContainer {
    (DummyView {}).into_boxed_view()
}

pub fn to_element_container<'a>(layout: LayoutBox<'a>) -> ElementContainer {
    match layout.box_type {
        BoxType::BlockBox(props) | BoxType::InlineBox(props) => {
            match props {
                // Element
                BoxProps {
                    node_type: NodeType::Element(ref el),
                    ..
                } => {
                    let mut panel = Panel::new(LinearLayout::vertical()).title(el.tag_name.clone());
                    match el.tag_name.as_str() {
                        _ => {
                            for child in layout.children.into_iter() {
                                panel.with_view_mut(|view| view.add_child(to_element_container(child)));
                            }
                        }
                    };

                    panel.into_boxed_view()
                },
                // Text
                BoxProps {
                    node_type: NodeType::Text(ref text),
                    ..
                } => {
                    let text_to_display = text.data.clone();
                    let text_to_display = text_to_display.replace("\n", "");
                    let text_to_display = text_to_display.trim();
                    // テキストが空の場合はダミーViewを返す
                    if text_to_display.is_empty() {
                        (DummyView {}).into_boxed_view()
                    } else {
                        TextView::new(text_to_display).into_boxed_view()
                    }
                }
            }
        },
        BoxType::AnonymousBox =>  {
            let mut panel = Panel::new(LinearLayout::horizontal());
            for child in layout.children.into_iter() {
                panel.with_view_mut(|view| view.add_child(to_element_container(child)));
            }
            panel.into_boxed_view()
        }
    }
}