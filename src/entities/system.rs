use std::{
    any::{Any},
    marker::PhantomData, ops::DerefMut
};
use std::ops::Deref;

use crate::resources::Resources;

use super::{Entities, FnQuery, FnQueryContainedTupleType};

pub struct Res<'a, T> {
	resources: &'a Resources,
	phantom: PhantomData<&'a T>,
}

impl<'a, T> Res<'a, T> {
	pub fn new(resources: &'a Resources) -> Self {
		Self {
			resources, phantom: PhantomData
		}
	}
}

impl<'a, T> Deref for Res<'a, T> 
where T: Any
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
        if let Ok(reff) = self.resources.get_ref::<T>() {
        	reff
        } else {
        	panic!("Attempt to use NONEXISTANT RESOURCE, consider inserting it into the ECS :)");
        }
	}
}

pub struct ResMut<'a, T> {
	resources: &'a mut Resources,
	phantom: PhantomData<&'a T>,
}

impl<'a, T> ResMut<'a, T> {
	pub fn new(resources: &'a mut Resources) -> Self {
		Self {
			resources, phantom: PhantomData
		}
	}
}

impl<'a, T> Deref for ResMut<'a, T> 
where T: Any
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
        if let Ok(reff) = self.resources.get_ref::<T>() {
        	reff
        } else {
        	panic!("Attempt to use NONEXISTANT RESOURCE, consider inserting it into the ECS :)");
        }
	}
}

impl<'a, T> DerefMut for ResMut<'a, T> 
where T: Any
{
	fn deref_mut(&mut self) -> &mut Self::Target {
        if let Ok(reff) = self.resources.get_mut::<T>() {
        	reff
        } else {
        	panic!("Attempt to use NONEXISTANT RESOURCE, consider inserting it into the ECS :)");
        }
	}
}

trait ResParamType<'a> {
	fn get(resources: &'a mut Resources) -> Self where Self: Sized;
}

impl<'a, T> ResParamType<'a> for Res<'a, T>
where T: Any
{
	fn get(resources: &'a mut Resources) -> Self where Self: Sized {
	    Self::new(resources)
	}
}

impl<'a, T> ResParamType<'a> for ResMut<'a, T>
where T: Any
{
	fn get(resources: &'a mut Resources) -> Self where Self: Sized {
	    Self::new(resources)
	}
}

trait SystemParams<'a> {
	fn get(entities: &'a Entities, resources: &'a mut Resources) -> Self where Self: Sized;
}

impl<'a, T> SystemParams<'a> for FnQuery<'a, T>
where T: FnQueryContainedTupleType<'a>
{
	fn get(entities: &'a Entities, _resources: &'a mut Resources) -> Self {
	    Self::new(entities)
	}
}

impl<'a, T> SystemParams<'a> for Res<'a, T>
where T: Any
{
	fn get(_entities: &'a Entities, resources: &'a mut Resources) -> Self {
	    Self::new(resources)
	}
}

impl<'a, T> SystemParams<'a> for ResMut<'a, T>
where T: Any
{
	fn get(_entities: &'a Entities, resources: &'a mut Resources) -> Self {
	    Self::new(resources)
	}
}

pub trait IntoSystem<'a, Arguments> {
	fn run(self, entities: &'a Entities, resources: &'a mut Resources);
}

impl<'a, F, T> IntoSystem<'a, T> for F 
where 
	T: SystemParams<'a>,
	F: Fn(T)
{
	fn run(self, entities: &'a Entities, resources: &'a mut Resources) {
	    (self)(T::get(entities, resources))
	}
}

impl<'a, F, T1, T2> IntoSystem<'a, (T1, T2)> for F 
where 
	T1: SystemParams<'a>,
	T2: SystemParams<'a>,
	F: Fn(T1, T2)
{
	fn run(self, entities: &'a Entities, resources: &'a mut Resources) {
	    (self)(T1::get(entities, resources), T2::get(entities, resources))
	}
}