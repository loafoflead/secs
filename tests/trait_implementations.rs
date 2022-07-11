use secs::prelude::*;

#[test]
fn test_debug() -> eyre::Result<()> {
    let world = init_world()?;

    dbg!(world);

    let world = init_world()?;

    println!("{}", world);
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

struct Foo(u8);
struct Bar(u32);

struct Egg(char);

struct CoolResource {
    eggs: i32,
    egg_name: String,
}