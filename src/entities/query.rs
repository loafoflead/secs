use super::*;

#[derive(Debug)]
pub struct Query<'a> {
    map: u128,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    /**
     * Creates and returns a new Query struct.
     * 
     * Takes an immutable reference to an entites struct.
     */
    pub fn new(entities: &'a Entities) -> Self {
        Self { map: 0, entities, type_ids: Vec::new() }
    }

    /**
     * Function that combines the bitmask of the component type given
     * with the query's current bitmap.
     * 
     * Essentially adding the type to the query.
     * 
     * Returns an error if the component queried doesn't exist in the entites struct passed in.
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct Component1(pub i8);
     * struct Component2(pub char);
     * 
     * let mut entities = Entities::default();
     * // add in a dummy entity
     * entities.create_entity()
     *     .insert_checked(Component1(-5)).unwrap()
     *     .insert_checked(Component2('r')).unwrap();
     * 
     * let query_res = Query::new(&entities)
     *      .with_component::<Component1>().unwrap()
     *      .with_component::<Component2>().unwrap()
     *      .run();
     * 
     * let n1s = &query_res[0];
     * let n2s = &query_res[1];
     * 
     * assert_eq!(n1s.len(), n2s.len());
     * assert_eq!(n1s.len(), 1);
     * 
     * ```
     */
    pub fn with_component<T: Any>(&mut self) -> eyre::Result<&mut Self> {
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
     * Executes and returns the result of a query in the form of a vector of vectors 
     * of [ComponentType](types.ComponentType.html).
     * 
     * ```
     * use secs::prelude::*;
     * 
     * struct Component1(pub i8);
     * struct Component2(pub char);
     * 
     * let mut entities = Entities::default();
     * // add in a dummy entity
     * entities.create_entity()
     *     .insert_checked(Component1(-5)).unwrap()
     *     .insert_checked(Component2('r')).unwrap();
     * 
     * entities.create_entity()
     *     .insert_checked(Component1(120)).unwrap()
     *     .insert_checked(Component2('b')).unwrap();
     * 
     * let query_res = Query::new(&entities)
     *      .with_component::<Component1>().unwrap()
     *      .with_component::<Component2>().unwrap()
     *      .run();
     * 
     * let n1s = &query_res[0];
     * let n2s = &query_res[1];
     * 
     * let first1 = n1s[0].borrow();
     * let first1 = first1.downcast_ref::<Component1>().unwrap();
     * assert_eq!(first1.0, -5);
     * 
     * let first2 = n2s[0].borrow();
     * let first2 = first2.downcast_ref::<Component2>().unwrap();
     * assert_eq!(first2.0, 'r');
     * 
     * let second1 = n1s[1].borrow();
     * let second1 = second1.downcast_ref::<Component1>().unwrap();
     * assert_eq!(second1.0, 120);
     * 
     * let second2 = n2s[1].borrow();
     * let second2 = second2.downcast_ref::<Component2>().unwrap();
     * assert_eq!(second2.0, 'b');
     * ```
     */
    pub fn run(&mut self) -> Vec<Vec<ComponentType>> {
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

#[derive(thiserror::Error, Debug)]
enum QueryError {
    #[error("Attempted to query an unregistered component")]
    UnregisteredComponentError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_mask_updating() -> eyre::Result<()> {
        let ents = init_entities()?;

        let mut query = Query::new(&ents);
        query.with_component::<Component1>()?
            .with_component::<Component2>()?;

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
        query.with_component::<Component1>()?
            .with_component::<Component2>()?
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

    struct Component1(pub i8);

    struct Component2(pub char);
}