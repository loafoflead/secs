mod resources;
mod world;
mod entities;

pub use resources::*;
pub use world::*;
pub use entities::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
