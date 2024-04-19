function add_node(with_feats) {
  let node_idx = wasm_exports.create_node();
  wasm_exports.set_node(node_idx, js_object(with_feats));
  return node_idx;
}

function add_edge(start, end, with_feats) {
  const edge_idx = wasm_exports.create_edge(start, end);
  wasm_exports.set_edge(edge_idx, js_object(with_feats));
  return edge_idx;
}

function set_node(node, with_feats) {
  wasm_exports.set_node(node, js_object(with_feats));
}
function set_edge(edge, with_feats) {
  wasm_exports.set_edge(edge, js_object(with_feats));
}

function reset() {
  wasm_exports.reset_graph();
}

function get_edges(node) {
  const u8_array = wasm_exports.node_edges(node);
  return Uint32Array(u8_array.buffer);
}
