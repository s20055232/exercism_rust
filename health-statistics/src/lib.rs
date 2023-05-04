// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User{
            name,
            age,
            weight
        }
        // unimplemented!()
    }

    pub fn name(&self) -> &str {
        &self.name
        // unimplemented!()
    }

    pub fn age(&self) -> u32 {
        self.age
        // unimplemented!()
    }

    pub fn weight(&self) -> f32 {
        self.weight
        // unimplemented!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
        // unimplemented!()
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
        // unimplemented!()
    }
}
