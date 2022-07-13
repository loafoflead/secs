use sceller::prelude::*;

#[test]
fn delete_entity() -> eyre::Result<()> {
    let mut world = World::new();

    world.spawn()
        .insert_checked(Location(12, 12))?
        .insert_checked(Size(-2))?;

    world.spawn()
        .insert_checked(Location(1049, 20))?
        .insert_checked(Size(19))?;

    world.delete_entity(0)?; // delete 1st entity

    let query = world.query().with_component_checked::<Location>()?.run();

    // assert that the query is only one component
    assert_eq!(query.len(), 1);
    // assert that there is only 1 'Location' components in the system
    assert_eq!(query[0].len(), 1);

    let borrowed_locations = query[0][0].borrow();
    let loc = borrowed_locations.downcast_ref::<Location>().unwrap();

    // assert that now the first location is 1049 (the second component)
    assert_eq!(loc.0, 1049);

    world.spawn().insert_checked(Location(0, 0))?;

    let query = world.query().with_component_checked::<Location>()?.run();

    // assert that there are two location components in the system
    assert_eq!(query[0].len(), 2);

    let borrowed_locations = query[0][0].borrow();
    let loc = borrowed_locations.downcast_ref::<Location>().unwrap();

    // assert that the second location is now the new one
    assert_eq!(loc.0, 0);

    Ok(())
}



#[test]
fn create_entity() -> eyre::Result<()> {
    let pos = Location(2, 12);

    let mut world = World::new();
    // world.register_component::<Location>();
    // world.register_component::<Size>();

    world.spawn()
        .insert_checked(pos)?
        .insert_checked(Size(-2))?;

    Ok(())
}

#[test]
fn test_queries() -> eyre::Result<()> {
    let mut world = World::new();

    let mut indexes = Vec::new();

    world.spawn()
        .insert_checked(Location(12, 12))?
        .insert_checked(Size(-2))?;

    world.spawn()
        .insert_checked(Location(1049, 20))?
        .insert_checked(Size(19))?;

    world.spawn()
        .insert_checked(Location(0, 0))?
        .insert_checked(Size(10))?;

    let query = world.query()
        .with_component_checked::<Location>()?
        .with_component_checked::<Size>()?
        .read_indexes_to_buf(&mut indexes)
        .run();

    let locations = &query[0];
    let sizes = &query[1];

    assert_eq!(locations.len(), sizes.len());

    let first1 = locations[0].borrow();
    let first1 = first1.downcast_ref::<Location>().unwrap();
    assert_eq!(first1.0, 12);

    let first2 = sizes[0].borrow();
    let first2 = first2.downcast_ref::<Size>().unwrap();
    assert_eq!(first2.0, -2);

    Ok(())
}

#[test]
fn delete_component_from_ent() -> eyre::Result<()> {
    let mut world = World::new();

    world.register_component::<Location>();
    world.register_component::<Size>();

    world.spawn()
        .insert_checked(Location(6, 6))?
        .insert_checked(Size(10))?;

    world.spawn()
        .insert_checked(Location(9, 2))?
        .insert_checked(Size(2))?;

    world.delete_component_from_ent_checked::<Location>(0)?;

    let mut indexes = Vec::new();
    let query = world.query().with_component_checked::<Location>()?.with_component_checked::<Size>()?.read_indexes_to_buf(&mut indexes).run();

    assert_eq!(query[0].len(), 1);
    // assert_eq!(query[0], 1);

    Ok(())
}

#[test] 
fn add_component_to_ent() -> eyre::Result<()>{
    let mut world = World::new();

    world.register_component::<Location>();
    world.register_component::<Size>();

    world.spawn()
        .insert_checked(Location(6, 6))?
        .insert_checked(Size(10))?;

    world.spawn()
        .insert_checked(Location(9, 2))?
        .insert_checked(Size(2))?;

    world.insert_component_into_entity(Unique, 0);

    let query = world.query().with_component_checked::<Location>()?.with_component_checked::<Unique>()?.run();

    // the is one 'Unique' struct
    assert_eq!(query[1].len(), 1);

    Ok(())
}

#[test]
fn delete_component() -> eyre::Result<()> {
    let mut world = World::new();

    world.register_component::<Location>();
    world.register_component::<Size>();

    world.spawn()
        .insert(Location(6, 6))
        .insert(Size(10));

    world.spawn()
        .insert(Location(9, 2))
        .insert(Size(2));

    world.unregister_component_checked::<Location>()?;

    let mut query = world.query();
    let query = query.with_component_checked::<Location>();

    assert!(query.is_err());

    Ok(())
}

struct Location(pub i32, pub i32);
struct Size(pub i8);

struct Unique;