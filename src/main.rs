//! # Pocket Multiverse

mod universes;
mod entities;
mod spaceship;
mod graphics;
mod engine;


fn main() {
    println!("Welcome to the Pocket Multiverse.");

    // We begin with a basic test. In the future we will move on to adding
    // controls to universe loading and such.
    println!("Loading 'Hambone' pocket universe...");
    let pocket = universes::Universe::load("hambone").unwrap();
    println!("Success!");

    graphics::test();
}

/* TODO on Entities:
* Make a SmallEntity trait that represents any small object existing in the
    2D space, and implement on it behaviors of movement within the space.
    This trait will be applied to objects such as spaceships, asteroids, etc. that
    are moveable across the universe's surface and are affected by gravity wells, etc.
    - This can be first implemented as a struct instead of a trait for simplicity
        of understanding, and later the methods can be moved into a trait so that they
        can be applied easily to multiple separate structs. Later, we may even make a
        LargeEntity and an Entity trait, too, to further subdivide the behaviors.
        (LargeEntites would include planets, stars, black holes, and other things that
        we would need to assign precise locations or movements to, because stepwise
        simulation of orbits is imperfect. I don't know how to do this yet on a
        curved surface, because then gravity is inconsistent.) (Entity would only
        be useful if there are shared behaviors between Large and Small entities.)
    - A SmallEntity should possess: mass, velocity, angular velocity, radius.
    - A SmallEntity should have a step method representing one step in the
        simulation, in which a Trajectory object (representing movement in that step of
        a certain length in a certain direction) is created, and its functions
        operated as necessary to move the entity across the map.- A SmallEntity
        should have a thrust method which modifies velocity and/or angular velocity
        according to a force vector at the end of an optional moment arm of length
        equal to the entity's radius. Used for both thrust and gravitation.
    - A SmallEntity should have a bounce method which does some collision physics
        magic I don't know yet, and maybe calls the thrust method? We would like to end
        up with a spectrum between perfect bounces and perfect conglomeration upon
        collision. This is also complicated by moment arms (rotation upon collision,
        depending on angles and offsets or something). This is useful because
        spaceships are likely to bounce if they have shields up, crunch if they
        don't, and maybe somewhere in between for weak shields - but even if we
        use a very simple spaceship concept instead of this cool stuff, asteroids
        will bounce decently well. I recommend starting with a simple perfect bounce
        for all situations. Later we can add in more, and then later still maybe we
        can add in behavior for when an entity breaks apart into pieces upon a
        collision with force exceeding some internal integrity value.

* Maybe make a PlayerControl struct that has methods for specific keyboard and
    mouse inputs and translates these to changes within a linked entity.
    However, this will need to be somewhat flexible, and be scaled by internal
    values representing the particular spaceship's acceleration etc. Probably need
    another in between struct representing a Spaceship which stores such values,
    which are then used by the controller. Or maybe just the Spaceship struct is
    needed until multiple possible forms or sources of control are added. Spaceship
    could have method for forward, left, right, and back, for starters.
*/