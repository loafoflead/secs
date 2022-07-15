mod fn_query_mut;
pub use fn_query_mut::*;

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    marker::PhantomData, rc::Rc,
};

use super::{Entities, Query};

trait FnQueryParams<'a>
{  
    fn get(entities: &'a Entities) -> Self where Self: Sized;
}

impl<'a, T> FnQueryParams<'a> for FnQuery<'a, T> {
    fn get(entities: &'a Entities) -> Self {
        Self::new(entities) // because this is the generic constructor for FnQuery
    }
}

impl<'a, F, T> IntoQueryFunction<'a, T> for F
where 
    T: FnQueryParams<'a>,
    F: Fn(T),
{
    fn run(self, entities: &'a Entities) {
        (self)(FnQueryParams::get(entities))
    }
}
impl<'a, Func, T1, T2> IntoQueryFunction<'a, (T1, T2)> for Func
where
    Func: Fn(T1, T2),
    T1: FnQueryParams<'a>, T2: FnQueryParams<'a>,
{
    fn run(self, entities: &'a Entities) {
        (self)(FnQueryParams::get(entities), FnQueryParams::get(entities))
    }
}
impl<'a, Func, T1, T2, T3> IntoQueryFunction<'a, (T1, T2, T3)> for Func
where
    Func: Fn(T1, T2, T3),
    T1: FnQueryParams<'a>, T2: FnQueryParams<'a>,
    T3: FnQueryParams<'a>,
{
    fn run(self, entities: &'a Entities) {
        (self)(
            FnQueryParams::get(entities), 
            FnQueryParams::get(entities),
            FnQueryParams::get(entities)
        )
    }
}


pub trait IntoQueryFunction<'a, ArgList> {
    fn run(self, entities: &'a Entities);
}

// impl<'a, 'b, 'c, Func, T, T2> IntoQueryFunction<(FnQuery<'a, T>, FnQuery<'b, T2>)> for Func
// where
//     Func: for<'r, 's> Fn(FnQuery<'r, T>, FnQuery<'s, T2>),
// {
//     fn run(self, entities: &Entities) {
//         (self)(FnQuery::new(entities), FnQuery::new(entities))
//     }
// }

// impl<Func, T, T2, T3> IntoQueryFunction<(FnQuery<'_, T>, FnQuery<'_, T2>, FnQuery<'_, T3>)> for Func
// where
//     Func: for<'r, 's, 'e> Fn(FnQuery<'r, T>, FnQuery<'s, T2>, FnQuery<'e, T3>),
// {
//     fn run(self, entities: &Entities) {
//         (self)(FnQuery::new(entities), FnQuery::new(entities), FnQuery::new(entities))
//     }
// }

// impl<'a, F, T> IntoQueryFunction<FnQuery<'a, T>> for F
// where
//     F: Fn(FnQuery<'_, T>),
//     T: Any,
//     // T1: Any, T2: Any
// {
//     fn run(self, entities: &Entities) {
//         self(FnQuery::new(entities))
//     }
// }



/**
The type of the function parameter of an immutable function query. See [FnQueryMut](struct.FnQueryMut.html)
for the mutable implementation of this type.

This struct permits the use of [Query::query_fn], where a function is passed into a query to execute it.

# Examples

```
use sceller::prelude::*;

struct Health(u32);

fn print_healths(healths: FnQuery<Health>) {
    for hp in healths.into_iter() {
        println!("{}", hp.0);
    }
}

let mut world = World::new();

world.spawn().insert(Health(10));

let query = world.query();
query.query_fn(&print_healths); // runs this function with the querys result as a parameter.
```

As of now the struct can handle up to three parameters in a query in the form of a tuple:

```
use sceller::prelude::*;

struct Health(u32);
struct Speed(u32);
struct Size(u32);

fn print_all(healths: FnQuery<(Health, Speed, Size)>) {
    for (hp, speed, size) in healths.iter() {
        println!("{}, {}, {}", hp.0, speed.0, size.0);
    }
}

let mut world = World::new();

world.spawn().insert(Health(10)).insert(Speed(65)).insert(Size(15));

let query = world.query();
query.query_fn(&print_all); // runs this function with the querys result as a parameter.
```
 */
