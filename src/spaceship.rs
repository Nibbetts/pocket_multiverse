//! ## Spaceship

use crate::entities::{ Entity, StepUpdates };

enum InputType {
    Keyboard,
    Mouse,
    AI,
    Uncontrolled,
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

impl Controller {
    fn poll(&self) -> Action {
        match &self.input_type {
            InputType::Keyboard => {
                // TODO: fill in input polling
                let l = false;
                let r = false;
                let u = false;
                let d = false;
                Action {
                    fwd_thrust: (u as u32 as f32)*1. + (d as u32 as f32)*-1.,
                    rot_thrust: (r as u32 as f32)*1. + (l as u32 as f32)*-1.,
                    // Here, right is positive rotation, but this switches later
                    //  for math's sake.
                }
            },
            InputType::Mouse => todo!(),
            InputType::AI => todo!(),
            InputType::Uncontrolled => Action { fwd_thrust: 0., rot_thrust: 0. },
        }
    }
    //TODO: Not sure if polling is correct! This game engine might be event-driven instead,
    //  in which case events might have to be passed by reference in a list to the Controller.
}

struct Action {
    /// Ranges from -1. to 1.; the direction and ratio of input forward thrust
    /// desired; how much the user is trying to change forward/backward motion
    fwd_thrust: f32,
    /// Also -1. to 1.; how much user is trying to change spin
    rot_thrust: f32,
    //TODO: add other buttons etc.
}

///
// TODO: should this be moved to the entities file?
struct Spaceship {
    /// The physics Entity we're using to track the ship in space
    entity: Entity,
    /// The input-gathering object for this spaceship
    controller: Controller,
    /// How powerful/how many engines on this ship;
    /// Note this has nothing to do with ship mass - that is factored in by
    /// the physics Entity, later down the pipeline
    fwd_thrust_factor: f32,
    rev_thrust_factor: f32,
    rrot_thrust_factor: f32,
    lrot_thrust_factor: f32,
    // rstrafe_thrust_factor: f32,
    // lstrafe_thrust_factor: f32,
    // TODO: later, states of things like what thrusters or safeguards are broken, etc.
}

impl Spaceship {
    /// gets the user inputs in an abstracted form from the controller
    fn update_input(&mut self) -> Action {
        self.controller.poll() // TODO: unless event-driven game loop
    }

    /// Translates inputs into frame of reference of the ship and its current
    ///     thruster power and functional-ness, updating its physics body
    ///     entity accordingly.
    /// Should only happen once per game step (once per delta_time),
    ///     and be affected by the delta time
    fn enact_thrust(&mut self, action: Action, delta_time: f32) {
        let mut f = action.fwd_thrust;
        let mut r = action.rot_thrust;
        if f > 0. {
            f *= self.fwd_thrust_factor
        } else {
            f *= self.rev_thrust_factor }; 
        if r > 0. {
            r *= self.rrot_thrust_factor
        } else {
            r *= self.lrot_thrust_factor
        };
        // Forward is to the right when ship angle is zero, but we are applying
        // force in the direction of motion. Also, a right rotation
        // is actually negative. 
        self.entity.apply_force([f, 0.], -r, true, delta_time);

        // It could be fun if your rotational safeguards break AND a thruster
        //  gets jammed!
    }

    /// updates the position of the spaceship
    fn step_entity(&mut self, delta_time: f32) {
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
