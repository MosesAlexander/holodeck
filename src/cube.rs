
pub struct Cube {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub center: (f32, f32, f32),
}

impl Cube {
    pub fn new(side_length: f32, center: (f32, f32, f32)) -> Cube {
        // Because of textures, each vertex needs 3 copies in the current format
        // so that each face can have a proper texture
        let mut vertices_cube: Vec<f32> = Vec::new();

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_A));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_A));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_A));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));

        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_B));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_B));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_B));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));


        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_C));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_C));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_C));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));


        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_D));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_D));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_D));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));


        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_E));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_E));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_E));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));


        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_F));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_F));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::TopRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_F));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));


        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_G));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_G));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_G));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));


        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_H));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomLeft));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_H));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));
        vertices_cube.extend_from_slice(&Cube::generate_cube_corner_coords(center, side_length, CubeCorner::COORDS_H));
        vertices_cube.extend_from_slice(&Cube::generate_texture_coords(TextureCorner::BottomRight));

        // This is just hell, gotta find a generic way to produce cubes..
        let indices_cube: Vec<u32> = vec![
            0, 3, 9, 0, 9, 6, // first face
            12, 15, 21, 12, 21, 18, // second face
            2, 5, 17, 2, 17, 14, // third face
            8, 11, 23, 8, 23, 20, // fourth face
            1, 13, 19, 1, 19, 7, // fifth face
            4, 16, 22, 4, 22, 10, // sixth face
        ];

        Cube {
            vertices: vertices_cube,
            indices: indices_cube,
            center: center,
        }
    }

    pub fn generate_texture_coords(texture_corer: TextureCorner) -> [f32;2] {
        match texture_corer {
            TextureCorner::BottomLeft => {
                [0.0, 0.0]
            },
            TextureCorner::BottomRight => {
                [1.0, 0.0]
            },
            TextureCorner::TopLeft => {
                [0.0, 1.0]
            },
            TextureCorner::TopRight => {
                [1.0, 1.0]
            }
        }
    }

    pub fn generate_cube_corner_coords(center_point: (f32,f32,f32), side_length: f32, cube_corner: CubeCorner) -> [f32;3] {
        match cube_corner {
            CubeCorner::COORDS_A => {
                [center_point.0 + side_length / 2.0, center_point.1 + side_length / 2.0, center_point.2 - side_length / 2.0]
            },
            CubeCorner::COORDS_B => {
                [center_point.0 - side_length / 2.0, center_point.1 + side_length / 2.0, center_point.2 - side_length / 2.0]
            },
            CubeCorner::COORDS_C => {
                [center_point.0 + side_length / 2.0, center_point.1 - side_length / 2.0, center_point.2 - side_length / 2.0]
            },
            CubeCorner::COORDS_D => {
                [center_point.0 - side_length / 2.0, center_point.1 - side_length / 2.0, center_point.2 - side_length / 2.0]

            },
            CubeCorner::COORDS_E => {
                [center_point.0 + side_length / 2.0, center_point.1 + side_length / 2.0, center_point.2 + side_length / 2.0]
            },
            CubeCorner::COORDS_F => {
                [center_point.0 - side_length / 2.0, center_point.1 + side_length / 2.0, center_point.2 + side_length / 2.0]
            },
            CubeCorner::COORDS_G => {
                [center_point.0 + side_length / 2.0, center_point.1 - side_length / 2.0, center_point.2 + side_length / 2.0]
            },
            CubeCorner::COORDS_H => {
                [center_point.0 - side_length / 2.0, center_point.1 - side_length / 2.0, center_point.2 + side_length / 2.0]
            },
        }
    }


}
pub enum CubeCorner {
    COORDS_A,
    COORDS_B,
    COORDS_C,
    COORDS_D,
    COORDS_E,
    COORDS_F,
    COORDS_G,
    COORDS_H,
}

pub enum TextureCorner {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}


#[test]
fn test_generate_cube_corner_coords() {
    let cube = Cube::new(0.1, (0.0, 0.0, 0.0));

    assert_eq!([1.0,1.0,1.0], Cube::generate_cube_corner_coords((0.0,0.0,0.0), 2.0));
}



