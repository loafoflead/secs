//! # Query 
//! 
//! The query Module defines the Query struct, which is the primary way of interfacing with 
//! entities in the ECS. They are intended to be created by the [struct.World.html] and 
//! then filled out and run.

use super::*;
use super::auto_query::{AutoQuery, AutoQueryMut};
use super::query_entity::*;

//
// ideas: turn Query result into tuple of Vec<> of all different types
//

#[derive(Debug)]
/**
A struct used to interface with the ECS.

Contains a map of components included and a reference to the Entites struct, as well
as a vector of the type_ids contained in the query for ease of use.
 */
pub struct Query<'a> {
    map: u128,
    pub(super) entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    /**
    Creates and returns a new Query struct.
    
    Takes an immutable reference to an entites struct.
     */
    pub fn new(entities: &'a Entities) -> Self {
        Self { map: 0, entities, type_ids: Vec::new() }
    }

    /**
    Function that combines the bitmask of the component type given
    with the query's current bitmap.
    
    Essentially adding the type to the query.
    
    Panics if the component queried doesn't exist in the entites struct passed in.
    
    ```
    use sceller::prelude::*;
    
    struct Component1(pub i8);
    struct Component2(pub char);
    
    let mut entities = Entities::default();
    // add in a dummy entity
    entities.create_entity()
        .insert_checked(Component1(-5)).unwrap()
        .insert_checked(Component2('r')).unwrap();
    
    let query_res = Query::new(&entities)
         .with_component_checked::<Component1>().unwrap()
         .with_component_checked::<Component2>().unwrap()
         .run();
    
    let n1s = &query_res[0];
    let n2s = &query_res[1];
    
    assert_eq!(n1s.len(), n2s.len());
    assert_eq!(n1s.len(), 1);
    
    ```
     */
    pub fn with_component<T: Any>(&mut self) -> &mut Self {
        self.with_component_checked::<T>().unwrap()
    }

    /**
    Function that combines the bitmask of the component type given
    with the query's current bitmap.
    
    Essentially adding the type to the query.
    
    Returns an error if the component queried doesn't exist in the entites struct passed in.
    
    ```
    use sceller::prelude::*;
    
    struct Component1(pub i8);
    struct Component2(pub char);
    
    let mut entities = Entities::default();
    // add in a dummy entity
    entities.create_entity()
        .insert_checked(Component1(-5)).unwrap()
        .insert_checked(Component2('r')).unwrap();
    
    let query_res = Query::new(&entities)
         .with_component_checked::<Component1>().unwrap()
         .with_component_checked::<Component2>().unwrap()
         .run();
    
    let n1s = &query_res[0];
    let n2s = &query_res[1];
    
    assert_eq!(n1s.len(), n2s.len());
    assert_eq!(n1s.len(), 1);
    
    ```
     */
    pub fn with_component_checked<T: Any>(&mut self) -> eyre::Result<&mut Self> {
        let typeid = TypeId::of::<T>();
        if let Some(bitmask) = self.entities.get_bitmask(&typeid) {
            self.map |= bitmask;
            self.type_ids.push(typeid);
        } else {
            return Err(QueryError::UnregisteredComponentError.into())
        }

        Ok(self)
    }

    /**
    Executes and returns the result of a query in the form of a vector of vectors 
    of [ComponentType](types.ComponentType.html).

    ```
    use sceller::prelude::*;

    struct Component1(pub i8);
    struct Component2(pub char);

    let mut entities = Entities::default();
    // add in a dummy entity
    entities.create_entity()
        .insert_checked(Component1(-5)).unwrap()
        .insert_checked(Component2('r')).unwrap();

    entities.create_entity()
        .insert_checked(Component1(120)).unwrap()
        .insert_checked(Component2('b')).unwrap();

    let query_res = Query::new(&entities)
         .with_component_checked::<Component1>().unwrap()
         .with_component_checked::<Component2>().unwrap()
         .run();

    let n1s = &query_res[0];
    let n2s = &query_res[1];

    let first1 = n1s[0].borrow();
    let first1 = first1.downcast_ref::<Component1>().unwrap();
    assert_eq!(first1.0, -5);

    let first2 = n2s[0].borrow();
    let first2 = first2.downcast_ref::<Component2>().unwrap();
    assert_eq!(first2.0, 'r');

    let second1 = n1s[1].borrow();
    let second1 = second1.downcast_ref::<Component1>().unwrap();
    assert_eq!(second1.0, 120);

    let second2 = n2s[1].borrow();
    let second2 = second2.downcast_ref::<Component2>().unwrap();
    assert_eq!(second2.0, 'b');
    ```
     */
    pub fn run(&mut self) -> Vec<Vec<ComponentType>> {
        // signifies that we have no valid components to query
        if self.map == 0 {
            return vec![]
        }

        let indexes = self.entities.map.iter().enumerate().filter_map(|(index, map)| {
            if map & self.map == self.map {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

        self.type_ids.iter().map(|typeid| {
            let components = self.entities.components.get(typeid).unwrap();
            let mut query_components = Vec::new();
            for index in &indexes {
                query_components.push(components[*index].clone());
            }
            query_components.into_iter().flatten().collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<ComponentType>>>()
    }

    /**
    Executes the [Query] and returns the result in the form of a vector or [QueryEntity]s. 

    ```
    use sceller::prelude::*;

    struct Component1(i8);
    struct Component2(char);

    let mut ents = Entities::default();

    // add in a dummy entity
    ents.create_entity()
        .insert_checked(Component1(-5)).unwrap()
        .insert_checked(Component2('r')).unwrap();

    let mut query = Query::new(&ents);

    let entities: Vec<QueryEntity> = query.with_component_checked::<Component1>().unwrap().run_entity().unwrap();

    assert_eq!(entities.len(), 1);

    for e in entities {
        assert_eq!(e.id, 0);
        let mut component1: RefMut<Component1> = e.get_component_mut::<Component1>().unwrap();
        component1.0 += 1;
        assert_eq!(component1.0, -4);
    }
    ```

    Essentially provides a more user-friendly way of making queries, remains non-destructive of the 
    [Entities] object passed in.
     */
    pub fn run_entity(&self) -> eyre::Result<Vec<QueryEntity>> {
        // signifies that we have no valid components to query
        if self.map == 0 {
            return Err(QueryError::UnregisteredComponentError.into());
        }

        Ok(self.entities.map.iter().enumerate().filter_map(|(index, map)| {
            if map & self.map == self.map {
                Some(QueryEntity::new(index, self.entities))
            } else {
                None
            }
        })
        .collect::<Vec<QueryEntity>>())
    }

    /**
    Quick and dirty way of querying one specific component.

    # Examples

    ```
    use sceller::prelude::*;

    struct Health(u32); struct Speed(f32);

    let mut ents = Entities::default();

    ents.create_entity().insert(Health(12)).insert(Speed(89.0f32));
    ents.create_entity().insert(Health(1202)).insert(Speed(1.0f32));
    ents.create_entity().insert(Health(3)).insert(Speed(1204.02f32));

    // let's say we just want to get all of the health components immutably and print them out.

    {
        let query = Query::new(&ents);
        let auto_query = query.auto::<Health>(); // use turbofish syntax to define the type to query for.

        // we can then iterate over the auto query:
        for health in auto_query {
            println!("Health value: {}", health.0);
        }
    }

    // the same process can be done but with mutable borrows:

    {
        let query = Query::new(&ents);
        let mut auto_query = query.auto_mut::<Health>(); // use turbofish syntax to define the type to query for.

        // we can then iterate over the auto query:
        for mut health in auto_query {
            health.0 = 10;
        }
    }

    // now we can assert that all health values were set to '10'

    {
        let query = Query::new(&ents);
        let auto_query = query.auto::<Health>(); // use turbofish syntax to define the type to query for.

        // we can then iterate over the auto query:
        for health in auto_query {
            assert_eq!(health.0, 10);
        }
    }
    ```

    This form of query uses a struct that implements IntoIterator, as well as an iterator form.
    The ECS's interior mutability architecture permits this kind of thing.

    For more info on the implementation, check the source or the documentation for
    [super::auto_query].
     */
    pub fn auto<T: Any>(&self) -> AutoQuery<T> {
        AutoQuery::new(&self.entities)
    }
    
    /**
    Quick and dirty way of querying one specific component mutably.

    # Examples

    ```
    use sceller::prelude::*;

    struct Health(u32); struct Speed(f32);

    let mut ents = Entities::default();

    ents.create_entity().insert(Health(12)).insert(Speed(89.0f32));
    ents.create_entity().insert(Health(1202)).insert(Speed(1.0f32));
    ents.create_entity().insert(Health(3)).insert(Speed(1204.02f32));

    // let's say we just want to get all of the health components immutably and print them out.

    {
        let query = Query::new(&ents);
        let auto_query = query.auto::<Health>(); // use turbofish syntax to define the type to query for.

        // we can then iterate over the auto query:
        for health in auto_query {
            println!("Health value: {}", health.0);
        }
    }

    // the same process can be done but with mutable borrows:

    {
        let query = Query::new(&ents);
        let mut auto_query = query.auto_mut::<Health>(); // use turbofish syntax to define the type to query for.

        // we can then iterate over the auto query:
        for mut health in auto_query {
            health.0 = 10;
        }
    }

    // now we can assert that all health values were set to '10'

    {
        let query = Query::new(&ents);
        let auto_query = query.auto::<Health>(); // use turbofish syntax to define the type to query for.

        // we can then iterate over the auto query:
        for health in auto_query {
            assert_eq!(health.0, 10);
        }
    }
    ```

    This form of query uses a struct that implements IntoIterator, as well as an iterator form.
    The ECS's interior mutability architecture permits this kind of thing.

    For more info on the implementation, check the source or the documentation for
    [super::auto_query].
     */
    pub fn auto_mut<T: Any>(&self) -> AutoQueryMut<T> {
        AutoQueryMut::new(&self.entities)
    }

    /**
    Gets the indexes of all the components in this query and fills them into a passed buffer.
    
    ```
    use sceller::prelude::*;
    
    struct Hi(u8);
    struct Hello(usize);
    
    let mut ents = Entities::default();
    
    ents.create_entity()
        .insert_checked(Hi(9)).unwrap()
        .insert_checked(Hello(1242359)).unwrap();
    ents.create_entity()
        .insert_checked(Hi(1)).unwrap()
        .insert_checked(Hello(1259)).unwrap();
    
    let mut indexes = Vec::new();
    
    let query1 = Query::new(&ents).with_component_checked::<Hi>().unwrap().read_indexes_to_buf(&mut indexes).run();
    
    // asserts that the number of 'Hi' components is equal to the number of entities. In occurence, this is correct.
    assert_eq!(indexes.len(), *&query1[0].len());
    ```
    
    All this function does in essence is loop over the inner 'map' of the entities, which 
    stores their respective bitmasks, and do the & product of it and the Query object's bitmask map.
    
    It pushes these indexes into a vector and then places this into 'buf'.
     */
    pub fn read_indexes_to_buf(&mut self, buf: &mut Vec<usize>) -> &mut Self {
        *buf = self.entities.map.iter().enumerate().filter_map(|(index, map)| {
            if map & self.map == self.map {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
        self
    }
}

// Trait implementations
impl<'a> std::fmt::Display for Query<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(thiserror::Error, Debug)]
pub enum QueryError {
    #[error("Attempted to query an unregistered component, maybe you forgot to register it?")]
    UnregisteredComponentError,
    #[error("QueryEntity contains out of bounds components.")]
    OutOfBoundsIdError,
}

#[cfg(test)]
mod tests {
    use std::cell::{Ref, RefMut};

    use super::*;

    #[test]
    fn auto_query_test() -> Result<()> {
        let mut ents = Entities::default();

        // add in a dummy entity
        ents.create_entity()
            .insert(Component1(-5))
            .insert(Component2('r'));

        let query = Query::new(&ents);
        let auto = query.auto::<Component1>();

        for e in auto {
            // let component = e.get_component();
            dbg!(e);
        }

        Ok(())
    }

    #[test]
    fn query_for_entity_mut() -> eyre::Result<()> {
        let mut ents = Entities::default();

        // add in a dummy entity
        ents.create_entity()
            .insert(Component1(-5))
            .insert(Component2('r'));

        let mut query = Query::new(&ents);

        let entities: Vec<QueryEntity> = query.with_component_checked::<Component1>()?.run_entity()?;

        assert_eq!(entities.len(), 1);

        for e in entities {
            assert_eq!(e.id, 0);
            let mut component1: RefMut<Component1> = e.get_component_mut::<Component1>()?;
            component1.0 += 1;
            assert_eq!(component1.0, -4);
        }

        Ok(())
    }

    #[test]
    fn query_for_entity_ref() -> eyre::Result<()> {
        let mut ents = Entities::default();

        // add in a dummy entity
        ents.create_entity()
            .insert(Component1(-5))
            .insert(Component2('r'));

        let mut query = Query::new(&ents);

        let entities: Vec<QueryEntity> = query.with_component_checked::<Component1>()?.run_entity()?;

        assert_eq!(entities.len(), 1);

        for e in entities {
            assert_eq!(e.id, 0);
            let component1: Ref<Component1> = e.get_component::<Component1>()?;
            assert_eq!(component1.0, -5);
        }

        Ok(())
    }

    #[test]
    fn query_mask_updating() -> eyre::Result<()> {
        let ents = init_entities()?;

        let mut query = Query::new(&ents);
        query.with_component_checked::<Component1>()?
            .with_component_checked::<Component2>()?;

        assert_eq!(query.map, 3);
        assert_eq!(TypeId::of::<Component1>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<Component2>(), query.type_ids[1]);

        Ok(())
    }

    #[test]
    fn run_query() -> eyre::Result<()> {
        let ents = init_entities()?;

        let mut indexes = Vec::new();

        let mut query = Query::new(&ents);
        query.with_component_checked::<Component1>()?
            .with_component_checked::<Component2>()?
            .read_indexes_to_buf(&mut indexes);

        let query_res = query.run();
        let n1s = &query_res[0];
        let n2s = &query_res[1];

        assert_eq!(n1s.len(), n2s.len());
        assert_eq!(n1s.len(), indexes.len());
        assert_eq!(n1s.len(), 2);
        
        let first1 = n1s[0].borrow();
        let first1 = first1.downcast_ref::<Component1>().unwrap();
        assert_eq!(first1.0, -5);

        let first2 = n2s[0].borrow();
        let first2 = first2.downcast_ref::<Component2>().unwrap();
        assert_eq!(first2.0, 'r');

        let second1 = n1s[1].borrow();
        let second1 = second1.downcast_ref::<Component1>().unwrap();
        assert_eq!(second1.0, 120);

        let second2 = n2s[1].borrow();
        let second2 = second2.downcast_ref::<Component2>().unwrap();
        assert_eq!(second2.0, 'b');

        Ok(())
    }

    fn init_entities() -> eyre::Result<Entities> {
        let mut ents = Entities::default();

        // add in a dummy entity
        ents.create_entity()
            .insert(Component1(-5))
            .insert(Component2('r'));

        // add in a second dummy entity
        ents.create_entity()
            .insert(Component1(120))
            .insert(Component2('b'));
        
        Ok(ents)
    }

    #[derive(Debug)]
    struct Component1(pub i8);

    #[derive(Debug)]
    struct Component2(pub char);
}