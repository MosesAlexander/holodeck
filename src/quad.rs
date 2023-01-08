use crate::cube::TextureCorner;
use crate::cube::*;

pub struct Quad {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub center: (f32, f32, f32),
}

impl Quad {
    // TODO: get plane given center_offset vector, for now just put plane on XZ using Y component from center_offset
    pub fn new(width: f32, height: f32, center_offset: (f32, f32, f32), center: (f32, f32, f32), texture_scale_factor: (f32, f32)) -> Quad {
        let mut vertices_quad: Vec<f32> = Vec::new();
        let mut indices_quad: Vec<u32> = Vec::new();

        match center_offset {
            (x, y, z) if x != 0.0 => {
                vertices_quad.extend_from_slice(&[x, 0.0, -width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft, texture_scale_factor));
                vertices_quad.extend_from_slice(&[x, height,  -width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft, texture_scale_factor));
                vertices_quad.extend_from_slice(&[x, 0.0,  width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight, texture_scale_factor));
                vertices_quad.extend_from_slice(&[x,  height,  width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight, texture_scale_factor));

                indices_quad.extend_from_slice(&[0,1,2, 1,3,2 ]);
            },
            (x, y, z) if y != 0.0 => {
                vertices_quad.extend_from_slice(&[-width / 2.0, y, -width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft, texture_scale_factor));
                vertices_quad.extend_from_slice(&[ width / 2.0, y, -width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight, texture_scale_factor));
                vertices_quad.extend_from_slice(&[-width / 2.0, y,  width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft, texture_scale_factor));
                vertices_quad.extend_from_slice(&[ width / 2.0, y,  width / 2.0]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight, texture_scale_factor));

                indices_quad.extend_from_slice(&[0,1,2, 1,3,2 ]);
            },
            (x, y, z) if z != 0.0 => {
                vertices_quad.extend_from_slice(&[-width / 2.0, 0.0, z]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft, texture_scale_factor));
                vertices_quad.extend_from_slice(&[width / 2.0,  0.0, z]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight, texture_scale_factor));
                vertices_quad.extend_from_slice(&[-width / 2.0, height, z]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft, texture_scale_factor));
                vertices_quad.extend_from_slice(&[width / 2.0,  height, z]);
                vertices_quad.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight, texture_scale_factor));

                indices_quad.extend_from_slice(&[0,1,2, 1,3,2 ]);
            },
            (_, _, _) => {
                println!("ERROR: only simple offset vectors accepted")
            }
        }


        Quad { vertices: vertices_quad, indices: indices_quad, center: center}
    }
}