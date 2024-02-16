use cursive::{views::{Layer, LayerPosition}, CbSink};
use std::rc::Rc;

use crate::renderer::Renderer;

pub struct RendererAPI {
    ui_cb_sink: Rc<CbSink>,
}

impl RendererAPI {
    pub fn new(ui_cb_sink: Rc<CbSink>) -> RendererAPI {
        RendererAPI {
            ui_cb_sink
        }
    }

    // Viewに再描画を要求する
    pub fn rerender(&self) {
        self.ui_cb_sink
            .send(Box::new(move |siv: &mut cursive::Cursive| {
                let screen = siv.screen_mut();
                let layer: &mut Renderer = screen
                    .get_mut(LayerPosition::FromBack(0))
                    .unwrap()
                    .downcast_mut()
                    .unwrap();
                layer.rerender()
            }))
            .unwrap();
    }
}