use secs::prelude::*;

#[derive(Debug)]
// resource that should be globally accessible, and can be used at any time and mutated.
struct PlayerResource {
    name: String,
    speed: u32,
}

// Resource that is not registered in the World to demonstrate the error handling.
struct InvalidResource;

// function returns result to catch errors. (not strictly neccessary but I will include it for the sake
// of this example. It helps to know what went wrong.)
fn main() -> Result<()> {
    let mut world = World::new();

    world.insert_resource(PlayerResource { name: "LoafOfLead".to_owned(), speed: 11 });

    // get an immutable reference to the PlayerResource
    let player_res = world.get_resource::<PlayerResource>()?;

    // debug it out!
    println!("Got player resource: {:?}!", player_res);

    let mut player_res = world.get_resource_mut::<PlayerResource>()?;

    player_res.speed += 1;
    player_res.name = "SomebodyElse".to_owned();
    println!("Changed player resource: {:?}?", player_res);

    // this will fail, as we have never registered "InvalidResource" into our World.
    let invalid_res = world.get_resource::<InvalidResource>();
    println!("Error when accessing invalid resource: {:?}.", invalid_res.err());

    Ok(())
}