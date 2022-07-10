mod query;

use std::{any::{Any, TypeId}, rc::Rc, cell::{RefCell}, collections::HashMap};
use eyre::*;

pub use self::query::Query;

type ComponentType = Rc<RefCell<dyn Any>>;


#[derive(Debug, Default)]
/**
 * Struct to store Entites and Components in an Entity component System.
 * 
 * Uses bitmaps to execute queries, and as such has a hard limit on the number of individual components that
 * are able to be registered at a time. This particular instance uses a u128, allowing for 128 unique components.
 * 
 * The struct also contains an entity counter to help with automatic registering of components, as well as
 * a hashmap of the different bit masks of each component as well as a vector containing the entity id's 
 * in the form of their bit masks.
 * 
 * The struct stores a 'map' which is a vector of bitmasks that function essentially as entity id's.
 * When an entity is created, a bitmask is appended to this vector with the components the new entity has.
 * So the vector will look something like this (assuming arbitrary entites and components exist in the system):
 * 
 *  map [...0010_1101, ...0111_1111, ...0101_000, ...] where each 1 corresponds to a component the entity has.
 * 
 * In contrast, the 'bit_masks' hashmap will ressemble this:
 * 
 *  bit_masks { Component1: ...0000_0001, Component2: ...0000_0010, Component3: ...0000_0100, ... } where a 1 corresponds to the component's 'id.
 * 
 * Note: in the place of 'Component1' the code actually uses TypeIds, so it would be TypeId::of::<Component1>().
 */
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<ComponentType>>>,
    entity_count: usize,

    bit_masks: HashMap<TypeId, u128>,
    map: Vec<u128>,
}

