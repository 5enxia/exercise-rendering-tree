use exercise_rendering_tree::{
    html,
    css,
    layout::to_layout_box,
    render::to_element_container,
    style::to_styled_node,
    dom::{Node, NodeType}
};

const HTML: &str = r#"<body>
    <p>hello</p>
    <p class="inline">world</p>
    <p class="inline">:)</p>
    <div class="none"><p>this should not be shown</p></div>
    <style>
        .none {
            display: none;
        }
        .inline {
            display: inline;
        }
    </style>
</body>"#;

const DEFAULT_STYLESHEET: &str = r#"
script, style {
    display: none;
}
p, div {
    display: block;
}
"#;

fn main() {
    let mut siv = cursive::default();

    let node = html::parse(HTML);
    let stylesheet = css::parse(&format!(
        "{}\n{}",
        DEFAULT_STYLESHEET,
        collect_tag_inners(&node, "style".into()).join("\n")
    ));

    let container = to_styled_node(&node, &stylesheet)
        .and_then(|styled_node| Some(to_layout_box(styled_node)))
        .and_then(|layout_box| Some(to_element_container(layout_box)));
    if let Some(container) = container {
        siv.add_fullscreen_layer(container);
    }

    siv.run();
}

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