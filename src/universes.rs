use std::fs::OpenOptions;
use stl_io;



pub struct Universe {
    name: String,
    vertices: Vec<stl_io::Vertex>,
    faces: Vec<stl_io::IndexedTriangle>,
    scale: f32, // TODO: make average of edge length
    // h: f32, // TODO
    // w: f32,
    // d: f32,
}

impl Universe {
    pub fn load(level_name: String) -> Universe {

        let mut file = OpenOptions::new().read(true).open(level_name.clone() + ".stl").unwrap();
        let stl = stl_io::read_stl(&mut file).unwrap();
        stl.validate().unwrap(); // TODO: Is this necessary? How time efficient is this validation?
        // file = OpenOptions::new().read(true).open(level_name + ".toml").unwrap();

        // TODO: find neighbors and make a graph structure
        // TODO: make sure all triangles have exactly three neighbors

        Universe {
            name: String::from("Hambone"),
            vertices: stl.vertices,
            faces: stl.faces,
            scale: 1., // TODO: find average edge length, use to give default scale if not included in level metadata
        }
    }
}