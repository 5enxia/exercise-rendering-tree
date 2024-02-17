use std::rc::Rc;

use cursive::theme::Theme;
use exercise_rendering_tree::{
    html,
    renderer::Renderer
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

    <div id="result">
        <p>not loaded</p>
    </div>
    <script>
        document.getElementById("result").innerHTML = `\x3cp\x3eloaded\x3c/p\x3e`
    </script>    
</body>"#;


fn main() {
    let mut siv = cursive::default();
    let mut palette= siv.current_theme().palette.clone();
    palette.set_color("background", cursive::theme::Color::TerminalDefault);

    let theme = Theme {
        shadow: false,
        borders: cursive::theme::BorderStyle::Simple,
        palette: palette
    };
    siv.set_theme(theme);

    let node = html::parse(HTML);

    // Rendererを生成する
    let mut renderer = Renderer::new(Rc::new(siv.cb_sink().clone()), node);

    // inline JavaScriptを実行する
    renderer.execute_inline_scripts();

    // Cursiveによる描画を開始する
    // siv.add_fullscreen_layer(renderer.view);
    siv.add_fullscreen_layer(renderer);

    siv.run();
}