impl Entities {
    /**
     * Adds new index into the hashmap of components and adds the bitmask of the new type into bitmask vec.
     */
    pub fn register_component<T: Any + 'static>(&mut self) {
        let typeid = TypeId::of::<T>();
        let bitmask = 2_u128.pow(self.components.len() as u32);
        self.components.insert(typeid, Vec::new());
        self.bit_masks.insert(typeid, bitmask);
    }

    /**
     * Convenience function used when auto registering new components.
     * 
     * Basically it makes sure to fill the blank spots in the hashmap with 'None' values
     * to make sure the indexing doesn't get messes up doing queries.
     * 
     * |-----------------------------------------------|
     * |           | ent1 | ent2 | ent3 | newent | ... |
     * |-----------------------------------------------|
     * |Component1 |  56  | 45   |  56  |  None  | ... |
     * |Component2 | None | None |  12  |  None  | ... |
     * |NewComp    |      |      |      |        | ... | <- New component added 
     * |-----------------------------------------------|    automatically with [insert()](struct.Entities.html#method.insert) method.
     * 
     * |-----------------------------------------------|
     * |           | ent1 | ent2 | ent3 | newent | ... |
     * |-----------------------------------------------|
     * |Component1 |  56  | 45   |  56  |  None  | ... |
     * |Component2 | None | None |  12  |  None  | ... |
     * |NewComp    | None | None | None |  None  | ... | <- The new vec is filled with None values by this function. 
     * |-----------------------------------------------|
     */
    fn fill_new_component<T: Any>(&mut self) {
        let comps = self.components.get_mut(&TypeId::of::<T>()).unwrap();
        for _ in 0..self.entity_count { comps.push(None); }
    }

    #[allow(dead_code)]
    /**
     * Convenience function used when auto registering new components but returns an optional value if it fails.
     * 
     * Basically it makes sure to fill the blank spots in the hashmap with 'None' values
     * to make sure the indexing doesn't get messes up doing queries.
     * 
     * |-----------------------------------------------|
     * |           | ent1 | ent2 | ent3 | newent | ... |
     * |-----------------------------------------------|
     * |Component1 |  56  | 45   |  56  |  None  | ... |
     * |Component2 | None | None |  12  |  None  | ... |
     * |NewComp    |      |      |      |        | ... | <- New component added 
     * |-----------------------------------------------|    automatically with [insert()](struct.Entities.html#method.insert) method.
     * 
     * |-----------------------------------------------|
     * |           | ent1 | ent2 | ent3 | newent | ... |
     * |-----------------------------------------------|
     * |Component1 |  56  | 45   |  56  |  None  | ... |
     * |Component2 | None | None |  12  |  None  | ... |
     * |NewComp    | None | None | None |  None  | ... | <- The new vec is filled with None values by this function. 
     * |-----------------------------------------------|
     */
    fn fill_new_component_checked<T: Any>(&mut self) -> Result<()> {
        let comps = self.components.get_mut(&TypeId::of::<T>()).ok_or(ComponentError::AutomaticRegistrationError)?;
        for _ in 0..self.entity_count { comps.push(None); }
        Ok(())
    }

    /**
     * Adds a new entry in each component vector and fills it with a 'None' option value.
     * In effect, adds a new entity into the struct. The entity will be pushed to the end
     * and as such any subsequent calls to [insert()](struct.Entities.html#method.insert) will
     * effect the latest entity added with this function.
     * 
     * ```
     * use secs::prelude::*;
     * use std::any::TypeId;
     * 
     * struct Health(u8);
     * struct Speed(i8);
     * 
     * let mut ents = Entities::default();
     * 
     * ents.create_entity()
     *     .insert_checked(Health(10_u8)).unwrap()
     *     .insert_checked(Speed(-16)).unwrap();
     * ```
     */
    pub fn create_entity(&mut self) -> &mut Self {
        self.components.iter_mut().for_each(|(_key, value)| {
            value.push(None);
        });

        self.map.push(0);

        self.entity_count += 1;
        self
    }

    /**
     * Inserts a component into whatever is the newest newly created entity. Returns Err if the component 
     * 
     * Note: automatically calls [register_component()](struct.Entities.html#method.register_component) and 
     * [fill_new_component()](struct.Entities.html#method.fill_new_component) to streamline the creation of new
     * entities.
     * 
     * ```
     * use secs::prelude::*;
     * use std::any::TypeId;
     * 
     * struct Health(u8);
     * struct Speed(i8);
     * 
     * let mut ents = Entities::default();
     * 
     * ents.create_entity()
     *     .insert_checked(Health(10_u8)).unwrap()
     *     .insert_checked(Speed(-16)).unwrap();
     * ```
     */
    pub fn insert<T: Any>(&mut self, data: T) -> &mut Self {
        // auto register new component types
        if !self.bit_masks.contains_key(&TypeId::of::<T>()) {
            // register and initialize with default value of none
            self.register_component::<T>();
            self.fill_new_component::<T>();
        }

        let map_index = self.map.len() - 1;

        if let Some(components) = self.components.get_mut(&data.type_id()) {
            let last_component = components.last_mut().ok_or(ComponentError::NonexistentEntity).unwrap();
            let typeid = data.type_id();
            *last_component = Some(Rc::new(RefCell::new(data)));

            let bitmask = self.bit_masks.get(&typeid).unwrap();
            self.map[map_index] |= *bitmask;
        } else {
            panic!("Attempted to add a component that was not registered to an entity.");
        }
        self
    }

    /**
     * Inserts a component into whatever is the newest newly created entity. Returns Err if the component isn't registered.
     * 
     * Note: automatically calls [register_component()](struct.Entities.html#method.register_component) and 
     * [fill_new_component()](struct.Entities.html#method.fill_new_component) to streamline the creation of new
     * entities.
     * 
     * ```
     * use secs::prelude::*;
     * use std::any::TypeId;
     * 
     * struct Health(u8);
     * struct Speed(i8);
     * 
     * let mut ents = Entities::default();
     * 
     * ents.create_entity()
     *     .insert_checked(Health(10_u8)).unwrap()
     *     .insert_checked(Speed(-16)).unwrap();
     * ```
     */
    pub fn insert_checked<T: Any>(&mut self, data: T) -> eyre::Result<&mut Self> {
        // auto register new component types
        if !self.bit_masks.contains_key(&TypeId::of::<T>()) {
            // register and initialize with default value of none
            self.register_component::<T>();
            self.fill_new_component_checked::<T>()?;
        }

        let map_index = self.map.len() - 1;

        if let Some(components) = self.components.get_mut(&data.type_id()) {
            let last_component = components.last_mut().ok_or(ComponentError::NonexistentEntity)?;
            let typeid = data.type_id();
            *last_component = Some(Rc::new(RefCell::new(data)));

            let bitmask = self.bit_masks.get(&typeid).unwrap();
            self.map[map_index] |= *bitmask;
        } else {
            bail!("Attempted to add a component that was not registered to an entity.");
        }
        Ok(self)
    }


    pub fn delete_component_by_entity_id<T: Any>(&mut self, index: usize) -> Result<()> {
        let typeid = TypeId::of::<T>();
        let mask = self.bit_masks.get(&typeid).ok_or(ComponentError::UnregisteredComponentError)?;

        self.map[index] ^= *mask;

        Ok(())
    }

    pub fn insert_component_into_entity_by_id<T: Any>(&mut self, data: T, map_index: usize) {
        // auto register new component types
        if !self.bit_masks.contains_key(&TypeId::of::<T>()) {
            // register and initialize with default value of none
            self.register_component::<T>();
            self.fill_new_component::<T>();
        }

        if let Some(components) = self.components.get_mut(&data.type_id()) {
            let replaced_component = components.get_mut(map_index).ok_or(ComponentError::NonexistentEntity).unwrap();
            let typeid = data.type_id();
            *replaced_component = Some(Rc::new(RefCell::new(data)));

            let bitmask = self.bit_masks.get(&typeid).unwrap();
            self.map[map_index] |= *bitmask;
        } else {
            panic!("Attempted to add a component that was not registered to an entity.");
        }
    }

    pub fn insert_component_into_entity_by_id_checked<T: Any>(&mut self, data: T, map_index: usize) -> eyre::Result<()> {
        // auto register new component types
        if !self.bit_masks.contains_key(&TypeId::of::<T>()) {
            // register and initialize with default value of none
            self.register_component::<T>();
            self.fill_new_component_checked::<T>()?;
        }

        if let Some(components) = self.components.get_mut(&data.type_id()) {
            let replaced_component = components.get_mut(map_index).ok_or(ComponentError::NonexistentEntity)?;
            let typeid = data.type_id();
            *replaced_component = Some(Rc::new(RefCell::new(data)));

            let bitmask = self.bit_masks.get(&typeid).ok_or(ComponentError::UnregisteredComponentError)?;
            self.map[map_index] |= *bitmask;
        } else {
            bail!("Attempted to add a component that was not registered to an entity.");
        }
        Ok(())
    }

    pub fn delete_component<T: Any>(&mut self) {
        let (_, bitmask) = self.bit_masks.remove_entry(&TypeId::of::<T>()).unwrap();
        for component_bitmask in &mut self.map {
            *component_bitmask ^= bitmask;
        }
    }

    pub fn delete_component_checked<T: Any>(&mut self) -> eyre::Result<()> {
        let (_, bitmask) = self.bit_masks.remove_entry(&TypeId::of::<T>()).ok_or(ComponentError::UnregisteredComponentError)?;
        for component_bitmask in &mut self.map {
            *component_bitmask ^= bitmask;
        }
        Ok(())
    }

    /**
     * Convenience function to get the bitmask of a given TypeId. 
     * 
     * Returns None if the component requested isn't registered.
     */
    pub fn get_bitmask(&self, typeid: &TypeId) -> Option<u128> {
        self.bit_masks.get(typeid).copied()
    }
}

