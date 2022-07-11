//! SECS or Snug Entity Component System (no other reason for it to be called that) is an ECS crate.
//! 
//! ## SOURCES
//! It is modelled from the tutorial series by [Brooks Builds](https://www.youtube.com/channel/UCT1-XRVnJA-wws2bfbLbFcQ) on Youtube and his series on [how to make an ECS in rust](https://www.youtube.com/watch?v=CTuTEi4YUb8&list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs).
//! 
//! This project is likely riddled with bugs, and yet I am very proud of it.
//! 
//! ## HOW TO START
//! For tips and pointers on how to have SECS, check the examples folder, or ask a knowledgeable friend. (wow that's an ugly word)
//! Most importantly, have fun!
//! The '[prelude]' module contains the only modules you will probably ever need.
//! 
//! If you have any bugs to report or questions, leave 'em on the github friends. If you want me to teach you rust, ask someone who knows rust (maybe that friend we talked about ealier).
//! If you have a terribly negative review or want to chastise my design choices, leave that on the github too. I recommend you __don't__ check out my other work, because it just isn't very good. 
//! Ok bye.
//! 
//! Oh, and i forgot to mention something really important about this crate, don't ever ever **ever** forget t
//! 

pub mod resources;
pub mod world;
pub mod entities;

pub mod prelude {
    pub use super::resources::*;
    pub use super::world::*;
    pub use super::entities::*;

    pub use std::cell::{Ref, RefMut};

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
