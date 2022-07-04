use secs::World;

#[test]
fn create_and_get_immutable_resource() {
    let mut world = World::new();

    world.insert_resource(SizeResource(12.0));

    let retrieved_size = world.get_resource::<SizeResource>().unwrap();
    assert_eq!(retrieved_size.0, 12.0);
}

struct SizeResource(f32);