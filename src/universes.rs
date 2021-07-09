use std::fs::OpenOptions;
use stl_io;

// CONSTANTS
const MODEL_PATH:   &str = "resources/models/";
const LEVEL_PATH:   &str = "resources/levels/";
const MODEL_FORMAT: &str = ".stl";
const LEVEL_FORMAT: &str = ".toml";


/// A pocket universe, with all data and functions necessary for navigating
/// it preprocessed at universe instantiation.
pub struct Universe {
    name: String,
    vertices: Vec<stl_io::Vertex>,
    faces: Vec<stl_io::IndexedTriangle>,
    scale: f32, // TODO: make 1/(average of edge length)
    // h: f32, // TODO
    // w: f32,
    // d: f32,
}

impl Universe {
    //
}

/// Loads a universe by name from the model and level directories,
/// for its 3D data and metadata, respectively. Performs all the 3D geometry
/// processing necessary to prepare it for orientation and navigation on its
/// surface.
pub fn load(level_name: &str) -> Universe {

    // Load the data from its files
    // TODO: add error catching later, instead of 'panic'ing
    let mut file = OpenOptions::new().read(true).open(
        MODEL_PATH.to_string() + level_name + MODEL_FORMAT).expect(
            format!("Unable to find {}{}", level_name, MODEL_FORMAT)
        );
    let model = stl_io::read_stl(&mut file).expect(
        format!("Failed to read {}{}", level_name, MODEL_FORMAT)
    );
    model.validate().expect(
        format!("{}{} was corrupt", level_name, MODEL_FORMAT)
    ); // TODO: Is this necessary? How time efficient is this validation?
    file = OpenOptions::new().read(true).open(
        LEVEL_PATH.to_string() + level_name + LEVEL_FORMAT).expect(
            format!("Unable to find {}{}", level_name, LEVEL_FORMAT)
        );
    // let metadata = ???

    // TODO: find neighbors and make a graph structure
    // TODO: make sure all triangles have exactly three neighbors

    // Finally, instantiate it and return it
    Universe {
        name: String::from("Hambone"),
        vertices: model.vertices,
        faces: model.faces,
        scale: 1., // TODO: find average edge length, use to give default scale if not included in level metadata
    }
}