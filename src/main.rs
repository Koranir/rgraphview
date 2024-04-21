use macroquad::rand;

pub enum UpdateMessage {
    NewNode(rgraphview::Node<sapp_jsutils::JsObject>),
}

thread_local! {
    pub static GRAPH: std::cell::RefCell<rgraphview::Graph<sapp_jsutils::JsObject, sapp_jsutils::JsObject>> = std::cell::RefCell::new(rgraphview::Graph::new());
}

#[no_mangle]
pub extern "C" fn get_node_data(node: u32) -> sapp_jsutils::JsObjectWeak {
    GRAPH.with_borrow(|g| {
        g.get_node(unsafe { rgraphview::graph::GPtr::from_index(node) })
            .unwrap()
            .data
            .weak()
    })
}
#[no_mangle]
pub extern "C" fn get_node_color(node: u32) -> sapp_jsutils::JsObject {
    sapp_jsutils::JsObject::string(unsafe {
        let [r, g, b, a]: [u8; 4] = GRAPH
            .with_borrow(|g| {
                g.get_node(rgraphview::graph::GPtr::from_index(node))
                    .unwrap()
                    .color
            })
            .into();
        &format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    })
}
#[no_mangle]
pub extern "C" fn get_node_radius(node: u32) -> f32 {
    unsafe {
        GRAPH.with_borrow(|g| {
            g.get_node(rgraphview::graph::GPtr::from_index(node))
                .unwrap()
                .radius
        })
    }
}
#[no_mangle]
pub extern "C" fn get_node_label(node: u32) -> sapp_jsutils::JsObject {
    unsafe {
        GRAPH.with_borrow(|g| {
            sapp_jsutils::JsObject::string(
                g.get_node(rgraphview::graph::GPtr::from_index(node))
                    .unwrap()
                    .label
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(""),
            )
        })
    }
}
#[no_mangle]
pub extern "C" fn get_node_shape(node: u32) -> sapp_jsutils::JsObject {
    unsafe {
        GRAPH.with_borrow(|g| {
            sapp_jsutils::JsObject::string(
                match g
                    .get_node(rgraphview::graph::GPtr::from_index(node))
                    .unwrap()
                    .shape
                {
                    rgraphview::node::Shape::Circle => "circle",
                    rgraphview::node::Shape::Square => "square",
                    rgraphview::node::Shape::Triangle => "triangle",
                },
            )
        })
    }
}

