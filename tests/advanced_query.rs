use sceller::prelude::*;

#[derive(Debug)]
struct Position(i32, i32);
#[derive(Debug)]
struct Health(u16);
#[derive(Debug)]
struct Enemy;

struct PlayerResource(String);

#[test]
fn query_functions() -> Result<()> {
    let world = init_world()?;

    let query = world.query();
    query.query_fn(&update_healths);
    query.query_fn(&update_enemies);

    query.query_fn(&update_healths_and_positions_seperately);

    Ok(())
}

#[test]
fn query_functions_mut() -> Result<()> {
    let world = init_world()?;

    let query = world.query();
    
    query.query_fn(&update_healths);
    query.query_fn(&change_healths);
    query.query_fn(update_healths);

    Ok(())
}

fn change_healths(health: FnQueryMut<Health>) {
    for mut hp in health.into_iter() {
        hp.0 += 10;
    }
}

fn update_healths(healths: FnQuery<Health>) {
    for thing in healths.into_iter() {
        println!("{:?}", thing);
    }
}

fn update_healths_and_positions_seperately(healths: FnQuery<Health>, positions: FnQueryMut<Position>) {
    for hp in healths.into_iter() {
        println!("Health at {:?}", hp);
    }
    for mut pos in positions.into_iter() {
        pos.0 += 12;
        println!("is {:?}", pos);
    }
}

fn update_enemies(enemies: FnQuery<(Enemy, Health, Position)>) {
    for (_, hp, pos) in enemies.iter() {
        println!("Enemy: {:?}, {:?}", hp, pos);
    }
}

#[test]
fn auto_querys() -> Result<()> {
    let world = init_world()?;

    let query = world.query(); let auto = query.auto::<Health>();

    assert_eq!(auto.len(), 3);

    let mut iter = auto.into_iter();

    assert_eq!(iter.next().unwrap().0, 12);
    assert_eq!(iter.next().unwrap().0, 6);
    assert_eq!(iter.next().unwrap().0, 15);

    Ok(())
}

fn init_world() -> Result<World> {
    let mut world = World::new();

    world.spawn().insert_checked(Position(0, 0))?.insert_checked(Health(15))?;
    world.spawn().insert_checked(Position(12, 10))?.insert_checked(Health(6))?;
    world.spawn().insert_checked(Position(6, 6))?.insert_checked(Health(12))?.insert_checked(Enemy)?;

    world.insert_resource(PlayerResource("Loafoflead".to_owned()));

    Ok(world)
}