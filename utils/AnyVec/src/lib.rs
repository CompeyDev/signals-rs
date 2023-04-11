#![feature(box_into_inner)]

use std::any::Any;
use std::vec::Vec;
type Anything = Box<dyn Any>;

#[derive(Debug)] 
pub struct AnyVec(Vec<Anything>);
 
impl AnyVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }
 
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn insert<V: Any>(&mut self, key: usize, val: V) {
        self
            .0
            .insert(key, Box::new(val));
    }
 
    pub fn get<V: Any>(&self, idx: usize) -> Option<&V> {
        self.0.get(idx)?.downcast_ref::<V>()
    }
 
    pub fn get_mut<V: Any>(&mut self, idx: usize) -> Option<&mut V> {
        self.0
            .get_mut(idx)?
            .downcast_mut::<V>()
    }
 
    pub fn remove<V: Any>(&mut self, idx: usize) -> Option<V> {
        let boxed = self
            .0
            .remove(idx)
            .downcast::<V>()
            .ok()?;
 
        Some(Box::into_inner(boxed))
    }
}