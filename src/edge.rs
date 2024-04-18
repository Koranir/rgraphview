use macroquad::color::Color;

use crate::{graph::GPtr, Node};

pub struct Edge<NodeData, EdgeData> {
    /// The start node.
    pub(crate) start: GPtr<Node<NodeData>>,
    /// The end node.
    pub(crate) end: GPtr<Node<NodeData>>,
    /// User-defined data.
    pub data: EdgeData,

    pub color: Color,
    pub thickness: f32,
}
impl<NodeData, EdgeData> Edge<NodeData, EdgeData> {
    pub fn new(start: GPtr<Node<NodeData>>, end: GPtr<Node<NodeData>>, data: EdgeData) -> Self {
        Self {
            start,
            end,
            data,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            thickness: 10.0,
        }
    }
}
impl<NodeData, EdgeData: Default> Edge<NodeData, EdgeData> {
    #[must_use]
    pub fn new_with_default(start: GPtr<Node<NodeData>>, end: GPtr<Node<NodeData>>) -> Self {
        Self::new(start, end, EdgeData::default())
    }
}
