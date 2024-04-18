use macroquad::{math::Vec2, rand::gen_range};

use crate::{Edge, Node};

pub struct Graph<ND, ED> {
    nodes: Vec<(Node<ND>, Vec<GPtr<Edge<ND, ED>>>, macroquad::math::Vec2)>,
    edges: Vec<Edge<ND, ED>>,
    bounds: macroquad::math::Rect,
    dragging: Option<GPtr<Node<ND>>>,
}
impl<ND, ED> Default for Graph<ND, ED> {
    fn default() -> Self {
        Self::new()
    }
}

impl<ND, ED> Graph<ND, ED> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            bounds: macroquad::math::Rect::new(-500.0, -500.0, 1000.0, 1000.0),
            dragging: None,
        }
    }
    pub fn add_node(&mut self, node: Node<ND>) -> GPtr<Node<ND>> {
        self.nodes.push((
            node,
            Vec::new(),
            Vec2::new(gen_range(0.0, 1.0), gen_range(0.0, 1.0)),
        ));
        GPtr {
            idx: self.nodes.len() as u32 - 1,
            _marker: std::marker::PhantomData,
        }
    }
    #[must_use]
    pub fn get_node_with_meta(
        &self,
        ptr: GPtr<Node<ND>>,
    ) -> Option<&(Node<ND>, Vec<GPtr<Edge<ND, ED>>>, Vec2)> {
        self.nodes.get(ptr.idx as usize)
    }
    pub fn get_node_with_meta_mut(
        &mut self,
        ptr: GPtr<Node<ND>>,
    ) -> Option<&mut (Node<ND>, Vec<GPtr<Edge<ND, ED>>>, Vec2)> {
        self.nodes.get_mut(ptr.idx as usize)
    }
    #[must_use]
    pub fn get_node(&self, ptr: GPtr<Node<ND>>) -> Option<&Node<ND>> {
        self.get_node_with_meta(ptr).map(|f| &f.0)
    }
    pub fn get_node_mut(&mut self, ptr: GPtr<Node<ND>>) -> Option<&mut Node<ND>> {
        self.get_node_with_meta_mut(ptr).map(|f| &mut f.0)
    }
    #[must_use]
    pub fn get_node_edges(
        &self,
        ptr: GPtr<Node<ND>>,
    ) -> Option<impl Iterator<Item = &Edge<ND, ED>>> {
        self.get_node_with_meta(ptr)
            .map(|f| f.1.iter().map(|&f| self.get_edge(f).unwrap()))
    }
    // pub fn get_node_edges_mut(
    //     &mut self,
    //     ptr: GPtr<Node<ND>>,
    // ) -> Option<impl Iterator<Item = &mut Edge<ND, ED>>> {
    //     self.get_node_with_edges(ptr)
    //         .map(|f| f.1.iter().map(|&f| self.get_edge_mut(f).unwrap()))
    // }

    pub fn add_edge(&mut self, edge: Edge<ND, ED>) -> GPtr<Edge<ND, ED>> {
        let nodes = [edge.start, edge.end];
        self.edges.push(edge);
        let edge_ptr = GPtr {
            idx: self.edges.len() as u32 - 1,
            _marker: std::marker::PhantomData,
        };
        for node in nodes {
            self.get_node_with_meta_mut(node).unwrap().1.push(edge_ptr);
        }
        edge_ptr
    }
    #[must_use]
    pub fn get_edge(&self, edge: GPtr<Edge<ND, ED>>) -> Option<&Edge<ND, ED>> {
        self.edges.get(edge.idx as usize)
    }
    pub fn get_edge_mut(&mut self, edge: GPtr<Edge<ND, ED>>) -> Option<&mut Edge<ND, ED>> {
        self.edges.get_mut(edge.idx as usize)
    }

    pub fn step(&mut self) {
        let diffs = self
            .nodes
            .iter()
            .map(|f| {
                let pos = f.2;
                self.nodes
                    .iter()
                    .map(|f| {
                        let diff_vec = pos - f.2;

                        diff_vec.normalize_or_zero() * (20.0 - diff_vec.length_squared().log2())
                    })
                    .filter(|d| d.is_finite())
                    .reduce(|acc, ele| acc + ele)
            })
            .collect::<Vec<_>>();
        let riffs = self
            .nodes
            .iter()
            .enumerate()
            .map(|(node_idx, f)| (node_idx, &f.2, &f.1))
            .map(|(node_idx, &this, f)| {
                let graph = &*self;
                f.iter()
                    .map(move |f| {
                        let edge = graph.get_edge(*f).unwrap();
                        let other = if edge.start.idx == node_idx as u32 {
                            graph.get_node_with_meta(edge.end).unwrap().2
                        } else {
                            graph.get_node_with_meta(edge.start).unwrap().2
                        };

                        (other - this) * 0.025
                    })
                    .reduce(|acc, f| acc + f)
            })
            .collect::<Vec<_>>();
        let mut bounds = macroquad::math::Rect::default();
        let mut diff_iter = diffs.into_iter().zip(riffs);

        let mouse_pos = macroquad::input::mouse_position_local()
            .mul_add(Vec2::new(0.5, -0.5), Vec2::splat(0.5))
            * self.bounds.size()
            + self.bounds.point();

        if macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Left) {
            self.dragging = None;
        }
        let drag_new =
            macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left);

        for (node_idx, node) in self.nodes.iter_mut().enumerate() {
            if let Some((diff, riff)) = diff_iter.next() {
                node.2 += diff.unwrap_or_default() * 0.5;
                node.2 += riff.unwrap_or_default();
            }
            // node.2 *= 0.9;

            if drag_new
                && self.dragging.is_none()
                && mouse_pos.distance_squared(node.2) < (node.0.radius * node.0.radius)
            {
                self.dragging = Some(GPtr {
                    idx: node_idx as u32,
                    _marker: std::marker::PhantomData,
                });
            }

            bounds = bounds.combine_with(macroquad::math::Rect::new(
                node.2.x,
                node.2.y,
                node.0.radius,
                node.0.radius,
            ));
        }

        if let Some(ptr) = self.dragging {
            self.get_node_with_meta_mut(ptr).unwrap().2 = mouse_pos;
        }

        let aspect = macroquad::window::screen_width() / macroquad::window::screen_height();
        self.bounds = Self::scale_aspect(bounds, aspect, 1.2);
    }

    fn scale_aspect(
        display_rect: macroquad::math::Rect,
        camera_aspect: f32,
        scale: f32,
    ) -> macroquad::math::Rect {
        let display_aspect = display_rect.w / display_rect.h;

        let diff_aspect = camera_aspect / display_aspect;

        let (new_width, new_height) = if diff_aspect > 1.0 {
            (display_rect.w * diff_aspect * scale, display_rect.h * scale)
        } else {
            (
                display_rect.w * scale,
                display_rect.h * (1. / diff_aspect) * scale,
            )
        };
        let width_offset = display_rect.w - new_width;
        let heigh_offset = display_rect.h - new_height;

        macroquad::math::Rect::new(
            width_offset.mul_add(0.5, display_rect.x),
            heigh_offset.mul_add(0.5, display_rect.y),
            new_width,
            new_height,
        )
    }

    pub fn draw(&self) {
        let view = macroquad::camera::Camera2D::from_display_rect(self.bounds);
        macroquad::camera::push_camera_state();
        macroquad::camera::set_camera(&view);

        for edge in &self.edges {
            if edge.end == edge.start {
                let (node, _, v) = self.get_node_with_meta(edge.start).unwrap();
                // macroquad::shapes::draw_ellipse inlined to reduce splitting.
                {
                    let x = v.x;
                    let y = node.radius.mul_add(2.0, v.y);
                    let w = node.radius;
                    let h = node.radius * 2.0;
                    let thickness = edge.thickness;
                    let color = edge.color;
                    let sides = node.radius as i32;

                    let rot = 0.0f32.to_radians();
                    let sr = rot.sin();
                    let cr = rot.cos();
                    for i in 0..sides {
                        let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2.).cos();
                        let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2.).sin();
                        let px = w * rx;
                        let py = h * ry;
                        let rotated_x = px.mul_add(cr, -(py * sr));
                        let rotated_y = py.mul_add(cr, px * sr);

                        let p0 = Vec2::new(x + rotated_x, y + rotated_y);

                        let rx = ((i + 1) as f32 / sides as f32 * std::f32::consts::PI * 2.).cos();
                        let ry = ((i + 1) as f32 / sides as f32 * std::f32::consts::PI * 2.).sin();
                        let px = w * rx;
                        let py = h * ry;
                        let rotated_x = px.mul_add(cr, -(py * sr));
                        let rotated_y = py.mul_add(cr, px * sr);

                        let p1 = Vec2::new(x + rotated_x, y + rotated_y).lerp(p0, -0.2);

                        macroquad::shapes::draw_line(p0.x, p0.y, p1.x, p1.y, thickness, color);
                    }
                }
            } else {
                let v1 = self.get_node_with_meta(edge.start).unwrap().2;
                let v2 = self.get_node_with_meta(edge.end).unwrap().2;

                macroquad::shapes::draw_line(v1.x, v1.y, v2.x, v2.y, edge.thickness, edge.color);
            }
        }

        for node in &self.nodes {
            let pos = node.2;
            let node = &node.0;
            match node.shape {
                crate::node::Shape::Circle => {
                    macroquad::shapes::draw_circle(pos.x, pos.y, node.radius, node.color);
                }
                crate::node::Shape::Square => {
                    macroquad::shapes::draw_rectangle(
                        pos.x,
                        pos.y,
                        node.radius,
                        node.radius,
                        node.color,
                    );
                }
                crate::node::Shape::Triangle => {
                    let v1 = pos + Vec2::new(0.0, node.radius / 2.0);
                    let v2 = pos + Vec2::new(node.radius / 2.0, -node.radius / 2.0);
                    let v3 = pos + Vec2::new(-node.radius / 2.0, -node.radius / 2.0);
                    macroquad::shapes::draw_triangle(v1, v2, v3, node.color);
                }
            }
        }
        macroquad::camera::pop_camera_state();
    }
}

pub struct GPtr<T> {
    pub(crate) idx: u32,
    _marker: std::marker::PhantomData<T>,
}
impl<T> PartialEq for GPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}
impl<T> Eq for GPtr<T> {}
impl<T> Clone for GPtr<T> {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<T> Copy for GPtr<T> {}
