use std::{any::{Any, TypeId}, rc::Rc, cell::RefCell, collections::HashMap};
use eyre::*;

type ComponentType = Option<Rc<RefCell<dyn Any>>>;

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<ComponentType>>,
    entity_count: usize,
}

impl Entities {
    pub fn register_component<T: Any + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), Vec::new());
    }

    fn fill_new_component<T: Any>(&mut self) -> Result<()> {
        let comps = self.components.get_mut(&TypeId::of::<T>()).ok_or(ComponentError::AutomaticRegistrationError)?;
        for _ in 0..self.entity_count { comps.push(None); }
        Ok(())
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components.iter_mut().for_each(|(_key, value)| {
            value.push(None);
        });
        self.entity_count += 1;
        self
    }

    pub fn insert<T: Any>(&mut self, data: T) -> eyre::Result<&mut Self> {
        // auto register new component types
        if !self.components.contains_key(&TypeId::of::<T>()) {
            // register and initialize with default value of none
            self.register_component::<T>();
            self.fill_new_component::<T>()?;
        }

        if let Some(components) = self.components.get_mut(&data.type_id()) {
            let last_component = components.last_mut().ok_or(ComponentError::NonexistentEntity)?;
            *last_component = Some(Rc::new(RefCell::new(data)));
        } else {
            bail!("Attempted to add a component that was not registered to an entity.");
        }
        Ok(self)
    }


}

#[derive(thiserror::Error, Debug)]
enum ComponentError {
    #[error("Attempt to add component to nothing.")]
    NonexistentEntity,
    #[error("This error should never happen. (Failed to fill fields of newly generated component on the fly)")]
    AutomaticRegistrationError,
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
            .insert(Health(100))?
            .insert(Id(String::from("hi")))?;

        ents.create_entity()
            .insert(Unique)?
            .insert(Health(50))?
            .insert(Id(String::from("hey")))?;

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

    #[derive(Debug)]
    struct Health(u16);
    struct Id(String);

    struct Unique;
}