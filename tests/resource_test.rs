use secs::prelude::*;

#[test]
fn create_and_get_immutable_resource() {
    let mut world = World::new();

    world.insert_resource(SizeResource(12.0));

    let retrieved_size = world.get_resource::<SizeResource>().unwrap();
    assert_eq!(retrieved_size.0, 12.0);
}

#[test]
fn get_resources_mutably() {
    let mut world = World::new();

    world.insert_resource(SizeResource(12.0));
    
    {
        let fps = world.get_resource_mut::<SizeResource>().unwrap();
        fps.0 += 12.0;
    }

    let fps = world.get_resource::<SizeResource>().unwrap();
    assert_eq!(fps.0, 24.0);
}

#[test]
fn delete_resource() -> eyre::Result<()> {
    let mut world = init_world();

    world.delete_resource::<SizeResource>()?;

    assert!(world.get_resource::<SizeResource>().is_err());
    Ok(())
}

fn init_world() -> World {
    let mut world = World::new();

    world.insert_resource(SizeResource(12.0));
    world
}

#[derive(Debug, PartialEq)]
struct SizeResource(f32);