use secs::World;

#[test]
fn create_entity() -> eyre::Result<()> {
    let pos = Location(2, 12);

    let mut world = World::new();
    // world.register_component::<Location>();
    // world.register_component::<Size>();

    world.spawn()
        .insert(pos)?
        .insert(Size(-2))?;

    

    Ok(())
}

#[test]
fn test_queries() -> eyre::Result<()> {
    let mut world = World::new();

    world.spawn()
        .insert(Location(12, 12))?
        .insert(Size(-2))?;

    world.spawn()
        .insert(Location(1049, 20))?
        .insert(Size(19))?;

    world.spawn()
        .insert(Location(0, 0))?
        .insert(Size(10))?;

    let query = world.query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();

    let locations = query[0];
    let sizes = query[0];

    assert_eq!(locations.len(), sizes.len());

    Ok(())
}

struct Location(pub i32, pub i32);
struct Size(pub i8);