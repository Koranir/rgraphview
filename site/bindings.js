function add_node(with_feats) {
}

function standardize_color(str){
    var ctx = document.createElement('canvas').getContext('2d');
    ctx.fillStyle = str;
    return ctx.fillStyle;
}

class Node {
  constructor(node_idx) {
    this.node_idx = node_idx;
  }

  get data() {
    return get_js_object(wasm_exports.get_node_data(this.node_idx));
  }

  get color() {
    return consume_js_object(wasm_exports.get_node_color(this.node_idx));
  }

  get radius() {
    return wasm_exports.get_node_radius(this.node_idx);
  }

  get label() {
    return consume_js_object(wasm_exports.get_node_label(this.node_idx));
  }

  get shape() {
    return consume_js_object(wasm_exports.get_node_shape(this.node_idx));
  }
}



class Graph {
  add_node(properties) {
    let node_idx = wasm_exports.create_node();
    wasm_exports.set_node(node_idx, js_object(properties));
    return node_idx;
  }
  add_edge(start, end, properties) {
    const edge_idx = wasm_exports.create_edge(start, end);
    wasm_exports.set_edge(edge_idx, js_object(with_feats));
    return edge_idx;
  }
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
  js_objects = [];
}

function get_edges(node) {
  const u8_array = consume_js_object(wasm_exports.node_edges(node));
  return new Uint32Array(u8_array.buffer);
}

function get_nodes() {
  const u8_array = consume_js_object(wasm_exports.get_nodes());
  return new Uint32Array(u8_array.buffer);
}
