mod universes;


fn main() {
    println!("Welcome to the Pocket Multiverse.");

    // We begin with a basic test. In the future we will move on to adding
    // controls to universe loading and such.
    println!("Loading 'Hambone' pocket universe...");
    let pocket = universes::load("hambone");
}

