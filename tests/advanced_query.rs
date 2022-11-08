use sceller::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position(i32, i32);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Health(u16);
#[derive(Debug)]
struct Enemy;

struct PlayerResource(String);

#[test]
fn test_fn_query() -> Result<()> {
    let world = init_world()?;

    let query = world.query();

    query.query_fn(list_healths);

    Ok(())
}

fn list_healths(hps: FnQuery<&Health>) {
    let mut iter = hps.iter();

    assert_eq!(iter.next().unwrap().0, 12);
    assert_eq!(iter.next().unwrap().0, 6);
    assert_eq!(iter.next().unwrap().0, 15);
}

#[test]
fn test_mut_fn_query() -> Result<()> {
    let world = init_world()?;

    let query = world.query();

    query.query_fn(list_healths);
    query.query_fn(edit_healths);
    query.query_fn(list_new_healths);

    Ok(())
}

fn edit_healths(hps: FnQuery<&mut Health>) {
    for mut i in hps.iter() {
        i.0 += 1;
    }
}

fn list_new_healths(hps: FnQuery<&Health>) {
    let mut iter = hps.iter();

    assert_eq!(iter.next().unwrap().0, 13);
    assert_eq!(iter.next().unwrap().0, 7);
    assert_eq!(iter.next().unwrap().0, 16);
}

#[test]
fn test_tuple_fn_query() -> Result<()> {
    let world = init_world()?;

    let query = world.query();

    query.query_fn(list_healths_and_poses);
    query.query_fn(one_mut_and_one_not);
    query.query_fn(two_mut);
    query.query_fn(make_sure);

    query.query_fn(test_intoiter);

    Ok(())
}

fn list_healths_and_poses(query: FnQuery<(&Health, &Position)>) {
    let mut iter = query.iter();

    let (hp, pos) = iter.next().unwrap();
    assert_eq!(*hp, Health(12));
    assert_eq!(*pos, Position(6, 6));

    let (hp, pos) = iter.next().unwrap();
    assert_eq!(*hp, Health(6));
    assert_eq!(*pos, Position(12, 10));

    let (hp, pos) = iter.next().unwrap();
    assert_eq!(*hp, Health(15));
    assert_eq!(*pos, Position(0, 0));
}

fn one_mut_and_one_not(query: FnQuery<(&mut Health, &Position)>) {
    for (mut h, _) in query.iter() {
        h.0 += 1;
    }
}

fn two_mut(query: FnQuery<(&mut Health, &mut Position)>) {
    for (mut h, mut pos) in query.iter() {
        h.0 += 1;
        pos.1 = 3;
    }
}

fn make_sure(query: FnQuery<(&Health, &Position)>) {
    let mut iter = query.iter();

    let (hp, pos) = iter.next().unwrap();
    assert_eq!(*hp, Health(14));
    assert_eq!(*pos, Position(6, 3));

    let (hp, pos) = iter.next().unwrap();
    assert_eq!(*hp, Health(8));
    assert_eq!(*pos, Position(12, 3));

    let (hp, pos) = iter.next().unwrap();
    assert_eq!(*hp, Health(17));
    assert_eq!(*pos, Position(0, 3));
}

fn test_intoiter(query: FnQuery<(&Health, &Position, &mut Enemy)>) {
    for q in query {
        println!("{}", q.0.0);
    }
}

// #[test]
// fn test_mutable_iteration() -> Result<()> {
//     let world = init_world()?;

//     let query = world.query();
//     query.query_fn(mutability_test);

//     Ok(())
// }

// fn mutability_test(healths: FnQueryMut<Health>) {
//     let copy = healths.clone();

//     for h in &healths {
//         for mut h2 in &healths {
//             h2.0 += h.0;
//         }
//     }
// }

// #[test]
// fn query_functions() -> Result<()> {
//     let world = init_world()?;

//     let query = world.query();
//     query.query_fn(&update_healths);
//     query.query_fn(&update_enemies);

//     query.query_fn(&update_healths_and_positions_seperately);

//     Ok(())
// }

// #[test]
// fn query_functions_mut() -> Result<()> {
//     let world = init_world()?;

//     let query = world.query();

//     // test(|h: &Health| { print!("hi") });

//     // <&Health as FnQueryType>::map_ref(&std::cell::RefCell::new(5));

//     query.query_fn(update_healths);
//     query.query_fn(change_healths);
//     query.query_fn(update_healths_and_positions_seperately);

//     Ok(())
// }

// fn change_healths(health: FnQuery<&Health>) {
//     for hp in health.into_iter() {
//         hp.0;
//     }
// }

// // fn test<'a, T>(h: fn(T)) 
// // where T: FnQueryType<'a>
// // {
// //     T::map_ref(&std::cell::RefCell::new(5));
// // }

// fn update_healths(healths: FnQuery<&mut Health>) {
//     for mut thing in &healths {
//         println!("{:?}", thing);
//         thing.0 += 5;
//     }
// }

// fn update_healths_and_positions_seperately(healths: FnQuery<Health>, positions: FnQuery<Position>) {
//     for mut hp in healths.into_iter() {
//         println!("Health at {:?}", hp);
//         hp.0 += 12;
//     }

//     for mut pos in positions.into_iter() {
//         pos.0 += 12;
//         println!("is {:?}", pos);
//     }
// }

// fn update_enemies(enemies: FnQuery<(&mut Enemy, &Health)>) {
//     for (mut enem, hp) in enemies.iter() {
//         println!("Enemy: {:?}, ", hp);
//     }
// }

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