use crate::entities::Entity;

///
// TODO: should this be moved to the entities file?
struct Spaceship {
    /// The physics Entity we're using to track the ship in space
    entity: Entity,
    /// normalized direction vector representing input direction
    input_direction: (f64, f64),
    /// fixed value
    acceleration: f64,
}

impl Spaceship {
    /// gets the user input direction as a normalized direction vector
    todo!("is the input going to be WASD?");
    fn update_input(&mut self) {
        todo!("implement");
        self.input_direction = (0 as f64, 0 as f64)
    }

    fn update_velocity() {
        todo!("implement");
    }

    /// updates the position of the spaceship
    fn update_position(&mut self, delta_time: f64) {
        todo!("implement");
        // self.velocity * delta_time;
        // self.triangle_idx;
        // self.coords;
    }
}

// impl Entity for Spaceship {
//     fn update(&mut self, delta_time: f64) {
//         self.update_input();
//         self.update_velocity();
//         self.update_position(delta_time);
//     }

//     fn get_position(&self) -> (i64, (f64, f64)) {
//         (self.triangle_idx, self.coords)
//     }

//     fn get_velocity(&self) -> (f64, f64) {
//         self.velocity
//     }
// }