#[no_mangle]
pub extern "C" fn get_edge_data(edge: u32) -> sapp_jsutils::JsObjectWeak {
    GRAPH.with_borrow(|g| {
        g.get_edge(unsafe { rgraphview::graph::GPtr::from_index(edge) })
            .unwrap()
            .data
            .weak()
    })
}
#[no_mangle]
pub extern "C" fn get_edge_thickness(edge: u32) -> f32 {
    unsafe {
        GRAPH.with_borrow(|g| {
            g.get_edge(rgraphview::graph::GPtr::from_index(edge))
                .unwrap()
                .thickness
        })
    }
}
#[no_mangle]
pub extern "C" fn get_edge_label(edge: u32) -> sapp_jsutils::JsObject {
    unsafe {
        GRAPH.with_borrow(|g| {
            sapp_jsutils::JsObject::string(
                g.get_edge(rgraphview::graph::GPtr::from_index(edge))
                    .unwrap()
                    .label
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(""),
            )
        })
    }
}
#[no_mangle]
pub extern "C" fn get_edge_color(edge: u32) -> u32 {
    unsafe {
        let rgba: [u8; 4] = GRAPH
            .with_borrow(|g| {
                g.get_edge(rgraphview::graph::GPtr::from_index(edge))
                    .unwrap()
                    .color
            })
            .into();
        std::mem::transmute(rgba)
    }
}
#[no_mangle]
pub extern "C" fn get_edge_start(edge: u32) -> u32 {
    unsafe {
        GRAPH.with_borrow(|g| {
            g.get_edge(rgraphview::graph::GPtr::from_index(edge))
                .unwrap()
                .start
                .get_index()
        })
    }
}
#[no_mangle]
pub extern "C" fn get_edge_end(edge: u32) -> u32 {
    unsafe {
        GRAPH.with_borrow(|g| {
            g.get_edge(rgraphview::graph::GPtr::from_index(edge))
                .unwrap()
                .end
                .get_index()
        })
    }
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
pub extern "C" fn node_edges(node: u32) -> sapp_jsutils::JsObject {
    GRAPH.with_borrow(|g| {
        let edges = g
            .get_node_with_meta(unsafe { rgraphview::graph::GPtr::from_index(node) })
            .unwrap()
            .1
            .iter()
            .map(|f| unsafe { f.get_index() })
            .flat_map(|f| f.to_ne_bytes())
            .collect::<Vec<_>>();
        sapp_jsutils::JsObject::buffer(&edges)
    })
}

#[no_mangle]
pub extern "C" fn get_nodes() -> sapp_jsutils::JsObject {
    GRAPH.with_borrow(|g| {
        let edges = g
            .nodes()
            .enumerate()
            .map(|(idx, _f)| idx)
            .flat_map(|f| f.to_ne_bytes())
            .collect::<Vec<_>>();
        sapp_jsutils::JsObject::buffer(&edges)
    })
}

#[no_mangle]
pub extern "C" fn set_node(node_idx: u32, with: sapp_jsutils::JsObject) {
    unsafe {
        GRAPH.with_borrow_mut(|g| {
            let Some(node) = g.get_node_mut(rgraphview::graph::GPtr::from_index(node_idx)) else {
                panic!(
                    "Node {node_idx} was not in the graph: {:?}",
                    g.nodes().enumerate().fold(String::new(), |s, (id, n)| {
                        s + &format!("[{id}: {n:?}],")
                    })
                )
            };
            let mut string = String::new();

            if with.is_nil() {
                eprintln!("Node {node_idx} was nil");
                return;
            }

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
                let color = match string.as_str().trim() {
                    "red" => macroquad::color::RED,
                    s if s.starts_with('#') => {
                        macroquad::color::Color::from_hex(u32::from_str_radix(&s[1..], 16).unwrap())
                    }
                    _ => {
                        let hex = with.field_u32("color");
                        macroquad::color::Color::from_hex(hex)
                    }
                };
                string.clear();
                node.set_color(color);
            }
            if with.have_field("label") {
                with.field("label").to_string(&mut string);
                node.set_label(Some(string));
            }
        })
    }
}

#[no_mangle]
extern "C" fn set_edge(edge_idx: u32, with: sapp_jsutils::JsObject) {
    unsafe {
        GRAPH.with_borrow_mut(|g| {
            let Some(edge) = g.get_edge_mut(rgraphview::graph::GPtr::from_index(edge_idx)) else {
                panic!(
                    "Edge {edge_idx} was not in the graph: {:?}",
                    g.edges().enumerate().fold(String::new(), |s, (id, n)| {
                        s + &format!("[{id}: {n:?}],")
                    })
                )
            };
            let mut string = String::new();

            if with.is_nil() {
                eprintln!("Edge {edge_idx} was nil");
                return;
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
                edge.set_color(color);
            }
            if with.have_field("thickness") {
                edge.set_thickness(with.field_f32("thickness"));
            }
            if with.have_field("label") {
                with.field("label").to_string(&mut string);
                edge.set_label(Some(string));
            }
        });
    }
}

#[no_mangle]
pub extern "C" fn reset_graph() {
    GRAPH.with_borrow_mut(|graph| {
        graph.reset();
    })
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

    let arial_ttf = macroquad::text::load_ttf_font("Roboto-Medium.ttf")
        .await
        .unwrap();
    GRAPH.with_borrow_mut(|graph| {
        graph.set_font(arial_ttf);
    });

    loop {
        macroquad::window::clear_background(macroquad::color::WHITE);
        GRAPH.with_borrow_mut(|g| {
            g.step();
            g.draw();
        });
        macroquad::window::next_frame().await;
    }
}
