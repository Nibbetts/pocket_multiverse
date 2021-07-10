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

// TODO:  test and consider whether we need a full graph structure implementation
//  which stores things in Universe by their references instead of by indices. It would
//  probably nearly double the speed of some operations.

/// A smart vertex which knows:
///     Its position,
///     Its neighboring triangles by index in Universe,
///     Its neighboring vertices by index in Universe,
///     Its own 'Normal' (as an average of the Normals of the faces around it)
struct Vertex {
    position: stl_io::Vertex,
    triangles: [u32; 3],
    vertices: [u32; 3],
    normal: stl_io::Normal,
    //color: Color, // TODO: for regions in space?
}

/// A smart triangular face which knows:
///     Its Normal,
///     Its vertices,
///     Its neighboring triangles by index in Universe,
///     The equation coefficients of the lines of its edges
///         (same order as neighbors, each of form z=ax+by+c),
///     The pairs of vertices for each edge (same order as neighbors),
struct Triangle {
    normal: stl_io::Normal,
    vertices: [usize; 3],
    triangles: [u32; 3],
    equations: [[f32; 3]; 3],
    edges: [[u32; 2]; 3],
}

impl Universe {
    //
}

/// Loads a universe by name from the model and level directories,
/// for its 3D data and metadata, respectively.
/// 
/// Performs all the 3D geometry processing necessary to prepare it for
/// locating and navigating on its surface.
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