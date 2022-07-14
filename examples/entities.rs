use sceller::prelude::*;

#[derive(Debug)]
struct Health(u32);
#[derive(Debug)]
struct Speed(u32);

struct Enemy;

fn main() -> Result<()> {
    let mut world = World::new();

    // insert_checked will return an error if the insertion of a new component fails.
    world.spawn()
        .insert_checked(Health(12))?
        .insert_checked(Speed(15))?        
        .insert(Enemy);

    // on the other hand, insert will panic if there is an error. (which is unlikely)
    world.spawn()
        .insert(Health(12093))
        .insert(12_u32)
        .insert(Enemy);

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

    {
        let new_query = world.query().with_component::<Health>().run(); // all Health components

        let borrow = new_query[0][1].borrow(); // get second Health that we set to 50
        let first_health=borrow.downcast_ref::<Health>().unwrap();

        assert_eq!(first_health.0, 50);
        println!("Asserted that the value of the first Health struct was changed to 50");
    }

    // Auto queries are new, and they are just a really speedy way of getting all of *one* component:

    {
        let query = world.query();
        let auto = query.auto::<Health>(); // this is now an iterator over every health in the system.
        
        println!("All health values: (there are {} items)", auto.len());
        for health in auto {
            println!("{:?}", health);
        }
    }   

    // we can also use this to test deleting components from entities
    world.delete_component_from_ent::<Health>(0); // delete health from first entity

    // we can assert there should be 1 health value left in the system:

    {
        let query = world.query();
        let auto = query.auto::<Health>(); // this is now an iterator over every health in the system.

        assert_eq!(auto.len(), 1);
        println!("Asserted that there exists only 1 health component after deleting.");
    }

    {
        // AutoQueries can also be mutable:
        let query = world.query();
        let auto = query.auto_mut::<Health>(); // this is now an iterator over every health in the system.
        
        for mut hp in auto {
            hp.0 = 50;
        }
        // set all health values to 50
    }

    // Another method of querying is with Query Functions.
    // These are a little similar to [bevy](https://bevyengine.org/) systems, although a thousand times more
    // limiting and about a million times less sophisticated.

    // you pass in a function with the query in it's signature to make the query, as so:

    println!("Beginning function queries:");

    let query = world.query();
    query.query_fn(&print_healths); // this will execute this function and fill in the query

    // this function also exists in mutable form:

    query.query_fn_mut(&change_healths);

    query.query_fn(&print_healths); // Verify that the health values have changed
    
    // query functions currently support a tuple field of up to three components:

    query.query_fn(&print_two); // this also works with a function with a tuple of three components right now (maybe more later)

    // I have not yet implemented multiply queries in one function, but i might be able 
    // to wrap my head around it. hopefully.

    Ok(())
}

fn print_healths(healths: FnQuery<Health>) {
    for health in healths.into_iter() {
        println!("{:?}", health);
    }
}

fn change_healths(healths: FnQueryMut<Health>) {
    for mut health in healths.into_iter() {
        health.0 += 100;
    }
}

fn print_two(query: FnQuery<(Speed, Enemy)>) {
    // support tuple destructuring
    for (speed, _) in query.iter() {
        println!("Enemy: {:?}", speed);
    }
}