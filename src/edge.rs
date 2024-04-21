use macroquad::color::Color;

use crate::{graph::GPtr, Node};

pub struct Edge<NodeData, EdgeData> {
    /// The start node.
    pub start: GPtr<Node<NodeData>>,
    /// The end node.
    pub end: GPtr<Node<NodeData>>,
    /// User-defined data.
    pub data: EdgeData,

    pub color: Color,
    pub thickness: f32,
    pub label: Option<String>,
}

impl<NodeData, EdgeData> std::fmt::Debug for Edge<NodeData, EdgeData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edge")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("color", &self.color)
            .field("thickness", &self.thickness)
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}
impl<NodeData, EdgeData> Edge<NodeData, EdgeData> {
    pub fn new(start: GPtr<Node<NodeData>>, end: GPtr<Node<NodeData>>, data: EdgeData) -> Self {
        Self {
            start,
            end,
            data,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            thickness: 10.0,
            label: None,
        }
    }

    #[must_use = "The return value has the changed data."]
    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
    #[must_use = "The return value has the changed data."]
    pub fn with_thickness(self, thickness: f32) -> Self {
        Self { thickness, ..self }
    }
    #[must_use = "The return value has the changed data."]
    pub fn with_label(self, label: Option<String>) -> Self {
        Self { label, ..self }
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
    pub fn set_thickness(&mut self, thickness: f32) -> &mut Self {
        self.thickness = thickness;
        self
    }
    pub fn set_label(&mut self, label: Option<String>) -> &mut Self {
        self.label = label;
        self
    }
}
impl<NodeData, EdgeData: Default> Edge<NodeData, EdgeData> {
    #[must_use]
    pub fn new_with_default(start: GPtr<Node<NodeData>>, end: GPtr<Node<NodeData>>) -> Self {
        Self::new(start, end, EdgeData::default())
    }
}
