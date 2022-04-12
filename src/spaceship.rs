//! ## Spaceship

use crate::entities::{ Entity, StepUpdates };

enum InputType {
    Keyboard,
    Mouse,
    AI,
    None,
}

struct Controller {
    /// Current source of input to poll or be notified of
    input_type: InputType,
    // ship calls get_input or it could be a return from an update function.
    // For now: assume this action returns a forward thrust amount and rotational thrust amount.
    //  Also separate inputs for special weapons/turbo buttons, but generic.
    //  All of this can be packaged together as an Action

    // TODO: For storing to find button pushes rather than is_down
    // button1_laststate: ButtonState
}

struct Action {
    /// Ranges from -1. to 1.; the direction and ratio of input thrust desired
    forward_thrust: f32,
    /// Also -1. to 1.; how much user is trying to change spin
    rotational_thrust: f32,
    //TODO: add other buttons etc.
}

///
// TODO: should this be moved to the entities file?
struct Spaceship {
    /// The physics Entity we're using to track the ship in space
    entity: Entity,
    /// The input-gathering object for this spaceship
    controller: Controller,
    // TODO: later, states of things like what thrusters or safeguards are broken, etc.
}

impl Spaceship {
    /// gets the user inputs in an abstracted form from the controller
    fn update_input(&mut self) -> Action {
        todo!("implement");
    }

    /// Translates inputs into frame of reference of the ship and its current
    ///     thruster power and functional-ness, updating its physics body
    ///     entity accordingly
    fn enact_thrust(action: Action) {
        todo!("implement");
    }

    /// updates the position of the spaceship
    fn step_entity(&mut self, delta_time: f64) {
        self.entity.step(delta_time);
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
