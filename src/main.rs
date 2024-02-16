use exercise_rendering_tree::{
    html,
    css,
    layout::to_layout_box,
    render::to_element_container,
    style::to_styled_node
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
    let stylesheet = css::parse(&format!("{}\n{}", DEFAULT_STYLESHEET, ""));

    let container = to_styled_node(&node, &stylesheet)
        .and_then(|styled_node| Some(to_layout_box(styled_node)))
        .and_then(|layout_box| Some(to_element_container(layout_box)));
    if let Some(container) = container {
        siv.add_fullscreen_layer(container);
    }

    siv.run();
}
