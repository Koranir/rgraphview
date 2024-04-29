use macroquad::rand;

thread_local! {
    pub static GRAPH: std::cell::RefCell<rgraphview::Graph<sapp_jsutils::JsObject, sapp_jsutils::JsObject>> = std::cell::RefCell::new(rgraphview::Graph::new());
}

mod bindings;

#[rgraphview::main("RGraphView")]
async fn main() {
    GRAPH.with_borrow_mut(|graph| {
        let nodes: [_; 16] = std::array::from_fn(|_| {
            graph.add_node(rgraphview::Node::from_data(sapp_jsutils::JsObject::object()))
        });
        let _edges: [_; 32] = std::array::from_fn(|_| {
            graph.add_edge(rgraphview::Edge::new(
                nodes[rand::gen_range(0, 16)],
                nodes[rand::gen_range(0, 16)],
                sapp_jsutils::JsObject::object(),
            ))
        });
    });

    let arial_ttf = macroquad::text::load_ttf_font("Roboto-Medium.ttf")
        .await
        .unwrap();
    GRAPH.with_borrow_mut(|graph| {
        graph.set_font(arial_ttf);
    });

    loop {
        macroquad::window::clear_background(macroquad::color::Color::new(0.95, 0.95, 0.95, 1.0));
        GRAPH.with_borrow_mut(|g| {
            g.step();
            g.draw();
        });
        macroquad::window::next_frame().await;
    }
}
