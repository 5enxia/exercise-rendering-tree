
use crate::style::{Display, PropertyMap, StyledNode};
use crate::dom::NodeType;

#[derive(Debug, PartialEq)]
pub struct LayoutBox<'a> {
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum BoxType<'a> {
    BlockBox(BoxProps<'a>),
    InlineBox(BoxProps<'a>),
    AnonymousBox,
}

#[derive(Debug, PartialEq)]
pub struct BoxProps<'a> {
    pub node_type: &'a NodeType,
    pub properties: PropertyMap,
}

pub fn to_layout_box<'a>(snode: StyledNode<'a>) -> LayoutBox<'a> {
    // box typeを判定
    let display_type = snode.display();
    let box_type = match display_type {
        Display::Block => {
            BoxType::BlockBox(BoxProps {
                node_type: snode.node_type,
                properties: snode.properties
            })
        },
        Display::Inline => {
            BoxType::InlineBox(BoxProps {
                node_type: snode.node_type,
                properties: snode.properties
            })
        },
        Display::None => unreachable!("Root node has no display type"),
    };

    LayoutBox {
        box_type: box_type,
        children: vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::{css::CSSValue, dom::Element};

    use super::*;

    #[test]
    fn test_to_layout_box() {
        let block = [(
            "display".to_string(),
            CSSValue::Keyword("block".to_string()),
        )];
        let inline = [(
            "display".to_string(),
            CSSValue::Keyword("inline".to_string()),
        )];

        let node = NodeType::Element(Element {
            tag_name: "div".into(),
            attributes: [].iter().cloned().collect(),
        });
        let snode = StyledNode {
            node_type: &node,
            properties: block.iter().cloned().collect(),
            children: vec![
                StyledNode {
                    node_type: &node,
                    properties: block.iter().cloned().collect(),
                    children: vec![],
                },
                StyledNode {
                    node_type: &node,
                    properties: inline.iter().cloned().collect(),
                    children: vec![
                        StyledNode {
                            node_type: &node,
                            properties: block.iter().cloned().collect(),
                            children: vec![],
                        },
                        StyledNode {
                            node_type: &node,
                            properties: block.iter().cloned().collect(),
                            children: vec![],
                        },
                    ],
                },
                StyledNode {
                    node_type: &node,
                    properties: inline.iter().cloned().collect(),
                    children: vec![],
                },
                StyledNode {
                    node_type: &node,
                    properties: block.iter().cloned().collect(),
                    children: vec![],
                },
            ],
        };

        assert_eq!(
            to_layout_box(snode),
            LayoutBox {
                box_type: BoxType::BlockBox(BoxProps {
                    node_type: &node,
                    properties: block.iter().cloned().collect(),
                }),
                children: vec![
                    LayoutBox {
                        box_type: BoxType::BlockBox(BoxProps {
                            node_type: &node,
                            properties: block.iter().cloned().collect(),
                        }),
                        children: vec![],
                    },
                    LayoutBox {
                        box_type: BoxType::AnonymousBox,
                        children: vec![
                            LayoutBox {
                                box_type: BoxType::InlineBox(BoxProps {
                                    node_type: &node,
                                    properties: inline.iter().cloned().collect(),
                                }),
                                children: vec![
                                    LayoutBox {
                                        box_type: BoxType::BlockBox(BoxProps {
                                            node_type: &node,
                                            properties: block.iter().cloned().collect(),
                                        }),
                                        children: vec![],
                                    },
                                    LayoutBox {
                                        box_type: BoxType::BlockBox(BoxProps {
                                            node_type: &node,
                                            properties: block.iter().cloned().collect(),
                                        }),
                                        children: vec![],
                                    }
                                ],
                            },
                            LayoutBox {
                                box_type: BoxType::InlineBox(BoxProps {
                                    node_type: &node,
                                    properties: inline.iter().cloned().collect(),
                                }),
                                children: vec![],
                            }
                        ]
                    },
                    LayoutBox {
                        box_type: BoxType::BlockBox(BoxProps {
                            node_type: &node,
                            properties: block.iter().cloned().collect(),
                        }),
                        children: vec![],
                    }
                ],
            }
        );
    }
}
