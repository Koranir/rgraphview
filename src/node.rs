use macroquad::color::Color;
pub enum Shape {
    Circle,
    Square,
    Triangle,
}

pub struct Node<NodeData> {
    pub label: Option<String>,
    /// The shape of the node.
    pub shape: Shape,
    /// The fill color of the node.
    pub color: Color,
    /// The radius of a node, in pixels.
    pub radius: f32,
    /// User-defined data.
    pub data: NodeData,
}
impl<ND: Default> Node<ND> {
    #[must_use]
    pub fn new() -> Self {
        Self::from_data(ND::default())
    }
}
impl<ND: Default> Default for Node<ND> {
    fn default() -> Self {
        Self::new()
    }
}
impl<NodeData> Node<NodeData> {
    pub fn from_data(data: NodeData) -> Self {
        Self {
            shape: Shape::Circle,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            radius: 25.0,
            data,
            label: None,
        }
    }

    #[must_use = "The return value has the changed data."]
    pub fn with_shape(self, shape: Shape) -> Self {
        Self { shape, ..self }
    }
    #[must_use = "The return value has the changed data."]
    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
    #[must_use = "The return value has the changed data."]
    pub fn with_radius(self, radius: f32) -> Self {
        Self { radius, ..self }
    }
    #[must_use = "The return value has the changed data."]
    pub fn with_data(self, data: NodeData) -> Self {
        Self { data, ..self }
    }
    #[must_use = "The return value has the changed data."]
    pub fn with_label(self, label: Option<String>) -> Self {
        Self { label, ..self }
    }

    pub fn set_shape(&mut self, shape: Shape) -> &mut Self {
        self.shape = shape;
        self
    }
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }
    pub fn set_data(&mut self, data: NodeData) -> &mut Self {
        self.data = data;
        self
    }
    pub fn set_label(&mut self, label: Option<String>) -> &mut Self {
        self.label = label;
        self
    }
}
