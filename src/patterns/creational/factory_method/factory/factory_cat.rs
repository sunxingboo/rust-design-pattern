use crate::patterns::creational::factory_method::product::{base::Animal, product_cat::Cat};
use super::base::Factory;

// Cat工厂
pub(crate) struct CatFactory {}

impl Factory for CatFactory {
    fn new(name: String) -> Box<dyn Animal> {
        Box::new(Cat::new(name))
    }
}