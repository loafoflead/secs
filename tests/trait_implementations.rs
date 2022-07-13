use sceller::prelude::*;

#[test]
fn test_debug() -> eyre::Result<()> {
    let world = init_world()?;

    dbg!(world);

    let world = init_world()?;

    println!("{}", world);
    Ok(())
}

#[test]
fn test_queries() -> eyre::Result<()> {
    let world = init_world()?;

    let mut query = world.query();
    let query = query.with_component_checked::<Foo>()?.with_component_checked::<Bar>()?.run_entity()?;

    for ent in query {
        if let Ok(foo) = ent.get_component::<Foo>() { println!("foo {:?}", foo); }
        if let Ok(bar) = ent.get_component::<Bar>() { println!("bar {:?}", bar); }
    }

    Ok(())
}

#[test]
fn test_resources() -> eyre::Result<()> {
    let world = init_world()?;

    let cool = world.get_resource::<CoolResource>()?;

    assert_eq!(*cool, CoolResource {
        eggs: 0, egg_name: String::from("Hello")
    });

    Ok(())
}

fn init_world() -> eyre::Result<World> {
    let mut world = World::new();

    world.spawn()
        .insert_checked(Foo(255))?
        .insert_checked(Bar(16000))?;

    world.spawn()
        .insert_checked(Foo(9))?
        .insert_checked(Bar(100))?
        .insert_checked(Egg('g'))?;

    world.spawn()
        .insert_checked(Foo(25))?
        .insert_checked(Bar(190))?;

    world.insert_resource(CoolResource { eggs: 0, egg_name: String::from("Hello") } );

    Ok(world)
}

#[derive(Debug)]
struct Foo(u8);
#[derive(Debug)]
struct Bar(u32);

#[derive(Debug)]
struct Egg(char);

#[derive(Debug, PartialEq)]
struct CoolResource {
    eggs: i32,
    egg_name: String,
}