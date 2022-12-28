pub struct Cube {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub center: (f32, f32, f32),
}

impl Cube {
    pub fn new(side_length: f32, center: (f32, f32, f32)) -> Cube {
        // Because of textures, each vertex needs 3 copies in the current format
        // so that each face can have a proper texture
        let vertices_cube: Vec<f32> = vec![
            //position			//colors			//texture coords
            // Coord A
            center.0 + side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 - side_length / 2.0,
            0.8,
            0.8,
            0.8,
            0.0,
            1.0, // Top left
            center.0 + side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 - side_length / 2.0,
            0.8,
            0.8,
            0.8,
            1.0,
            1.0, // Top right
            center.0 + side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 - side_length / 2.0,
            0.8,
            0.8,
            0.8,
            1.0,
            1.0, // Top right
            // Coord B
            center.0 - side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 - side_length / 2.0,
            0.3,
            0.3,
            0.3,
            1.0,
            1.0, // Top right
            center.0 - side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 - side_length / 2.0,
            0.3,
            0.3,
            0.3,
            0.0,
            1.0, // Top left
            center.0 - side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 - side_length / 2.0,
            0.3,
            0.3,
            0.3,
            0.0,
            1.0, // Top left
            // Coord C
            center.0 + side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 - side_length / 2.0,
            0.1,
            0.1,
            0.1,
            0.0,
            0.0, // Bottom left
            center.0 + side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 - side_length / 2.0,
            0.1,
            0.1,
            0.1,
            1.0,
            0.0, // Bottom right
            center.0 + side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 - side_length / 2.0,
            0.1,
            0.1,
            0.1,
            0.0,
            1.0, // Top left
            // Coord D
            center.0 - side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 - side_length / 2.0,
            0.5,
            0.5,
            0.5,
            1.0,
            0.0, // Bottom right
            center.0 - side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 - side_length / 2.0,
            0.5,
            0.5,
            0.5,
            0.0,
            0.0, // Bottom left
            center.0 - side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 - side_length / 2.0,
            0.5,
            0.5,
            0.5,
            1.0,
            1.0, // Top right
            // Coord E
            center.0 + side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 + side_length / 2.0,
            0.8,
            0.8,
            0.8,
            1.0,
            1.0, // Top right
            center.0 + side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 + side_length / 2.0,
            0.8,
            0.8,
            0.8,
            0.0,
            1.0, // Top left
            center.0 + side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 + side_length / 2.0,
            0.8,
            0.8,
            0.8,
            1.0,
            0.0, // Bottom right
            // Coord F
            center.0 - side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 + side_length / 2.0,
            0.3,
            0.3,
            0.3,
            0.0,
            1.0, // Top left
            center.0 - side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 + side_length / 2.0,
            0.3,
            0.3,
            0.3,
            1.0,
            1.0, // Top right
            center.0 - side_length / 2.0,
            center.1 + side_length / 2.0,
            center.2 + side_length / 2.0,
            0.3,
            0.3,
            0.3,
            0.0,
            0.0, // Bottom left
            // Coord G
            center.0 + side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 + side_length / 2.0,
            0.1,
            0.1,
            0.1,
            1.0,
            0.0, // Bottom right
            center.0 + side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 + side_length / 2.0,
            0.1,
            0.1,
            0.1,
            0.0,
            0.0, // Bottom left
            center.0 + side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 + side_length / 2.0,
            0.1,
            0.1,
            0.1,
            0.0,
            0.0, // Bottom left
            // Coord H
            center.0 - side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 + side_length / 2.0,
            0.5,
            0.5,
            0.5,
            0.0,
            0.0, // Bottom left
            center.0 - side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 + side_length / 2.0,
            0.5,
            0.5,
            0.5,
            1.0,
            0.0, // Bottom right
            center.0 - side_length / 2.0,
            center.1 - side_length / 2.0,
            center.2 + side_length / 2.0,
            0.5,
            0.5,
            0.5,
            1.0,
            0.0, // Bottom right
        ];

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
}
