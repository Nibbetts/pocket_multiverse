mod universes;


fn main() {
    println!("Welcome to the Pocket Multiverse.");

    println!("Loading 'Hambone' pocket universe...");

    let pocket = universes::Universe::load(String::from("hambone"));
}

