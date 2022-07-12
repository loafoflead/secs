//! # Entity Queries
//! 
//! Entity Queries are a more user friendly implementation of the Query.

use std::{any::{Any, TypeId}, cell::{Ref, RefMut}};

use super::{Entities, ComponentError, query::QueryError};


pub struct QueryEntity<'a> {
    pub id: usize,
    entities: &'a Entities, 
}

impl<'a> QueryEntity<'a> {
    /**
    Constructor function, takes the index of the entity queried and a reference to the entities struct.

    ```
    use secs::prelude::*;

    let ents = Entities::default();

    let query = QueryEntity::new(0, &ents); // of course, this will fail if you try to run the query.

    ```
     */
    pub fn new(index: usize, entities: &'a Entities) -> Self {
        Self { id: index, entities }
    }

    /**
    Returns a [Ref] to a component in this [QueryEntity].

    ```
    use secs::prelude::*;

    struct Component1(i8);
    struct Component2(char);

    let mut ents = Entities::default();

    // add in a dummy entity
    ents.create_entity()
        .insert(Component1(-5))
        .insert(Component2('r'));

    let mut query = Query::new(&ents);

    let entities: Vec<QueryEntity> = query
        .with_component::<Component1>().unwrap()
        .run_entity().unwrap();

    assert_eq!(entities.len(), 1);

    for e in entities {
        assert_eq!(e.id, 0);
        let component1: Ref<Component1> = e.get_component::<Component1>().unwrap();
        assert_eq!(component1.0, -5);
    }
    ```
     */
    pub fn get_component<T: Any>(&self) -> eyre::Result<Ref<T>> {
        let typeid = TypeId::of::<T>();
        let components = self.entities.components.get(&typeid).ok_or(ComponentError::UnregisteredComponentError)?;

        let component = components.get(self.id)
            .ok_or(QueryError::OutOfBoundsIdError)?
            .as_ref()
            .ok_or(ComponentError::NonexistentComponentDataError)?;

        let borrow = component.borrow();

        Ok(
            Ref::map(borrow, |any| {
                any.downcast_ref::<T>().unwrap()
            })
        )
    }

    /**
    Returns a [RefMut] to a component in this [QueryEntity].

    ```
    use secs::prelude::*;

    struct Component1(i8);
    struct Component2(char);

    let mut ents = Entities::default();

    // add in a dummy entity
    ents.create_entity()
        .insert(Component1(-5))
        .insert(Component2('r'));

    let mut query = Query::new(&ents);

    let entities: Vec<QueryEntity> = query
        .with_component::<Component1>().unwrap()
        .run_entity().unwrap();

    assert_eq!(entities.len(), 1);

    for e in entities {
        assert_eq!(e.id, 0);
        let component1: Ref<Component1> = e.get_component::<Component1>().unwrap();
        assert_eq!(component1.0, -5);
    }
    ```
     */
    pub fn get_component_mut<T: Any>(&self) -> eyre::Result<RefMut<T>> {
        let typeid = TypeId::of::<T>();
        let components = self.entities.components.get(&typeid).ok_or(ComponentError::UnregisteredComponentError)?;

        let component = components.get(self.id)
            .ok_or(QueryError::OutOfBoundsIdError)?
            .as_ref()
            .ok_or(ComponentError::NonexistentComponentDataError)?;

        let borrow = component.borrow_mut();

        Ok(
            RefMut::map(borrow, |any| {
                any.downcast_mut::<T>().unwrap()
            })
        )
    }
}