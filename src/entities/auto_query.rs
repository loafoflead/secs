use std::{marker::PhantomData, cell::{Ref, RefMut, RefCell}, any::{TypeId, Any}, rc::Rc};

use super::{Entities};

/**
    AutoQuery is a struct that allows quick access of every instance of a single component immutably.
    (The mutable variant is [AutoQueryMut](struct.AutoQueryMut.html))

    It contains 'phantom' which is a PhantomData<T>, since the query needs to contain a type 
    for ease of use. And a reference to 'Entities'. 

    Pretty much all of this struct's functionality is implmenting IntoIterator, in which 
    the reference to Entities is used to get all components of the AutoQuery's type 'T'.
 */
pub struct AutoQuery<'a, T: Any> 
{
    entities: &'a Entities,
    phantom: PhantomData<T>,
}

impl<'a, T: 'static> AutoQuery<'a, T> {
    /// Constructs an AutoQuery
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            phantom: PhantomData
        }
    }

    /// Returns the number of items of this type in the ECS.
    pub fn len(&self) -> usize {
        let typeid = TypeId::of::<T>();
        // let components = self.entities.components.get(&typeid).unwrap();
        
        let selfmap = self.entities.bit_masks.get(&typeid).unwrap();

        self.entities.map.iter().fold(0, |aggr, bitmask| {
            if bitmask & selfmap == *selfmap {
                aggr + 1
            } else {
                aggr
            }
        })

        // components.iter().fold(0, |aggregate, comp| if comp.is_some() { aggregate + 1 } else { aggregate })
    }
}

impl<'a, T: 'static> std::iter::IntoIterator for AutoQuery<'a, T> {
    type IntoIter = AutoQueryIntoIterator<'a, T>;
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

        AutoQueryIntoIterator {
            components: components.into_iter()
                .map(|c| {
                    let component = c.as_ref();
                    let borrow = component.borrow();

                    Ref::map(borrow, |any| {
                        any.downcast_ref::<T>().unwrap()
                    })
                })
                .collect::<Vec<Ref<T>>>()
        }
    }
}

pub struct AutoQueryIntoIterator<'a, T> {
    components: Vec<Ref<'a, T>>,
}

impl<'a, T: 'static> std::iter::Iterator for AutoQueryIntoIterator<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.components.pop()
    }
}

/**
AutoQueryMut is a struct that allows quick access of every instance of a single component mutably.
(The immutable variant is [AutoQuery](struct.AutoQuery.html))

# WARNING

If you ever get an error: "thread 'main' panicked at 'already borrowed: BorrowMutError'"
this means you have two mutable borrows to the same data. This could be because you have two mutable
auto queries in the same scope.
The solution is to either drop them manually or to enclose them in a block:

```
use sceller::prelude::*;

struct Health; // example struct

{
    let ents = Entities::default();

    let query = Query::new(&ents);
    let mut auto = query.auto::<Health>();
    
    // <snip!>
} //<- ensures that the mutable borrow is dropped at the end of this block
```

It contains 'phantom' which is a PhantomData<T>, since the query needs to contain a type 
for ease of use. And a reference to 'Entities'. 

Pretty much all of this struct's functionality is implmenting IntoIterator, in which 
the reference to Entities is used to get all components of the AutoQueryMut's type 'T'.
 */
pub struct AutoQueryMut<'a, T: Any> 
{
    entities: &'a Entities,
    phantom: PhantomData<T>,
}

impl<'a, T: 'static> AutoQueryMut<'a, T> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            phantom: PhantomData
        }
    }
}

impl<'a, T: 'static> std::iter::IntoIterator for AutoQueryMut<'a, T> {
    type IntoIter = AutoQueryMutIntoIterator<'a, T>;
    type Item = RefMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let typeid = TypeId::of::<T>();
        let components = self.entities.components.get(&typeid).unwrap();
        // get all components with the type of this AutoQuery

        AutoQueryMutIntoIterator {
            components: components.into_iter()
                .flatten()
                .map(|c| {
                    let component = c.as_ref();
                    let borrow = component.borrow_mut();

                    RefMut::map(borrow, |any| {
                        any.downcast_mut::<T>().unwrap()
                    })
                })
                .collect::<Vec<RefMut<T>>>()
        }
    }
}


pub struct AutoQueryMutIntoIterator<'a, T> {
    components: Vec<RefMut<'a, T>>,
}

impl<'a, T: 'static> std::iter::Iterator for AutoQueryMutIntoIterator<'a, T> {
    type Item = RefMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.components.pop()
    }
}