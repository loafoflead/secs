use std::any::Any;

use crate::prelude::*;

#[derive(Debug, Default)]
/**
 * World contains the ecs, and is used to interact with it.
 */
pub struct World {
    resources: Resources,
    entities: Entities,
}

// Resource stuff
impl World {
    /**
     * Constructor function. Initialises all contained structs to their default values.
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Inserts a resource into the World structs inner resource struct. The resource
     * can later be retrieved using [get_resource()](struct.World.html#method.get_resource) or [get_resource_mut()](struct.World.html#method.get_resource_mut)
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct ImportantResource(String);
     * 
     * let mut world = World::new();
     * world.insert_resource(ImportantResource(String::from("This is important")));
     * 
     * assert_eq!(
     *      world.get_resource::<ImportantResource>().unwrap().0, 
     *      String::from("This is important"),
     * )
     * 
     * ```
     */
    pub fn insert_resource<T: std::any::Any>(&mut self, res: T) {
        self.resources.add(res);
    }

    /**
     * Optionally returns an immutable reference to a resource from within the World structs resource object.
     * Makes use of [Resources::get_ref()](struct.Resources.html#method.get_ref).
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct FpsCounter(u16);
     * 
     * let mut world = World::new();
     * 
     * world.insert_resource(FpsCounter(60));
     * 
     * let fps = world.get_resource::<FpsCounter>().unwrap();
     * assert_eq!(fps.0, 60);
     * ``` 
     */
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref()
    }

    /**
     * Optionally returns a mutable reference to a resource within the World structs Resources object.
     * Makes use of [Resources::get_mut()](struct.Resources.html#method.get_mut).
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct Thing(u8);
     * 
     * let mut world = World::new();
     * 
     * world.insert_resource(Thing(60));
     * {
     *     let mut thing = world.get_resource_mut::<Thing>().unwrap();
     *     thing.0 = 12;
     * }
     * let thing2 = world.get_resource::<Thing>().unwrap();
     * assert_eq!(thing2.0, 12);
     * ```
     */
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /**
     * Deletes and attempts to return a resource from the World.
     * 
     * See the [Resources](struct.Resources.html) documentation for more information.
     */
    pub fn delete_resource<T: Any>(&mut self) -> Option<T> {
        self.resources.delete::<T>()
    }
}

// Entity component stuff
impl World {
    /**
     * Registers a component into the ECS. This function is only there optionally, as calling [spawn](structs.World.html#method.spawn) will automatically perform this 
     * operation. 
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct Thing(u8);
     * 
     * let mut world = World::new();
     * 
     * world.register_component::<Thing>();
     * ```
     * 
     * Essentially creates a new index in the hashmap storing a vector of empty cells as long as the entity count.
     */
    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>()
    }

    /**
     * Creates a new entity and returns current Entities instance.
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct Thing(u8);
     * 
     * let mut world = World::new();
     * 
     * world.spawn()
     *     .insert(Thing(6));
     * ```
     */
    pub fn spawn(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn delete_component_by_id<T: Any>(&mut self, index: usize) -> eyre::Result<()> {
        self.entities.delete_component_by_entity_id::<T>(index)
    }

    pub fn insert_component_into_entity<T: Any>(&mut self, data: T, index: usize) {
        self.entities.insert_component_into_entity_by_id(data, index);
    }

    pub fn insert_component_into_entity_checked<T: Any>(&mut self, data: T, index: usize) -> eyre::Result<()> {
        self.entities.insert_component_into_entity_by_id_checked(data, index)
    }

    pub fn unregister_component<T: Any>(&mut self) {
        self.entities.delete_component::<T>();
    }

    pub fn unregister_component_checked<T: Any>(&mut self) -> eyre::Result<()> {
        self.entities.delete_component_checked::<T>()
    }
}

// Query stuff 
impl World {
    /**
     * Creates and returns a new query, allowing the user to query for elements in the ECS.
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct Thing(u8);
     * 
     * let mut world = World::new();
     * 
     * world.spawn()
     *     .insert(Thing(9));
     * 
     * let query = world.query().with_component::<Thing>().unwrap().run();
     * 
     * let borrow = query[0][0].borrow();
     * 
     * assert_eq!(borrow.downcast_ref::<Thing>().unwrap().0, 9);
     * ```
     * 
     * Returns a new Query instance with a reference to this World's Entities inside.
     */
    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }
}