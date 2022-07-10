mod resources;
mod world;
mod entities;

pub mod prelude {
    pub use super::resources::*;
    pub use super::world::*;
    pub use super::entities::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