#[derive(thiserror::Error, Debug)]
enum ComponentError {
    #[error("Attempt to add component to nothing.")]
    NonexistentEntity,
    #[error("This error should never happen. (Failed to fill fields of newly generated component on the fly)")]
    AutomaticRegistrationError,
    #[error("Attempt to make use of unregistered component.")]
    UnregisteredComponentError
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::*;

    #[test]
    fn register_entities() {
        let mut ents = Entities::default();
        ents.register_component::<Health>();
        ents.register_component::<Id>();

        let hp_component = ents.components.get(&TypeId::of::<Health>()).unwrap();

        assert_eq!(hp_component.len(), 0);
        dbg!(ents);
    }

    #[test]
    fn bitmask_update_on_register_entities() {
        let mut ents = Entities::default();
        ents.register_component::<Health>();
        ents.register_component::<Id>();

        let hp_component = ents.bit_masks.get(&TypeId::of::<Health>()).unwrap();

        assert_eq!(*hp_component, 1);
        dbg!(ents);
    }

    #[test]
    fn create_entity() {
        let mut ents = Entities::default();
        ents.register_component::<Health>();
        ents.register_component::<Id>();

        ents.create_entity();
        let hp = ents.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = ents.components.get(&TypeId::of::<Id>()).unwrap();

        assert!(hp.len() == speed.len() && hp.len() == 1);
        assert!(speed[0].is_none());
        assert!(hp[0].is_none());

        dbg!(ents.components);
    }

