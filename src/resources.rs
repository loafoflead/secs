use std::{any::{Any, TypeId}, collections::HashMap};

#[derive(Default, Debug)]
/**
A struct storing a hashmap of type id and value pairs. It is used as a resource storage in 
the ecs.
 */
pub struct Resources {
    values: HashMap<TypeId, Box<dyn Any>>
}

impl Resources {
    /**
    Creates and returns a new Resources struct using its Default Implementation.
    
    ```
    use secs::prelude::*;
    
    struct Health(u8);
    
    let mut resources = Resources::new();
    ```
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Inserts any value implementing the std::any::Any trait into the instance of 
    the Resources struct provided.
    
    ```
    use secs::prelude::*;
    
    struct Health(u8);
    
    let mut resources = Resources::new();
    resources.add(Health(10));
    
    assert_eq!(
         resources.get_ref::<Health>().unwrap().0,
         10
    );
    ```
     */
    pub fn add<T: Any>(&mut self, res: T) {
        self.values.insert(TypeId::of::<T>(), Box::new(res));
    }

    /**
    Gets and optionally returns an immutable reference any given resource from the Resources struct provided.
    Makes use of turbofish syntax (::<T>()) as opposed to concrete variables.

    Note: This function internally uses downcast_ref()

    ```
    use secs::prelude::*;

    struct Health(f32);

    let mut resources = Resources::new();
    resources.add(Health(42.0));

    let extracted_health = resources.get_ref::<Health>().unwrap();
    assert_eq!(extracted_health.0, 42.0);
    ```
     */
    pub fn get_ref<T: Any>(&self) -> eyre::Result<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.values.get(&type_id) {
            data.downcast_ref().ok_or(ResourcesError::NonexistentResourceError.into())
        } else {
            Err(ResourcesError::NonexistentResourceError.into())
        }
    }

    /**
    Optionally returns a mutable reference to a value of the given type.
    
    ```
    use secs::prelude::*;
    
    #[derive(Debug, PartialEq)]
    struct Health(i32);
    
    let mut resources = Resources::new();
    resources.add(Health(123));
    
    {
        let mut hp = resources.get_mut::<Health>().unwrap();
        assert_eq!(hp.0, 123);
        hp.0 = 42;
    }
    
    let hp = resources.get_ref::<Health>().unwrap();
    assert_eq!(hp.0, 42);
    ```
     */
    pub fn get_mut<T: Any>(&mut self) -> eyre::Result<&mut T> {
        if let Some(data) = self.values.get_mut(&TypeId::of::<T>()) {
            data.downcast_mut().ok_or(ResourcesError::NonexistentResourceError.into())
        } else {
            Err(ResourcesError::NonexistentResourceError.into())
        }
    }

    /**
    Attempts to delete and return a resource. 
    
    ```
    use secs::prelude::*;
    
    #[derive(Debug, PartialEq)]
    struct Health(i32);
    
    let mut resources = Resources::new();
    resources.add(Health(123));
    
    let hp = resources.get_ref::<Health>().unwrap();
    assert_eq!(hp.0, 123);
    
    resources.delete::<Health>();
    assert_eq!(resources.get_ref::<Health>(), None);
    ```
    
    This function tries to return the value that was stored in the Resources struct, and
    returns None if the type doesn't exist;
    
    ```
    use secs::prelude::*;
    
    #[derive(Debug, PartialEq)]
    struct Health(i32);
    
    struct No;
    
    let mut resources = Resources::new();
    resources.add(Health(123));
    
    let hp = resources.get_ref::<Health>().unwrap();
    assert_eq!(hp.0, 123);
    
    let res = resources.delete::<Health>();
    assert!(res.is_some());
    
    let res = resources.delete::<No>();
    assert!(!res.is_some());
    ```
     */
    pub fn delete<T: Any>(&mut self) -> eyre::Result<T> {
        if let Some(data) = self.values.remove(&TypeId::of::<T>())
        {
            // We must have a value by this point, or it would have failed to retrieve it from 
            // the hashmap. (i hope)
            Ok(*data.downcast::<T>().unwrap())
        } else {
            Err(ResourcesError::NonexistentResourceError.into())
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ResourcesError {
    #[error("Attempt to access non existent resource.")]
    NonexistentResourceError,
}

// Trait implementations
impl std::fmt::Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn add_resource() {
        let mut resources = Resources::new();

        let thing = Thing(12);
        resources.add(thing);

        let retreived_thing = resources.values.get(&TypeId::of::<Thing>()).unwrap();
        let thing2 = retreived_thing.downcast_ref::<Thing>().unwrap();
        assert_eq!(thing2.0, 12);
    }

    #[test] 
    fn get_resource_mut() {
        let mut resources = Resources::new();

        let thing = Thing(12);
        resources.add(thing);

        {
            let other = resources.get_mut::<Thing>().unwrap();
            other.0 = 129;
        }
        let other = resources.get_ref::<Thing>().unwrap();
        assert_eq!(other.0, 129);
    }

    #[test]
    fn get_resource() {
        let mut resources = Resources::new();
        resources.add(Thing(12));

        let thing = resources.get_ref::<Thing>().unwrap();
        assert_eq!(thing.0, 12);
    }

    #[test]
    fn delete_resource() -> eyre::Result<()> {
        let mut resources = init_resources();

        resources.delete::<Thing>()?;
        assert!(resources.get_ref::<Thing>().is_err());
        assert!(!resources.values.contains_key(&TypeId::of::<Thing>()));
        Ok(())
    }

    fn init_resources() -> Resources {
        let mut res = Resources::new();

        res.add(Thing(10));
        res
    }

    #[derive(Debug, PartialEq)]
    struct Thing(i32);
}