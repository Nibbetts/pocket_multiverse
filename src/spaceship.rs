//! ## Spaceship

use crate::entities::Entity;

enum InputType {
    Keyboard,
    Mouse,
    AI,
    None,
}

struct Controller {
    input_type: InputType,
    // ship calls get_input or it could be a return from an update function.
    // For now: assume this action returns a forward thrust amount and rotational thrust amount. Also separate inputs for special weapons/turbo buttons, but generic
}

///
// TODO: should this be moved to the entities file?
struct Spaceship {
    /// The physics Entity we're using to track the ship in space
    entity: Entity,
    /// The input-gathering object for this spaceship
    controller: Controller,
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
