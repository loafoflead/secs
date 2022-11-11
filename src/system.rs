use std::{
    any::Any,
    marker::PhantomData, cell::{Ref, RefMut}
};

use crate::resources::Resources;

use super::entities::{Entities, FnQuery, FnQueryContainedTupleType};

/**
A function parameter that denotes an immutable reference to a Resource. 
It's mutable equivalent is [ResMut].

Usage:
```
use sceller::prelude::*;

#[derive(Eq, PartialEq, Debug)]
struct ReeseOurse(usize);

let mut world = World::new();
world.insert_resource(ReeseOurse(55usize));

world.run_system(get_res);

fn get_res(res: Res<ReeseOurse>) {
   assert_eq!(*res.get(), ReeseOurse(55));
}

```
 */
pub struct Res<'a, T> {
	resources: &'a Resources,
	phantom: PhantomData<&'a T>,
}

impl<'a, T: Any> Res<'a, T> {
	pub fn new(resources: &'a Resources) -> Self {
		Self {
			resources, phantom: PhantomData
		}
	}

	/// Retrieve a Ref<T> to the content of the Resource
	pub fn get(&self) -> Ref<T> {
		self.resources.get_ref::<T>().unwrap()
	}
}

/**
A function parameter that denotes a mutable reference to a Resource. 
It's immutable equivalent is [ResMut].

Usage:
```
use sceller::prelude::*;

#[derive(Eq, PartialEq, Debug)]
struct ReeseOurse(usize);

let mut world = World::new();
world.insert_resource(ReeseOurse(55usize));

world.run_system(get_res);

fn get_res(res_mut: ResMut<ReeseOurse>) {
   assert_eq!(*res_mut.get(), ReeseOurse(55));

   {
	   res_mut.get().0 = 44;
   }

   assert_eq!(*res_mut.get(), ReeseOurse(44));
}

```
 */
pub struct ResMut<'a, T> {
	resources: &'a Resources,
	phantom: PhantomData<&'a T>,
}

impl<'a, T: Any> ResMut<'a, T> {
	pub fn new(resources: &'a Resources) -> Self {
		Self {
			resources, phantom: PhantomData
		}
	}

	/// Retrieve a RefMut<T> to the content of the Resource
	pub fn get(&self) -> RefMut<T> {
		self.resources.get_mut::<T>().unwrap()
	}
}


trait ResParamType<'a> {
	fn get(resources: &'a Resources) -> Self where Self: Sized;
}

impl<'a, T> ResParamType<'a> for Res<'a, T>
where T: Any
{
	fn get(resources: &'a Resources) -> Self where Self: Sized {
	    Self::new(resources)
	}
}

impl<'a, T> ResParamType<'a> for ResMut<'a, T>
where T: Any
{
	fn get(resources: &'a Resources) -> Self where Self: Sized {
	    Self::new(resources)
	}
}

trait SystemParams<'a> {
	fn get(entities: &'a Entities, resources: &'a Resources) -> Self where Self: Sized;
}

impl<'a, T> SystemParams<'a> for FnQuery<'a, T>
where T: FnQueryContainedTupleType<'a>
{
	fn get(entities: &'a Entities, _resources: &'a Resources) -> Self {
	    Self::new(entities)
	}
}

impl<'a, T> SystemParams<'a> for Res<'a, T>
where T: Any
{
	fn get(_entities: &'a Entities, resources: &'a Resources) -> Self {
	    Self::new(resources)
	}
}

impl<'a, T> SystemParams<'a> for ResMut<'a, T>
where T: Any
{
	fn get(_entities: &'a Entities, resources: &'a Resources) -> Self {
	    Self::new(resources)
	}
}

pub trait IntoSystem<'a, Arguments> {
	fn run(self, entities: &'a Entities, resources: &'a Resources);
}

impl<'a, F, T> IntoSystem<'a, T> for F 
where 
	T: SystemParams<'a>,
	F: Fn(T)
{
	fn run(self, entities: &'a Entities, resources: &'a Resources) {
	    (self)(T::get(entities, resources))
	}
}

impl<'a, F, T1, T2> IntoSystem<'a, (T1, T2)> for F 
where 
	T1: SystemParams<'a>,
	T2: SystemParams<'a>,
	F: Fn(T1, T2)
{
	fn run(self, entities: &'a Entities, resources: &'a Resources) {
	    (self)(T1::get(entities, resources), T2::get(entities, resources))
	}
}

impl<'a, F, T1, T2, T3> IntoSystem<'a, (T1, T2, T3)> for F 
where 
	T1: SystemParams<'a>,
	T2: SystemParams<'a>,
	T3: SystemParams<'a>,
	F: Fn(T1, T2, T3)
{
	fn run(self, entities: &'a Entities, resources: &'a Resources) {
	    (self)(
	    	T1::get(entities, resources), 
	    	T2::get(entities, resources),
	    	T3::get(entities, resources))
	}
}

impl<'a, F, T1, T2, T3, T4> IntoSystem<'a, (T1, T2, T3, T4)> for F 
where 
	T1: SystemParams<'a>,
	T2: SystemParams<'a>,
	T3: SystemParams<'a>,
	T4: SystemParams<'a>,
	F: Fn(T1, T2, T3, T4)
{
	fn run(self, entities: &'a Entities, resources: &'a Resources) {
	    (self)(
	    	T1::get(entities, resources), 
	    	T2::get(entities, resources),
	    	T3::get(entities, resources),
	    	T4::get(entities, resources),
	    	)
	}
}

impl<'a, F, T1, T2, T3, T4, T5> IntoSystem<'a, (T1, T2, T3, T4, T5)> for F 
where 
	T1: SystemParams<'a>,
	T2: SystemParams<'a>,
	T3: SystemParams<'a>,
	T4: SystemParams<'a>,
	T5: SystemParams<'a>,
	F: Fn(T1, T2, T3, T4, T5)
{
	fn run(self, entities: &'a Entities, resources: &'a Resources) {
	    (self)(
	    	T1::get(entities, resources), 
	    	T2::get(entities, resources),
	    	T3::get(entities, resources),
	    	T4::get(entities, resources),
	    	T5::get(entities, resources),
	    	)
	}
}