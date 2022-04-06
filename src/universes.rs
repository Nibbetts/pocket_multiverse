use std::fs::OpenOptions;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::Error;
use ndarray::{Array1, arr1, arr2, ArrayView1};
use unordered_pair::UnorderedPair;
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
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    maxdim: f32, // make max(h,w,d)/scale
    count: u32, // triangle count
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
    position: Array1<f32>,
    normal: Array1<f32>,
    //color: Color, // TODO: for regions in space? No, this should be managed by color centers, not by vertex, I think?
}

/// A smart triangular face which knows:
///     Its Normal,
///     Its vertices by index in Universe
///     Its neighboring triangles by index in Universe,
///     The line segments of its edges (same order as neighbors)
///         represented in 2D for navigation across its surface,
///     The pairs of vertices for each edge (same order as neighbors)
///         by index in Universe
/// 
/// NOTE: The 2D representation of a Triangle is pre-scaled, so that travel
/// across it is straightforward.
struct Triangle {
    normal: Array1<f32>,
    // vertices: [usize; 3],
    neighbors: [usize; 3],
    segments: [LineSegment2D; 3],
    // edges: [[u32; 2]; 3],
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
        }// TODO: What happens when Trajectory is flat or vertical??
        // If bound is upper, trajectory should never intersect floor of triangle. For other cases, figure out when slope is < or > triangle side slopes.
        // Make sure to include support for triangles with inversely sloped legs.
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

    /// Constructor
    fn from(x: f32, y: f32, vel2D: ArrayView1<f32>) -> Trajectory2D {
        let m = vel2D[1] / vel2D[0];
        Trajectory2D {
            m,
            b: y - m * x,
            bound: if (vel2D[1] > 0.) || (vel2D[1] == 0. &&  vel2D[0] >= 0.)
                {LineBound::Upper} else {LineBound::Lower},
            x,
            y,
            length: l2_norm(vel2D),
        }
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
pub fn load(level_name: &str) -> Result<Universe, Error> {

    // Load the data from its files
    // TODO: add error catching later, instead of 'panic'ing
    let mut file = OpenOptions::new().read(true).open(
        MODEL_PATH.to_string() + level_name + MODEL_FORMAT)?;
    let model = stl_io::read_stl(&mut file)?;
    model.validate()?;
    file = OpenOptions::new().read(true).open(
        LEVEL_PATH.to_string() + level_name + LEVEL_FORMAT)?;
    // let metadata = ???

    // Find Neighbors and create graph structure

    // Find the triangles adjacent to each vertex and edge
    let (model_v, model_t) = (model.vertices, model.faces);
    let mut v_to_t: HashMap<usize, Vec<&stl_io::IndexedTriangle>> = HashMap::new(); // Vertex index -> Vec<adjacent triangles>
    let mut e_to_t: HashMap<UnorderedPair<usize>, Vec<usize>> = HashMap::new(); // edge -> Vec<adjacent triangles' indices>
    for (c, t) in model_t.iter().enumerate() {
        // Add to vertex-triangle map
        for vi in t.vertices {
            match v_to_t.get(&vi) {
                Some(o) => o.push(t),
                None => {v_to_t.insert(vi, vec![t]);},
            }
        }
        // Add to edge-triangle map
        // NOTE: No need to check for duplicates or holes or zero-area
        //  triangles, since this is done by stl_io::IndexedMesh::validate
        // TODO: Need to check that each vertex normal is no more than 90 deg from each adjacent face normal? (use dot product?)
        for (i1, i2) in [(0, 1), (1, 2), (2, 0)] {
            let e = UnorderedPair(t.vertices[i1], t.vertices[i2]);
            match e_to_t.get(&e) {
                Some(o) => o.push(c),
                None => {e_to_t.insert(e, vec![c]);},
            }
        }
    }
    // Create Vertex structs
    let vertices = Vec::new();
    for (vi, tris) in v_to_t {
        let v = model_v[vi];
        let position = arr1(&[v[0], v[1], v[2]]);
        let len = tris.len(); // get number of triangles adjacent
        let mut normal = normalize(tris.iter().fold( // Find the average normal of the others
            arr1(&[0f32, 0f32, 0f32]), |sum, t| {
                let n = t.normal;
                sum + arr1(&[n[0], n[1], n[2]])
            }));
        vertices.push(Vertex{position, normal});
    }
    // Create Triangle structs
    let triangles = Vec::new();
    for (c, t) in model_t.iter().enumerate() {
        let n = t.normal;
        let normal = arr1(&[n[0], n[1], n[2]]);
        let find_other = |e: UnorderedPair<usize>| {
            let ti = e_to_t.get(&e).unwrap();
            if c == ti[0] { ti[1] } else { ti[0] }
        };
        let via = t.vertices[0]; // Index to vertex
        let vib = t.vertices[1]; // Index to vertex
        let vic = t.vertices[2]; // Index to vertex
        let neighbors = [
            find_other(UnorderedPair(via, vib)),
            find_other(UnorderedPair(vib, vic)),
            find_other(UnorderedPair(vic, via)),
        ];
        // Convert vertices to 2D and find line segments.
        //  We denote the triangle's three vertices as a, b, and c.
        //  We do this by representing the triangle in a new orthonormal basis,
        //      { ab / ||ab||_2, (ab/||ab||_2) X N, N }, where N is the normal
        //      of the triangle. This basis puts a at the origin, aligns ab
        //      (the bottom of the triangle) to the first axis,
        //      and lays the triangle flat in the third. Only one value needs
        //      recorded to represent b, and two for c.
        //  We find ct by first finding a transition matrix m_tu such that
        //      m_tu dot cu = ct, where u is for universe basis and t is for
        //      the 2D triangle basis.
        let ab = vertices[vib].position - vertices[via].position;
        let ab_norm = l2_norm(ab.view());
        let m_tu = arr2(&[[]]); // TODO: convert all code to nalgebra so we have access to cross product! Or just compute it.
        let at = (0., 0.);
        let bt = (ab_norm, 0.);
        let ct = ()
        triangles.push(Triangle { normal, neighbors, segments });
    }
    // TODO: make 1/(average of edge length), but scale all points before finalizing
    // TODO: make sure all triangles have exactly three neighbors

    // Finally, instantiate the universe and return it
    Ok(Universe {
        name: String::from(level_name),
        vertices: vertices,
        triangles: ?,
        maxdim: ?, // TODO: find average edge length, use to give default scale if not included in level metadata
        count: ?,
    })

    //TODO: later, learn how to do it all with references instead of just indices
}

fn l2_norm(x: ArrayView1<f32>) -> f32 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f32>) -> Array1<f32> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

// fn flatten_by_normal