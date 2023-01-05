use crate::cube::TextureCorner;
use crate::cube::*;

pub struct Quad {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub center: (f32, f32, f32),
}

impl Quad {
    // TODO: get plane given normal vector, for now just put plane on XZ using Y component from normal
    pub fn new(width: f32, height: f32, normal: (f32, f32, f32), center: (f32, f32, f32)) -> Quad {
        let mut vertices_quad: Vec<f32> = Vec::new();
        let mut indices_quad: Vec<u32> = Vec::new();

        vertices_quad.extend_from_slice(&[0.0, normal.1, 0.0]);
        vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));
        vertices_quad.extend_from_slice(&[20.0, normal.1, 0.0]);
        vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));
        vertices_quad.extend_from_slice(&[0.0, normal.1, 20.0]);
        vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));
        vertices_quad.extend_from_slice(&[20.0, normal.1, 20.0]);
        vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));

        
        indices_quad.extend_from_slice(&[0,1,2, 1,3,2 ]);

        Quad { vertices: vertices_quad, indices: indices_quad, center: center}
    }
}