/// An Entity is a physics object represented in the Universe.
/// For use, a spaceship or an asteroid object will contain an entity that
///     is used to track its physical representation.
pub struct Entity {
    /// The index of the Triangle that the Entity is currently on, and in the frame of reference of:
    triangle_idx: usize,
    /// Entity's current 2D orientation within its current Triangle's frame of reference:
    coords: (f32, f32),
    angle: f32,
    /// Entity's current 2D velocity:
    velocity: (f32, f32),
    angular_velocity: f32,
    /// Entity's current base physical properties:
    mass: f32,
    radius: f32,
    restitution: f32,
}

impl Entity {
    fn thrust(&mut self, accel: [f32; 2], angular: f32) {
        //TODO
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

    /// called every frame and used to update the entity's logic.
    /// delta_time is the time between the last frame and the next. It's used to scale things to how frequently frames are rendered.
    /// delta_time can be a fixed time to make things simpler for now.
    todo!("call every frame, maybe from an 'engine' struct/file");
    fn update(&mut self, delta_time: f64) {
        //TODO
    }

    // /// used to access an entity's position (triangle_idx, (coord_x, coord_y))
    // fn get_position(&self) -> (i64, (f64, f64)) {
    //     //TODO: is this function needed?
    // }

    // /// used to access an entity's velocity
    // fn get_velocity(&self) -> (f64, f64) {
    //     //TODO: is this function needed?
    // }
}