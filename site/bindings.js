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
    return get_js_object(wasm_exports.node_get_data(this.node_idx));
  }

  set data(data) {
    let obj = get_js_object(wasm_exports.node_get_data(this.node_idx));
    obj = data;
  }

  get color() {
    return consume_js_object(wasm_exports.node_get_color(this.node_idx));
  }
  set color(color) {
    wasm_exports.node_set_color(this.node_idx, js_object(color));
  }

  get radius() {
    return wasm_exports.node_get_radius(this.node_idx);
  }
  set radius(radius) {
    wasm_exports.node_set_radius(this.node_idx, radius);
  }

  get label() {
    return consume_js_object(wasm_exports.node_get_label(this.node_idx));
  }
  set label(label) {
    wasm_exports.node_set_label(this.node_idx, js_object(label));
  }

  get shape() {
    return consume_js_object(wasm_exports.node_get_shape(this.node_idx));
  }
  set shape(shape) {
    wasm_exports.node_set_shape(this.node_idx, js_object(shape));
  }
}



class Graph {
  static nodes() {
    return new Uint32Array(consume_js_object(wasm_exports.graph_nodes()).buffer);
  }
  
  static add_node(properties) {
    let node = new Node(wasm_exports.graph_make_node());
    // wasm_exports.set_node(node_idx, js_object(properties));
    return node;
  }
  // add_edge(start, end, properties) {
  //   const edge_idx = wasm_exports.graph_make_edge(start, end);
  //   wasm_exports.set_edge(edge_idx, js_object(with_feats));
  //   return edge_idx;
  // }
}

// function add_edge(start, end, with_feats) {
//   const edge_idx = wasm_exports.create_edge(start, end);
//   wasm_exports.set_edge(edge_idx, js_object(with_feats));
//   return edge_idx;
// }

// function set_node(node, with_feats) {
//   wasm_exports.set_node(node, js_object(with_feats));
// }
// function set_edge(edge, with_feats) {
//   wasm_exports.set_edge(edge, js_object(with_feats));
// }

// function reset() {
//   wasm_exports.reset_graph();
//   js_objects = [];
// }

// function get_edges(node) {
//   const u8_array = consume_js_object(wasm_exports.node_edges(node));
//   return new Uint32Array(u8_array.buffer);
// }

// function get_nodes() {
//   const u8_array = consume_js_object(wasm_exports.get_nodes());
//   return new Uint32Array(u8_array.buffer);
// }
