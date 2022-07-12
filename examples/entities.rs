use secs::prelude::*;

#[derive(Debug)]
struct Health(u32);
struct Speed(u32);

fn main() -> Result<()> {
    let mut world = World::new();

    // insert_checked will return an error if the insertion of a new component fails.
    world.spawn()
        .insert_checked(Health(12))?
        .insert_checked(Speed(15))?;

    // on the other hand, insert will panic if there is an error. (which is unlikely)
    world.spawn()
        .insert(Health(12093))
        .insert(12_u32);

    // Any struct or even primitive that implements the 'Any' trait (so basically everything) can 
    // be a component. As you can see, in our second entity I inserted a u32.

    // This means that when querying for these entities, you will need to use types:

    // This is an old fashioned query, meaning it returns a vector of vectors of components
    let query = world.query()
        .with_component_checked::<Health>()?
        .with_component_checked::<u32>()?
        .run();

    // Note that using 'with_component_checked' isn't neccessary, I'm just using it here for the sake
    // of the example. It can help to explain the problem if something goes wrong. 

    // We can be sure that this query will contain 1 components of type Health, and 1 of type u32:
    assert_eq!(query[0].len(), 1);
    assert_eq!(query[1].len(), 1);
    println!("Asserted that the query contains one item of type 'Health' and one of type 'u32'");
    // This is because the query filtered for entities with both u32 AND Health.
    // Note, queries are laid out in the order they are run in, so this query is laid out: [Health, u32]

    { // placed in a block so that this borrow ends after
        // We know the Healths are in first place.
        let healths = &query[0];

        // we can retrieve the values by doing this:
        let borrow = healths[0].borrow();
        let health1 = borrow.downcast_ref::<Health>().unwrap(); // this is unlikely to fail other than through human error.

        println!("retrieved health struct: {:?}", health1); // print out the health struct 
    }

    //
    // IMPORTANT:
    //
    // IF YOU EVER GET AN ERROR LIKE THIS: "Expected value of type <...whatever> but found &_"
    // IT MEANS YOU ARE USING THE WRONG BORROW FUNCTION!!! AND HAVE IMPORTED std::borrow::Borrow!!!!
    // 
    // So just remove the import and write borrow bc otherwise this query type will NOT WORK!!!
    //

    // you can also iterate over them:

    // iterate over components
    for refcell in &query[0] {
        let borrow = refcell.borrow();
        let health = borrow.downcast_ref::<Health>().unwrap();
        println!("health loop: {:?}", health);
    }

    // And finally you can borrow them mutably. 

    { // this is in a block to avoid ownership issues and 'stuff'
        let mut borrow_mut = query[0][0].borrow_mut(); // set the second hp to 50 
        let mut health = borrow_mut.downcast_mut::<Health>().unwrap();

        // this is the second Health as we queried for Health AND u32 and only the 
        // second entity has both.

        health.0 = 50;
    }

    let new_query = world.query().with_component::<Health>().run(); // all Health components

    let borrow = new_query[0][1].borrow(); // get second Health that we set to 50
    let first_health=borrow.downcast_ref::<Health>().unwrap();

    assert_eq!(first_health.0, 50);
    println!("Asserted that the value of the first Health struct was changed to 50");

    Ok(())
}