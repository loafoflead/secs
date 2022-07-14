use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    marker::PhantomData, rc::Rc,
};

use super::{Entities, Query};

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

impl<'a> Query<'a> {
    pub fn query_fn<F, T: 'a>(&self, gen: F)
    where
        F: Fn(FnQuery<'a, T>),
    {
        gen(FnQuery::new(&self.entities))
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



impl<'a, T: 'a, T2: 'a> FnQuery<'a, (T, T2)>
where
    T: Any,
    T2: Any,
{
    pub fn iter(self) -> FnQueryIntoIterator<'a, (Ref<'a, T>, Ref<'a, T2>)> {
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
    pub fn iter(self) -> FnQueryIntoIterator<'a, (Ref<'a, T>, Ref<'a, T2>, Ref<'a, T3>)> {
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

// fn get_vec_of_relevent_types<'a, T: 'static>(entities: &'a Entities) -> Vec<Ref<T>> {
//     let typeid = TypeId::of::<T>();

//         let selfmap = entities.bit_masks.get(&typeid).unwrap();

//         let all_components = entities.components.get(&typeid).unwrap();
//         // get all components with the type of this AutoQuery

//         // get all valid components (not deleted or None)
//         let components = all_components.into_iter().enumerate()
//             .map(|(ind, c)| {
//                 if (entities.map[ind] & selfmap == *selfmap) && c.is_some() {
//                     Some(c.as_ref().unwrap())
//                 } else {
//                     None
//                 }
//             })
//             .flatten()
//             .collect::<Vec<&Rc<RefCell<dyn Any>>>>();

//         components.into_iter()
//             .map(|c| {
//                 let component = c.as_ref();
//                 let borrow = component.borrow();

//                 Ref::map(borrow, |any| {
//                     any.downcast_ref::<T>().unwrap()
//                 })
//             })
//             .collect::<Vec<Ref<T>>>()
// }

// use super::{Entities, Query};

// // Any function under the form:
// //
// // fn(impl FnQueryParams) -> ()
// //
// // should be able to call the function
// pub trait IntoQueryFunction<'a, P, A> {
//     type QueryType: FnQueryParams<'a, P> + Sized;

//     fn get_query(&self) -> Self::QueryType;
// }

// // A is anything
// // this covers any function with a FnQeuryParams as parameter
// impl<'a, F, P, A> IntoQueryFunction<'a, P, A> for F
// where F: Fn(dyn FnQueryParams<'a, P, Output = Vec<Ref<'a, A>>>),
//       A: 'a,
// {
//     type QueryType = dyn FnQueryParams<'a, P, Output = Vec<Ref<'a, A>>>;

//     fn get_query(&self) -> Self::QueryType { }
// }

// pub trait FnQueryParams<'a, P: Sized> {
//     type Output: Sized;

//     fn run(&self, entities: &'a Entities) -> Self::Output;
// }

// pub struct FnQuery<P> {
//     phantom: PhantomData<P>
// }

// A query with only one type
// impl<'a, P: 'static> Params<'a> for AutoQuery<'a, P>
// where P: Any
// {
//     type Return = AutoQueryIntoIterator<'a, P>;

//     fn run(entities: &'a Entities) -> Self::Return {
//         let this = AutoQuery::new(entities);
//         this.into_iter()

//         // let typeid = TypeId::of::<P>();

//         // let selfmap = entities.bit_masks.get(&typeid).unwrap();

//         // let all_components = entities.components.get(&typeid).unwrap();
//         // // get all components with the type of this AutoQuery

//         // // get all valid components (not deleted or None)
//         // let components = all_components
//         //     .into_iter()
//         //     .enumerate()
//         //     .map(|(ind, c)| {
//         //         if (entities.map[ind] & selfmap == *selfmap) && c.is_some() {
//         //             Some(c.as_ref().unwrap())
//         //         } else {
//         //             None
//         //         }
//         //     })
//         //     .flatten()
//         //     .collect::<Vec<&Rc<RefCell<dyn Any>>>>();

//         // components
//         //     .into_iter()
//         //     .map(|c| {
//         //         let component = c.as_ref();
//         //         let borrow = component.borrow();

//         //         Ref::map(borrow, |any| any.downcast_ref::<P>().unwrap())
//         //     })
//         //     .collect::<Vec<Ref<P>>>()
//     }
// }

// impl<'a> Query<'a> {
//     pub fn query_fn<Q: FnQueryParams<'a, P> + 'static, P, F: IntoQueryFunction<'a, Q, P>>(&self, function: F) {

//         let query = function.get_query().run(&self.entities);

//         function(query);
//     }
// }
