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
///     The line segments of its edges (same order as neighbors)
///         represented in 2D for navigation across its surface,
///     The pairs of vertices for each edge (same order as neighbors)
///         by index in Universe
/// 
/// NOTE: The 2D representation of a Triangle is pre-scaled, so that travel
/// across it is straightforward.
struct Triangle {
    normal: stl_io::Normal,
    vertices: [usize; 3],
    triangles: [u32; 3],
    segments: [LineSegment2D; 3],
    edges: [[u32; 2]; 3],
}

/// A 2D line segment, including slope, intercept,
/// and upper and lower bounds, each of form y=mx+b, l <= y <= u.
/// Used to test whether a trajectory passes through it.
struct LineSegment2D {
    m: f32, // Slope
    b: f32, // y-Intercept
    l: f32, // Lower bound
    u: f32, // Upper bound
}

impl LineSegment2D {
    /// Finds the interction between a Trajectory2D and this segment,
    /// if it exists, subtracting from the trajectory's length and returning
    /// how far up the segment from l to u the intersection occurs, scaled to
    /// the interval [0, 1].
    /// This replaces coordinates, because each triangle has its own different
    /// coordinate plane, and 3D coordinates might be unreliable for
    /// calculating intersections of lines.
    fn intersect(&self, t: &Trajectory2D) -> Option<(f32, Interp)> {
        // Intersection: y = m1x + b1 = m2x + b2 -> x = (b2-b1) / (m1-m2)
        // Occurs at: y = m1(b2-b1)/(m1-m2) + b1
        if t.m == self.m { None } else {
            let x = (self.b - t.b) / (t.m - self.m);
            let y = t.m * x + t.b;

            // Cover all the base cases where no intersection happens
            if (y < self.l) | (y > self.u) { return None };
            match t.bound {
                LineBound::Upper if y > t.y => return None,
                LineBound::Lower if y < t.y => return None,
                _ => { // An intersection occurs if we go far enough
                    // Find the distance to the intersection point
                    let d = distance2D(x, y, t.x, t.y);
                    // Find the distance traveled and return it and the
                    // intersection point as Some Interp between the ends of the
                    // line segment
                    Some((d, Interp((y - self.l) / (self.u - self.l))))
                },
            }
        }
    }
}

/// An interpolation, or ratio, between two arbitrary things, from [0, 1]
struct Interp(f32);
// /// A Point in 2D space. Position, not movement or direction.
// struct Point2D { x: f32, y: f32 }

/*
If we can do it all in 3D without having problems when lines barely miss
because of imperfect calculations and representations, then that would likely
be faster than converting back and forth between various 2D frames of reference,
but I'm not sure if we can do so.

Trajectory3D:

[tax, tby, tcz] + [dx, ey, fz], t:[0, T]
|                 |             |     ^length remaining; must be updated after movement
|                 |             ^representation of moving from t_0 to T. t must be non-negative
|                 ^starting point of trajectory; must be updated after movement
^velocities in each dimention based on trajectory, represented as a vector.
[a, b, c] must be of unit length, or length gets skewed - unless we want 4D bending of space.

Well, that sort of represents positions. If instead we want to represent velocities,
length of travel would be harder, but speeds might make more sense? I'm confused.
*/

fn distance2D(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).powf(0.5)
}

/// A bound on a line, representing the start of a Trajectory
enum LineBound {
    Upper,
    Lower,
}

/// A trajectory across a 2D plane, with slope, intercept, upper or lower bound
/// from which movement originates, and length.
struct Trajectory2D {
    m: f32, // Slope
    b: f32, // y-Intercept
    bound: LineBound, // Whether bound above or below
    x: f32, // start position x
    y: f32, // start position y
    length: f32, // Distance yet to travel
}

impl Trajectory2D {
    /// Moves the trajectory to its end point, changing both start and length
    /// values. For use within a Triangle, as it has no knowledge of boundaries
    fn move_to_end(&mut self) {
        // TODO: move x and y by length in direction of trajectory
        self.length = 0.;
    }

    /// TODO
    fn advance_by() {

    }
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
            &format!("Unable to find {}{}", level_name, MODEL_FORMAT)
        );
    let model = stl_io::read_stl(&mut file).expect(
        &format!("Failed to read {}{}", level_name, MODEL_FORMAT)
    );
    model.validate().expect(
        &format!("{}{} was corrupt", level_name, MODEL_FORMAT)
    ); // TODO: Is this necessary? How time efficient is this validation?
    file = OpenOptions::new().read(true).open(
        LEVEL_PATH.to_string() + level_name + LEVEL_FORMAT).expect(
            &format!("Unable to find {}{}", level_name, LEVEL_FORMAT)
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