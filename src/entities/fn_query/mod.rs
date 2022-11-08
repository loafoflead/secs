use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData, rc::Rc
};

use super::{Entities, Query};

impl<'a> Query<'a> {
    pub fn query_fn<F, T: 'a>(&self, gen: F)
    where
        F: IntoFnQuery<'a, T>
    {
        gen.run(self.entities)
    }
}

//
// A Query type used inside of Query Functions
//
// e.g: fn query_healths(healths: FnQuery<&Health>) { ... }
//
pub struct FnQuery<'a, T> {
    entities: &'a Entities,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> FnQuery<'a, T> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities, phantom: PhantomData
        }
    }
}

// A trait implemented for any functions that can be run as queries
pub trait IntoFnQuery<'a, Arguments> {
    fn run(self, entities: &'a Entities);
}

// a trait that abstracts over all FnQuery types in query parameters or singular values,
// e.g: FnQuery<(&Health, &Damage)>, FnQuery<&Health>
// so that they can all be stored as one type
pub trait QueryParameterType<'a> {
    fn get(entities: &'a Entities) -> Self where Self: Sized;
}

/* 
    FnQuery<(Anything...)> is now abstracted by this type!!! 
    this means we can get an FnQuery<T> from the functions parameter
*/
impl<'a, T> QueryParameterType<'a> for FnQuery<'a, T> 
where T: FnQueryContainedTupleType<'a>
{
    // in any query function we can now say FnQuery::get(entities)
    fn get(entities: &'a Entities) -> Self where Self: Sized {
        Self::new(entities)
    }
}

// trait that abstracts over whether the type contained in an FnQuery<T> 
// is a tuple and of what size
pub trait FnQueryContainedTupleType<'a> {
    type ReturnType;

    fn map(entities: &'a Entities) -> Vec<Self::ReturnType>;
}

/*
    Implements containedTupleType for any given type that is an individual type so
    that we can use this abstraction over everything
*/  
impl<'a, T> FnQueryContainedTupleType<'a> for T
where T: FnQueryContainedIndividualType<'a>
{
    type ReturnType = T::ReturnType;

    fn map(entities: &'a Entities) -> Vec<Self::ReturnType> {
        T::map(entities)
    }
}

impl<'a, T1, T2> FnQueryContainedTupleType<'a> for (T1, T2)
where 
    T1: FnQueryContainedIndividualType<'a>,
    T2: FnQueryContainedIndividualType<'a>,
{
    type ReturnType = (T1::ReturnType, T2::ReturnType);

    fn map(entities: &'a Entities) -> Vec<Self::ReturnType> {
        T1::map(entities).into_iter().zip(T2::map(entities)).collect()
    }
}

impl<'a, T1, T2, T3> FnQueryContainedTupleType<'a> for (T1, T2, T3)
where 
    T1: FnQueryContainedIndividualType<'a>,
    T2: FnQueryContainedIndividualType<'a>,
    T3: FnQueryContainedIndividualType<'a>,
{
    type ReturnType = (T1::ReturnType, T2::ReturnType, T3::ReturnType);

    fn map(entities: &'a Entities) -> Vec<Self::ReturnType> {
        T1::map(entities).into_iter()
            .zip(T2::map(entities))
            .zip(T3::map(entities))
            .map(|((x, y), z)| (x, y, z))
            .collect()
    }
}

// A trait implemented that abstracts over all the different types 
// an FnQuery<> can contain:
//
// e.g: fn query(hps: FnQuery<&Health>/<&mut Health>)
pub trait FnQueryContainedIndividualType<'a> 
{
    type ReturnType;

    fn type_id_new() -> TypeId;

    fn map(entities: &'a Entities) -> Vec<Self::ReturnType> {
        let typeid = Self::type_id_new();

        let selfmap = entities.bit_masks.get(&typeid).unwrap();

        let all_components = entities.components.get(&typeid).unwrap();
        // get all components with the type of this AutoQuery

        // get all valid components (not deleted or None)
        let components = all_components.into_iter().enumerate()
            .map(|(ind, c)| {
                if (entities.map[ind] & selfmap == *selfmap) && c.is_some() {
                    Some(c.as_ref().unwrap())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<&Rc<RefCell<dyn Any>>>>();

        components.into_iter().map(|component| {
            Self::map_ref(&component.as_ref())
        }).collect()
    }

    fn map_ref(reference: &'a RefCell<dyn Any>) -> Self::ReturnType;
}

impl<'a, T: Any> FnQueryContainedIndividualType<'a> for &T 
{
    type ReturnType = Ref<'a, T>;

    fn type_id_new() -> TypeId {
        TypeId::of::<T>()
    }

    fn map_ref(reference: &'a RefCell<dyn Any>) -> Self::ReturnType {
        Ref::map(reference.borrow(), |any| {
            any.downcast_ref::<T>().unwrap()
        })
    }
}

impl<'a, T: Any> FnQueryContainedIndividualType<'a> for &mut T 
{
    type ReturnType = RefMut<'a, T>;

    fn type_id_new() -> TypeId {
        TypeId::of::<T>()
    }

    fn map_ref(reference: &'a RefCell<dyn Any>) -> Self::ReturnType {
        RefMut::map(reference.borrow_mut(), |any| {
            any.downcast_mut::<T>().unwrap()
        })
    }
}

impl<'a, T, F> IntoFnQuery<'a, T> for F
where 
    T: QueryParameterType<'a>,
    F: Fn(T),
{
    fn run(self, entities: &'a Entities) {
        (self)(QueryParameterType::get(entities))
    }
}

impl<'a, T> FnQuery<'a, T> 
where T: FnQueryContainedTupleType<'a>
{
    pub fn iter(&self) -> FnQueryIterator<'a, T::ReturnType> {
        FnQueryIterator {
            components: T::map(self.entities),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> std::iter::IntoIterator for FnQuery<'a, T> 
where T: FnQueryContainedTupleType<'a>
{
    type Item = T::ReturnType;
    type IntoIter = FnQueryIterator<'a, T::ReturnType>;

    fn into_iter(self) -> Self::IntoIter {
        FnQueryIterator {
            components: T::map(self.entities),
            phantom: PhantomData
        }
    }
}

pub struct FnQueryIterator<'a, T> {
    components: Vec<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> std::iter::Iterator for FnQueryIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.components.pop()
    }
}