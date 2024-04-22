use crate::GRAPH;

#[no_mangle]
pub unsafe extern "C" fn edge_get_data(edge: u32) -> sapp_jsutils::JsObjectWeak {
    GRAPH.with_borrow(|g| {
        g.get_edge(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .data
            .weak()
    })
}

#[no_mangle]
pub unsafe extern "C" fn edge_get_thickness(edge: u32) -> f32 {
    GRAPH.with_borrow(|g| {
        g.get_edge(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .thickness
    })
}
#[no_mangle]
pub unsafe extern "C" fn edge_set_thickness(edge: u32, thickness: f32) {
    GRAPH.with_borrow_mut(|g| {
        g.get_edge_mut(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .thickness = thickness;
    })
}

#[no_mangle]
pub unsafe extern "C" fn edge_get_label(edge: u32) -> sapp_jsutils::JsObject {
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
#[no_mangle]
pub unsafe extern "C" fn edge_set_label(edge: u32, label: sapp_jsutils::JsObject) {
    let label = if label.is_nil() {
        None
    } else {
        let mut string = String::new();
        label.to_string(&mut string);
        Some(string)
    };
    GRAPH.with_borrow_mut(|g| {
        g.get_edge_mut(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .label = label;
    });
}

#[no_mangle]
pub unsafe extern "C" fn edge_get_color(edge: u32) -> u32 {
    let rgba: [u8; 4] = GRAPH
        .with_borrow(|g| {
            g.get_edge(rgraphview::graph::GPtr::from_index(edge))
                .unwrap()
                .color
        })
        .into();
    std::mem::transmute(rgba)
}
#[no_mangle]
pub unsafe extern "C" fn edge_set_color(edge: u32, color: sapp_jsutils::JsObject) {
    GRAPH.with_borrow_mut(|g| {
        g.get_edge_mut(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .color = super::strtocol(color);
    });
}

#[no_mangle]
pub unsafe extern "C" fn edge_get_start(edge: u32) -> u32 {
    GRAPH.with_borrow(|g| {
        g.get_edge(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .start
            .get_index()
    })
}
#[no_mangle]
pub unsafe extern "C" fn edge_get_end(edge: u32) -> u32 {
    GRAPH.with_borrow(|g| {
        g.get_edge(rgraphview::graph::GPtr::from_index(edge))
            .unwrap()
            .end
            .get_index()
    })
}
