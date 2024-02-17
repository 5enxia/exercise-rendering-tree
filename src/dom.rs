use std::collections::HashMap;

use crate::html;
pub type AttrMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Box<Node>>,
}

impl Node {
    pub fn inner_text(&self) -> String {
        self.children
            .iter()
            .clone()
            .into_iter()
            .map(|node| match &node.node_type {
                NodeType::Text(text) => text.data.clone(),
                _ => node.inner_text(),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn inner_html(&self) -> String {
        self.children
            .iter()
            .clone()
            .into_iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn set_inner_html(&mut self, html: &str) {
        let nodes = html::parse_raw(html.into());
        self.children = nodes;
    }

    pub fn get_element_by_id<'a>(self: &'a mut Box<Node>, id: &str) -> Option<&'a mut Box<Node>> {
        match self.node_type {
            // HTML要素の場合
            NodeType::Element(ref el) => {
                // 指定したidがある場合はその要素を返す
                let has_element_id = el.id().map(|element_id| element_id.to_string() == id).unwrap_or(false);
                if has_element_id {
                    return Some(self);
                }
            },
            // テキストの場合は何もしない
            _ => (),
        };
        self.children
            .iter_mut()
            .find_map(|child| child.get_element_by_id(id))
    }
}

// Implement to_string for Node
impl ToString for Node {
    fn to_string(&self) -> String {
        match self.node_type {
            // html要素の場合は再帰的に処理する
            NodeType::Element(ref el) => {
                // attributes to string
                let attrs = el.attributes
                    .iter()
                    .clone()
                    .into_iter()
                    .map(|(key, value)| {
                        format!("{}=\"{}\"", key, value)
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                let children = self.children
                    .iter()
                    .clone()
                    .into_iter()
                    .map(|node| node.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                // attributesがない時
                if attrs.is_empty() {
                    // <tag_name>children</tag_name>
                    format!("<{}>{}</{}>", el.tag_name, children, el.tag_name)
                } else {
                    // <tag_name attributes>children</tag_name>
                    format!("<{} {}>{}</{}>", el.tag_name, attrs, children, el.tag_name)
                }
            },
            // テキストの場合はそのまま文字列を返す
            NodeType::Text(ref text) => text.data.clone()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Element(Element),
    Text(Text),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl Element {
    pub fn new(name: String, attributes: AttrMap, children: Vec<Box<Node>>) -> Box<Node> {
        Box::new(Node {
            node_type: NodeType::Element(Element {
                tag_name: name,
                attributes: attributes,
            }),
            children,
        })
    }

    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
}

#[derive(Debug, PartialEq)]
pub struct Text {
    pub data: String,
}

impl Text {
    pub fn new(text: String) -> Box<Node> {
        Box::new(Node {
            node_type: NodeType::Text(Text { data: text }),
            children: vec![],
        })
    }
}