    #[test]
    fn with_component() -> Result<()> {
        let mut ents = Entities::default();

        ents.create_entity()
            .insert(Health(100))
            .insert(Id(String::from("hi")));

        ents.create_entity()
            .insert(Unique)
            .insert(Health(50))
            .insert(Id(String::from("hey")));

        let health1 = &ents.components.get(&TypeId::of::<Health>()).unwrap()[0];
        let wrapped_health = health1.as_ref().unwrap();
        let borrowed_health = wrapped_health.borrow();
        let hp = borrowed_health.downcast_ref::<Health>().unwrap();

        assert_eq!(hp.0, 100);
        dbg!(hp);

        let hp = ents.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = ents.components.get(&TypeId::of::<Unique>()).unwrap();

        assert!(hp.len() == speed.len() && hp.len() == ents.entity_count);
        // assert!(speed[0].is_none());
        // assert!(hp[0].is_none());

        Ok(())
    }

    #[test]
    fn map_is_updated() -> Result<()> {
        let mut ents = Entities::default();
        // ents.register_component::<Health>();
        // ents.register_component::<Id>();

        ents.create_entity()
            .insert(Health(100))
            .insert(Id(String::from("hi")));

        let entity_map = ents.map[0];
        
        assert_eq!(entity_map, 3);

        ents.create_entity()
            .insert(Id(String::from("hi")));

        let entity_map = ents.map[1];
        
        assert_eq!(entity_map, 2);

        Ok(())
    }

    #[test]
    fn delete_component_by_ent_id() -> Result<()> {
        let mut ents = Entities::default();

        ents.create_entity()
            .insert(Health(100))
            .insert(Id(String::from("hi")));

        ents.create_entity()
            .insert(Health(50))
            .insert(Id(String::from("hey")));

        ents.delete_component_by_entity_id::<Health>(0)?;

        assert_eq!(ents.map[0], 2);

        Ok(())
    }

    #[test] 
    fn add_component_by_ent_id() -> eyre::Result<()> {
        let mut ents = Entities::default();

        ents.create_entity()
            .insert(Health(100))
            .insert(Id(String::from("hi")));

        ents.create_entity()
            .insert(Health(50))
            .insert(Id(String::from("hey")));

        // first entity will be: ...0000_0011
        // after this operation: ...0000_0111
        ents.insert_component_into_entity_by_id(Unique, 0);

        assert_eq!(ents.map[0], 7);

        Ok(())
    }

    #[test]
    fn remove_component() -> eyre::Result<()> {
        let mut ents = Entities::default();

        ents.create_entity()
            .insert_checked(Health(100))?
            .insert_checked(Id(String::from("hi")))?;

        ents.create_entity()
            .insert_checked(Health(50))?
            .insert_checked(Id(String::from("hey")))?;

        assert_eq!(ents.map[0], 3_u128);

        ents.delete_component_checked::<Health>()?;

        // asserts that when querying we will no longer find this component, effectively removing it.
        assert_eq!(ents.map[0], 2_u128);

        Ok(())
    }

    #[derive(Debug)]
    struct Health(u16);
    struct Id(String);

    struct Unique;
}