pub struct FnQuery<'a, T> {
    entities: &'a Entities,
    phantom: PhantomData<T>,
}

impl<'a, T> FnQuery<'a, T> {
    fn new(entities: &'a Entities) -> FnQuery<'a, T> {
        FnQuery {
            entities: entities,
            phantom: PhantomData,
        }
    }
}

// Implementation of actual functions
impl<'a> Query<'a> {
    /**
    Takes in a function to run with the query's result passed as parameters.

    # Examples

    ```
    use sceller::prelude::*;

    struct Health(u32);

    fn print_healths(healths: FnQuery<Health>) {
        for hp in healths.into_iter() {
            println!("{}", hp.0);
        }
    }

    let mut world = World::new();

    world.spawn().insert(Health(10));

    let query = world.query();
    query.query_fn(&print_healths); // runs this function with the querys result as a parameter.
    ```

    As of now the struct can handle up to three parameters in a query in the form of a tuple:

    ```
    use sceller::prelude::*;

    struct Health(u32);
    struct Speed(u32);
    struct Size(u32);

    fn print_all(healths: FnQuery<(Health, Speed, Size)>) {
        for (hp, speed, size) in healths.iter() {
            println!("{}, {}, {}", hp.0, speed.0, size.0);
        }
    }

    let mut world = World::new();

    world.spawn().insert(Health(10)).insert(Speed(65)).insert(Size(15));

    let query = world.query();
    query.query_fn(&print_all); // runs this function with the querys result as a parameter.
    ```
     */
    pub fn query_fn<F, T: 'a>(&self, gen: F)
    where
        F: IntoQueryFunction<'a, T>
    {
        gen.run(self.entities)
    }
}

impl<'a, T: 'static> std::iter::IntoIterator for FnQuery<'a, T>
where T: Any,
{
    type IntoIter = FnQueryIntoIterator<'a, Ref<'a, T>>;
    type Item = Ref<'a, T>;

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
                    let borrow = component.borrow();

                    Ref::map(borrow, |any| {
                        any.downcast_ref::<T>().unwrap()
                    })
                })
                .collect::<Vec<Ref<T>>>(),
            phantom: PhantomData
        }
    }
}

impl<'a, T: 'static> std::iter::IntoIterator for &'a FnQuery<'a, T>
where T: Any,
{
    type IntoIter = FnQueryIntoIterator<'a, Ref<'a, T>>;
    type Item = Ref<'a, T>;

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
                    let borrow = component.borrow();

                    Ref::map(borrow, |any| {
                        any.downcast_ref::<T>().unwrap()
                    })
                })
                .collect::<Vec<Ref<T>>>(),
            phantom: PhantomData
        }
    }
}



impl<'a, T: 'a, T2: 'a> FnQuery<'a, (T, T2)>
where
    T: Any,
    T2: Any,
{
    pub fn iter(&self) -> FnQueryIntoIterator<'a, (Ref<'a, T>, Ref<'a, T2>)> {
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
                Ref::map(query_components1[i].as_ref().borrow(), |any| {
                    any.downcast_ref::<T>().unwrap()
                }),
                Ref::map(query_components2[i].as_ref().borrow(), |any| {
                    any.downcast_ref::<T2>().unwrap()
                }),
            ));
        }

        FnQueryIntoIterator {
            components: return_vec,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, T2: 'a, T3: 'a> FnQuery<'a, (T, T2, T3)>
where
    T: Any,
    T2: Any,
    T3: Any,
{
    pub fn iter(&self) -> FnQueryIntoIterator<'a, (Ref<'a, T>, Ref<'a, T2>, Ref<'a, T3>)> {
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
                Ref::map(query_components1[i].as_ref().borrow(), |any| {
                    any.downcast_ref::<T>().unwrap()
                }),
                Ref::map(query_components2[i].as_ref().borrow(), |any| {
                    any.downcast_ref::<T2>().unwrap()
                }),
                Ref::map(query_components3[i].as_ref().borrow(), |any| {
                    any.downcast_ref::<T3>().unwrap()
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
