use secs::World;

#[test]
fn create_entity() -> eyre::Result<()> {
    let pos = Location {x: 2, y: 12};

    let mut world = World::new();
    // world.register_component::<Location>();
    // world.register_component::<Size>();

    world.spawn()
        .insert(pos)?
        .insert(Size(-2))?;

    

    Ok(())
}

struct Location { pub x: i32, pub y: i32 }
struct Size(pub i8);