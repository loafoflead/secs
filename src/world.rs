use std::any::Any;

use crate::resources::Resources;

#[derive(Debug, Default)]
/**
 * World contains the ecs, and is used to interact with it.
 */
pub struct World {
    resources: Resources,
}

impl World {
    /**
     * Constructor function. Initialises all contained structs to their default values.
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Inserts a resource into the World structs inner resource struct. The resource
     * can later be retrieved using get_resource::<T>() or get_resource_mut::<T>()
     * 
     * ```
     * struct ImportantResource(String);
     * 
     * let mut world = World::new();
     * world.insert_resource(ImportantResource(String::from("This is important")));
     * 
     * assert_eq!(
     *      world.resources.values, 
     *      HashMap::from[
     *          (TypeId::of::<ImportantResource>(),
     *          ImportantResource(String::from("This is important"))
     *      )]
     * )
     * 
     * ```
     */
    pub fn insert_resource<T: std::any::Any>(&mut self, res: T) {
        self.resources.add(res);
    }

    /**
     * Returns an immutable reference to a resource from within the World structs resource object.
     * Makes use of Resource::get_ref().
     * 
     * ```
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
}