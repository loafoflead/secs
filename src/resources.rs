use std::{any::{Any, TypeId}, collections::HashMap};

#[derive(Default, Debug)]
/**
 * A struct storing a hashmap of type id and value pairs. It is used as a resource storage in 
 * the ecs.
 */
pub struct Resources {
    values: HashMap<TypeId, Box<dyn Any>>
}

impl Resources {
    /**
     * Creates and returns a new Resources struct using its Default Implementation.
     * 
     * ```
     * struct Health(u8);
     * 
     * let mut resources = Resources::new();
     * 
     * assert_eq!(
     *      resources.values, 
     *      HashMap::default()
     * );
     * assert_eq!(
     *      resources, 
     *      Resources::default(),
     * );
     * ```
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Inserts any value implementing the std::any::Any trait into the instance of 
     * the Resources struct provided.
     * 
     * ```
     * struct Health(u8);
     * 
     * let mut resources = Resources::new();
     * resources.add(Health(10));
     * 
     * assert_eq!(
     *      resources.values, 
     *      HashMap::from([
     *          (TypeId::of::<Health>(), Health(10))
     *      ])
     * );
     * ```
     */
    pub fn add<T: Any>(&mut self, res: T) {
        self.values.insert(TypeId::of::<T>(), Box::new(res));
    }

    /**
     * Gets and optionally returns an immutable reference any given resource from the Resources struct provided.
     * Makes use of turbofish syntax (::<T>()) as opposed to concrete variables.
     * 
     * Note: This function internally uses downcast_ref()
     * 
     * ```
     * struct Health(f32);
     * 
     * let mut resources = Resources::new();
     * resources.add(Health(42.0));
     * 
     * let extracted_health = resources.get_ref::<Health>();
     * assert_eq!(extracted_health.0, 42.0);
     * ```
     */
    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.values.get(&type_id) {
            data.downcast_ref()
        } else {
            None
        }
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
    fn get_resource() {
        let mut resources = Resources::new();

        let thing = Thing(12);
        resources.add(thing);

        let other = resources.get_ref::<Thing>().unwrap();
        assert_eq!(other.0, 12);
    }

    struct Thing(i32);
}