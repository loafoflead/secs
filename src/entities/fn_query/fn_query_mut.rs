use std::{
    any::{Any, TypeId},
    cell::{RefMut, RefCell},
    marker::PhantomData, rc::Rc,
};

use super::{Entities, Query};

/**
The type of the function parameter of a mutable function query. See [FnQuery](struct.FnQuery.html)
for the immutable implementation of this type.

This struct permits the use of [Query::query_fn_mut], where a function is passed into a query to execute it.

# Examples

```
use sceller::prelude::*;

struct Health(u32);

fn change_healths(healths: FnQueryMut<Health>) {
    for mut hp in healths.into_iter() {
        hp.0 += 9;
    }
}

let mut world = World::new();

world.spawn().insert(Health(10));

let query = world.query();
query.query_fn_mut(&change_healths); // runs this function with the querys result as a parameter.
```

As of now the struct can handle up to three parameters in a query in the form of a tuple:

```
use sceller::prelude::*;

struct Health(u32);
struct Speed(u32);
struct Size(u32);

fn change_all(healths: FnQueryMut<(Health, Speed, Size)>) {
    for (mut hp, speed, mut size) in healths.iter() {
        hp.0 += 5;
        println!("{}", speed.0);
        size.0 -= 2;
    }
}

let mut world = World::new();

world.spawn().insert(Health(10)).insert(Speed(65)).insert(Size(15));

let query = world.query();
query.query_fn_mut(&change_all); // runs this function with the querys result as a parameter.
```
 */
pub struct FnQueryMut<'a, T> {
    entities: &'a Entities,
    phantom: PhantomData<T>,
}

impl<'a, T> FnQueryMut<'a, T> {
    fn new(entities: &'a Entities) -> FnQueryMut<'a, T> {
        FnQueryMut {
            entities: entities,
            phantom: PhantomData,
        }
    }
}

impl<'a> Query<'a> {
    /**
    Mutable implementation of [Query::query_fn]
     */
    pub fn query_fn_mut<F, T: 'a>(&self, gen: F)
    where
        F: Fn(FnQueryMut<'a, T>),
    {
        gen(FnQueryMut::new(&self.entities))
    }
}

impl<'a, T: 'static> std::iter::IntoIterator for FnQueryMut<'a, T>
where T: Any,
{
    type IntoIter = FnQueryIntoIterator<'a, RefMut<'a, T>>;
    type Item = RefMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let typeid = TypeId::of::<T>();

        let selfmap = self.entities.bit_masks.get(&typeid).unwrap();

        let all_components = self.entities.components.get(&typeid).unwrap();
        // get all components with the type of this AutoQuery

        // get all valid components (not deleted or None)
        let components = all_components.into_iter().enumerate()
            .map(|(ind, c)| {
                if (self.entities.map[ind] & selfmap == *selfmap) && c.is_some() {
                    Some(c.as_ref().unwrap())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<&Rc<RefCell<dyn Any>>>>();

        FnQueryIntoIterator {
            components: components.into_iter()
                .map(|c| {
                    let component = c.as_ref();
                    let borrow_mut = component.borrow_mut();

                    RefMut::map(borrow_mut, |any| {
                        any.downcast_mut::<T>().unwrap()
                    })
                })
                .collect::<Vec<RefMut<T>>>(),
            phantom: PhantomData
        }
    }
}



impl<'a, T: 'a, T2: 'a> FnQueryMut<'a, (T, T2)>
where
    T: Any,
    T2: Any,
{
    pub fn iter(self) -> FnQueryIntoIterator<'a, (RefMut<'a, T>, RefMut<'a, T2>)> {
        let typeid1 = TypeId::of::<T>();
        let typeid2 = TypeId::of::<T2>();

        // let selftype_ids = vec![typeid1, typeid2];

        let mut selfmap = self.entities.get_bitmask(&typeid1).unwrap();
        selfmap |= self.entities.get_bitmask(&typeid2).unwrap();

        let indexes = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, map)| {
                if map & selfmap == selfmap {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let mut return_vec = Vec::new();

        // Make this ^^^^ into the return type

        let components = self.entities.components.get(&typeid1).unwrap();
        let mut query_components = Vec::new();
        for index in &indexes {
            query_components.push(components[*index].as_ref());
        }
        let query_components1 = query_components.into_iter().flatten().collect::<Vec<_>>();

        let components = self.entities.components.get(&typeid2).unwrap();
        let mut query_components = Vec::new();
        for index in &indexes {
            query_components.push(components[*index].as_ref());
        }
        let query_components2 = query_components.into_iter().flatten().collect::<Vec<_>>();

        for i in 0..query_components1.len() {
            return_vec.push((
                RefMut::map(query_components1[i].as_ref().borrow_mut(), |any| {
                    any.downcast_mut::<T>().unwrap()
                }),
                RefMut::map(query_components2[i].as_ref().borrow_mut(), |any| {
                    any.downcast_mut::<T2>().unwrap()
                }),
            ));
        }

        FnQueryIntoIterator {
            components: return_vec,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, T2: 'a, T3: 'a> FnQueryMut<'a, (T, T2, T3)>
where
    T: Any,
    T2: Any,
    T3: Any,
{
    pub fn iter(self) -> FnQueryIntoIterator<'a, (RefMut<'a, T>, RefMut<'a, T2>, RefMut<'a, T3>)> {
        let typeid1 = TypeId::of::<T>();
        let typeid2 = TypeId::of::<T2>();
        let typeid3 = TypeId::of::<T3>();

        // let selftype_ids = vec![typeid1, typeid2];

        let mut selfmap = self.entities.get_bitmask(&typeid1).unwrap();
        selfmap |= self.entities.get_bitmask(&typeid2).unwrap();
        selfmap |= self.entities.get_bitmask(&typeid3).unwrap();

        let indexes = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, map)| {
                if map & selfmap == selfmap {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let mut return_vec = Vec::new();

        // Make this ^^^^ into the return type

        let components = self.entities.components.get(&typeid1).unwrap();
        let mut query_components = Vec::new();
        for index in &indexes {
            query_components.push(components[*index].as_ref());
        }
        let query_components1 = query_components.into_iter().flatten().collect::<Vec<_>>();

        let components = self.entities.components.get(&typeid2).unwrap();
        let mut query_components = Vec::new();
        for index in &indexes {
            query_components.push(components[*index].as_ref());
        }
        let query_components2 = query_components.into_iter().flatten().collect::<Vec<_>>();

        let components = self.entities.components.get(&typeid3).unwrap();
        let mut query_components = Vec::new();
        for index in &indexes {
            query_components.push(components[*index].as_ref());
        }
        let query_components3 = query_components.into_iter().flatten().collect::<Vec<_>>();

        for i in 0..query_components1.len() {
            return_vec.push((
                RefMut::map(query_components1[i].as_ref().borrow_mut(), |any| {
                    any.downcast_mut::<T>().unwrap()
                }),
                RefMut::map(query_components2[i].as_ref().borrow_mut(), |any| {
                    any.downcast_mut::<T2>().unwrap()
                }),
                RefMut::map(query_components3[i].as_ref().borrow_mut(), |any| {
                    any.downcast_mut::<T3>().unwrap()
                }),
            ));
        }

        FnQueryIntoIterator {
            components: return_vec,
            phantom: PhantomData,
        }
    }
}

pub struct FnQueryIntoIterator<'a, T> {
    components: Vec<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> std::iter::Iterator for FnQueryIntoIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.components.pop()
    }
}
