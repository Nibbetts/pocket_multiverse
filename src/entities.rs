//! ## Entities

use crate::universes::{ TriID, Bounds, Trajectory2D, l2_norm, set_norm };
use ndarray::{ ArrayView1, Array1 };

pub trait StepUpdates {
    /// called every frame and used to update an object's logic.
    /// delta_time is the time between the last frame and the next. It's used to scale things to how frequently frames are rendered.
    /// delta_time can be a fixed time to make things simpler for now.
    fn step(&mut self, delta_time: f32);
}

/// An Entity is a physics object represented in the Universe.
/// For use, a spaceship or an asteroid object will contain an entity that
///     is used to track its physical representation.
pub struct Entity {
    /// The index of the Triangle that the Entity is currently on, and in the frame of reference of:
    tri: TriID,
    /// Entity's current 2D orientation within its current Triangle's frame of reference:
    coords: [f32; 2],
    angle: f32,
    /// Entity's current 2D velocity:
    velocity: Array1<f32>,
    angular_velocity: f32,
    /// Entity's current base physical properties:
    mass: f32,
    radius: f32,
    restitution: f32,
}

impl Entity {
    /// Apply an instantaneous force to the Entity for a given time delta.
    /// For absolute directions in 2D space, use rel_to_angle=false,
    ///     but to apply it as if internally, rotated by the direction the
    ///     entity is facing, use rel_to_angle=true.
    pub fn apply_force(&mut self, force: [f32; 2], angular_accel: f32, rel_to_angle: bool, delta_time: f32) {
        // force = mass * accel -> accel = force / mass
        let accel_x = force[0] / self.mass * delta_time;
        let accel_y = force[1] / self.mass * delta_time;
        self.velocity[0] += if rel_to_angle { accel_x * self.angle.cos() } else { accel_x };
        self.velocity[1] += if rel_to_angle { accel_y * self.angle.sin() } else { accel_y };
        self.angular_velocity += angular_accel * delta_time;
    }

    /// Slow down velocity and rotation
    pub fn dampen(&mut self, force: f32, angular_accel: f32, delta_time: f32) {
        let accel = (force / self.mass * delta_time).abs();
        let norm = l2_norm(self.velocity.view());

        // Dampen velocity
        if accel >= norm { self.velocity *= 0.; }
        else { set_norm(&mut self.velocity, norm-accel); }

        // Now dampen angular velocity
        let mut aa = angular_accel.abs() * delta_time;
        if aa >= self.angular_velocity.abs() {
            self.angular_velocity = 0.;
        } else {
            if self.angular_velocity < 0. { aa *= -1.; }
            self.angular_velocity -= aa;
        }
    }

    fn bounce(&mut self, other: &Entity) {
        // Change only own physical properties here;
        //  bounce is to be called on both Entities.
        // TODO: beware of bugs where you change something the other depends
        //  upon for accurate calculation of its own bounce! If this is an
        //  issue, we may need to break bounce into two stages, or pass in the
        //  necessary information ourselves, or differentiate bounce and
        //  bounce_as_other, or something.
    }

    fn bounce_off_large(&mut self, other: &LargeEntity) {
        //TODO
    }

    // todo!("call every frame, maybe from an 'engine' struct/file");
    // fn update(&mut self, delta_time: f32) {
    //     //TODO
    // }

    // /// used to access an entity's position (triangle_idx, (coord_x, coord_y))
    // fn get_position(&self) -> (i64, (f64, f64)) {
    //     //TODO: is this function needed?
    // }

    // /// used to access an entity's velocity
    // fn get_velocity(&self) -> (f64, f64) {
    //     //TODO: is this function needed?
    // }
}
impl StepUpdates for Entity {
    fn step(&mut self, delta_time: f32) {
        let mut traj = Trajectory2D::new(
            self.tri,
            self.coords[0],
            self.coords[1],
            (self.velocity.to_owned() * delta_time).view()
        );
        let (tri, x, y) = traj.move_to_end();
        self.tri = tri;
        self.coords = [x, y];
        self.angle += self.angular_velocity * delta_time;
    }
}

/// LargeEntity is for things like planets, black holes, white holes, things in stable orbits, etc.
pub struct LargeEntity {
    /// The index of the Triangle that the Entity is currently on, and in the frame of reference of:
    triangle_idx: usize,
    /// Entity's current 2D orientation within its current Triangle's frame of reference:
    coords: [f32; 2],
    angle: f32,
    /// Entity's base physical properties:
    motion: MotionPattern,
    gravitation: Gravitation,
    radius: f32,
    restitution: f32,
}
impl StepUpdates for LargeEntity {
    fn step(&mut self, delta_time: f32) {
        //TODO:
    }
}

/// If we want a source of gravitation that exists outside of the 2D manifold,
/// in 3D space instead, then we use this entity type.
/// Such an entity will not be visible, except by its effects.
pub struct LargeEntity3D {
    /// Entity's current 3D location
    coords3D: [f32; 3],
    /// Entity's base physical properties:
    motion: MotionPattern3D,
    gravitation: Gravitation,
}
impl StepUpdates for LargeEntity3D {
    fn step(&mut self, delta_time: f32) {
        //TODO:
    }
}

enum MotionPattern {
    Stationary,
    Path { waypoints: Vec<UnivPos>, speeds: Vec<f32> },
    Orbit { center: [f32; 2], radius: f32, angular_velocity: f32 },
    Straight { velocity: [f32; 2], angular_velocity: f32 },
}
enum MotionPattern3D {
    Stationary,
    Oscillation {
        start_pos: [f32; 3],
        end_pos: [f32; 3],
        period: f32,
    }
}

// TODO: determine whether it's worth it to use this in normal entity stuff as well
struct UnivPos {
    tri: TriID,
    coords: [f32; 2],
    angle: f32,
}

enum Gravitation {
    Fixed(f32),
    Oscillating { min: f32, max: f32, period: f32 },
    Random { range: Bounds, max_time: f32 },
}