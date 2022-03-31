///
pub trait Entity {
    /// called every frame and used to update the entity's logic.
    /// delta_time is the time between the last frame and the next. It's used to scale things to how frequently frames are rendered.
    /// delta_time can be a fixed time to make things simpler for now.
    todo!("call every frame, maybe from an 'engine' struct/file");
    fn update(&mut self, delta_time: f64);

    /// used to access an entity's position (triangle_idx, (coord_x, coord_y))
    fn get_position(&self) -> (i64, (f64, f64));

    /// used to access an entity's velocity
    fn get_velocity(&self) -> (f64, f64);
}