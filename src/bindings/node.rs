use crate::GRAPH;

#[no_mangle]
pub unsafe extern "C" fn node_get_data(node: u32) -> sapp_jsutils::JsObjectWeak {
    GRAPH.with_borrow(|g| {
        g.get_node(rgraphview::graph::GPtr::from_index(node))
            .unwrap()
            .data
            .weak()
    })
}
// #[no_mangle]
// pub unsafe extern "C" fn node_set_data(node: u32, data: sapp_jsutils::JsObject) {
//     GRAPH.with_borrow(|g| {
//         g.get_node(rgraphview::graph::GPtr::from_index(node))
//             .unwrap()
//             .data
//             .weak()
//     })
// }

#[no_mangle]
pub unsafe extern "C" fn node_get_color(node: u32) -> sapp_jsutils::JsObject {
    let [r, g, b, a]: [u8; 4] = GRAPH
        .with_borrow(|g| {
            g.get_node(rgraphview::graph::GPtr::from_index(node))
                .unwrap()
                .color
        })
        .into();
    sapp_jsutils::JsObject::string(&format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a))
}
#[no_mangle]
pub unsafe extern "C" fn node_set_color(node: u32, color: sapp_jsutils::JsObject) {
    GRAPH.with_borrow_mut(|g| {
        g.get_node_mut(rgraphview::graph::GPtr::from_index(node))
            .unwrap()
            .color = super::strtocol(color);
    });
}

#[no_mangle]
pub unsafe extern "C" fn node_get_radius(node: u32) -> f32 {
    GRAPH.with_borrow(|g| {
        g.get_node(rgraphview::graph::GPtr::from_index(node))
            .unwrap()
            .radius
    })
}
#[no_mangle]
pub unsafe extern "C" fn node_set_radius(node: u32, radius: f32) {
    GRAPH.with_borrow_mut(|g| {
        g.get_node_mut(rgraphview::graph::GPtr::from_index(node))
            .unwrap()
            .radius = radius;
    })
}

#[no_mangle]
pub unsafe extern "C" fn node_get_label(node: u32) -> sapp_jsutils::JsObject {
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
#[no_mangle]
pub unsafe extern "C" fn node_set_label(node: u32, label: sapp_jsutils::JsObject) {
    let label = if label.is_nil() {
        None
    } else {
        let mut string = String::new();
        label.to_string(&mut string);
        Some(string)
    };
    GRAPH.with_borrow_mut(|g| {
        g.get_node_mut(rgraphview::graph::GPtr::from_index(node))
            .unwrap()
            .label = label
    })
}

#[no_mangle]
pub unsafe extern "C" fn node_get_shape(node: u32) -> sapp_jsutils::JsObject {
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
#[no_mangle]
pub unsafe extern "C" fn node_set_shape(node: u32, shape: sapp_jsutils::JsObject) {
    let mut string = String::new();
    shape.to_string(&mut string);
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
    GRAPH.with_borrow_mut(|g| {
        g.get_node_mut(rgraphview::graph::GPtr::from_index(node))
            .unwrap()
            .shape = shape;
    });
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
