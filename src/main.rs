use macroquad::rand;

#[rgraphview::main("RGraphView")]
async fn main() {
    let mut graph: rgraphview::Graph<(), ()> = rgraphview::Graph::new();

    let nodes: [_; 16] = std::array::from_fn(|_| graph.add_node(rgraphview::Node::new()));
    let edges: [_; 32] = std::array::from_fn(|n| {
        graph.add_edge(rgraphview::Edge::new(
            nodes[rand::gen_range(0, 16)],
            nodes[rand::gen_range(0, 16)],
            (),
        ))
    });

    loop {
        macroquad::window::clear_background(macroquad::color::WHITE);
        graph.step();
        graph.draw();
        macroquad::window::next_frame().await;
    }
}
