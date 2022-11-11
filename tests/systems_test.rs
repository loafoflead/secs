use sceller::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position(i32, i32);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Health(u16);
#[derive(Debug)]
struct Enemy;

struct PlayerResource(String);

#[test]
fn test_systems() -> Result<()> {
	let world = init_world()?;
	
	world.run_system(test);
	world.run_system(test2);
	world.run_system(assure_test2);

	Ok(())
}

fn test(res: Res<PlayerResource>, qry: FnQuery<(&Health, &Position)>) {
	assert_eq!(res.get().0, String::from("Loafoflead"));
	
	let mut iter = qry.into_iter();

	let thing = iter.next().unwrap();
	assert_eq!(thing.0.0, 12);
	assert_eq!(*thing.1, Position(6, 6));

	let thing = iter.next().unwrap();
	assert_eq!(thing.0.0, 6);
	assert_eq!(*thing.1, Position(12, 10));

	let thing = iter.next().unwrap();
	assert_eq!(thing.0.0, 15);
	assert_eq!(*thing.1, Position(0, 0));
}

fn test2(_qr: FnQuery<&mut Health>, resmut: ResMut<PlayerResource>) {
	resmut.get().0 = "Hi".to_owned();
}

fn assure_test2(res: Res<PlayerResource>) {
	assert_eq!(res.get().0, "Hi".to_owned());
}

fn init_world() -> Result<World> {
    let mut world = World::new();

    world.spawn().insert_checked(Position(0, 0))?.insert_checked(Health(15))?;
    world.spawn().insert_checked(Position(12, 10))?.insert_checked(Health(6))?;
    world.spawn().insert_checked(Position(6, 6))?.insert_checked(Health(12))?.insert_checked(Enemy)?;

    world.insert_resource(PlayerResource("Loafoflead".to_owned()));

    Ok(world)
}