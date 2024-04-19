use macroquad::rand;

pub enum UpdateMessage {
    NewNode(rgraphview::Node<sapp_jsutils::JsObject>),
}

thread_local! {
    pub static GRAPH: std::cell::RefCell<rgraphview::Graph<sapp_jsutils::JsObject, sapp_jsutils::JsObject>> = std::cell::RefCell::new(rgraphview::Graph::new());
}

#[no_mangle]
pub extern "C" fn create_node(data: sapp_jsutils::JsObject) -> u32 {
    let node = rgraphview::Node::from_data(data);
    unsafe { GRAPH.with_borrow_mut(|g| g.add_node(node)).get_index() }
}

#[no_mangle]
pub extern "C" fn create_edge(start: u32, end: u32, data: sapp_jsutils::JsObject) -> u32 {
    let edge = rgraphview::Edge::new(
        unsafe { rgraphview::graph::GPtr::from_index(start) },
        unsafe { rgraphview::graph::GPtr::from_index(end) },
        data,
    );
    unsafe { GRAPH.with_borrow_mut(|g| g.add_edge(edge)).get_index() }
}

#[no_mangle]
pub extern "C" fn set_node(node: u32, with: sapp_jsutils::JsObject) {
    unsafe {
        GRAPH.with_borrow_mut(|g| {
            let node = g
                .get_node_mut(rgraphview::graph::GPtr::from_index(node))
                .unwrap();
            let mut string = String::new();

            if with.have_field("shape") {
                with.field("shape").to_string(&mut string);
                string.make_ascii_lowercase();
                let shape = match string.as_str() {
                    "circle" | "c" => rgraphview::node::Shape::Circle,
                    "square" | "s" => rgraphview::node::Shape::Square,
                    "triangle" | "t" => rgraphview::node::Shape::Triangle,
                    _ => {
                        eprintln!("Shape was not [\"circle\", \"square\", \"triangle\"]");
                        rgraphview::node::Shape::Circle
                    }
                };
                string.clear();
                node.set_shape(shape);
            }
            if with.have_field("radius") {
                let rad = with.field_f32("radius");
                node.set_radius(rad);
            }
            if with.have_field("color") {
                with.field("color").to_string(&mut string);
                string.make_ascii_lowercase();
                let color = match string.as_str() {
                    "red" => macroquad::color::RED,
                    _ => {
                        let hex = with.field_u32("color");
                        macroquad::color::Color::from_hex(hex)
                    }
                };
                string.clear();
                node.set_color(color);
            }
        })
    }
}

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

    loop {
        macroquad::window::clear_background(macroquad::color::WHITE);
        GRAPH.with_borrow_mut(|g| g.step());
        GRAPH.with_borrow(|g| g.draw());
        macroquad::window::next_frame().await;
    }
}
