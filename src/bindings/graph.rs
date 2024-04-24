use crate::GRAPH;

#[no_mangle]
pub unsafe extern "C" fn graph_make_node(data: sapp_jsutils::JsObject) -> u32 {
    let node = rgraphview::Node::from_data(data);
    unsafe { GRAPH.with_borrow_mut(|g| g.add_node(node)).get_index() }
}

#[no_mangle]
pub unsafe extern "C" fn graph_make_edge(
    start: u32,
    end: u32,
    data: sapp_jsutils::JsObject,
) -> u32 {
    let edge = rgraphview::Edge::new(
        rgraphview::graph::GPtr::from_index(start),
        rgraphview::graph::GPtr::from_index(end),
        data,
    );
    GRAPH.with_borrow_mut(|g| g.add_edge(edge)).get_index()
}

#[no_mangle]
pub extern "C" fn graph_nodes() -> sapp_jsutils::JsObject {
    GRAPH.with_borrow(|g| {
        let edges = g
            .nodes()
            .enumerate()
            .map(|(idx, _)| idx)
            .flat_map(|f| f.to_ne_bytes())
            .collect::<Vec<_>>();
        sapp_jsutils::JsObject::buffer(&edges)
    })
}
#[no_mangle]
pub extern "C" fn graph_edges() -> sapp_jsutils::JsObject {
    GRAPH.with_borrow(|g| {
        let edges = g
            .edges()
            .enumerate()
            .map(|(idx, _)| idx)
            .flat_map(|f| f.to_ne_bytes())
            .collect::<Vec<_>>();
        sapp_jsutils::JsObject::buffer(&edges)
    })
}

#[no_mangle]
pub extern "C" fn graph_reset() {
    GRAPH.with_borrow_mut(|graph| {
        graph.reset();
    })
}
