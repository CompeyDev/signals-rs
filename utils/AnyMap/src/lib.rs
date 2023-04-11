#![feature(box_into_inner)]

use std::any::{Any, TypeId};
use std::cmp::Eq;
use hashbrown::HashMap;
use std::hash::Hash;
 
type HashKey<K> = (K, TypeId);
type Anything = Box<dyn Any>;
 
pub struct AnyMap<K: Eq + Hash>(HashMap<HashKey<K>, Anything>);
 
impl<K: Eq + Hash> AnyMap<K> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
 
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn insert<V: Any>(&mut self, key: K, val: V) -> Option<V> {
        let boxed = self
            .0
            .insert((key, val.type_id()), Box::new(val))?
            .downcast::<V>()
            .ok()?;
 
        Some(Box::into_inner(boxed))
    }
 
    pub fn get<V: Any>(&self, key: K) -> Option<&V> {
        self.0.get(&(key, TypeId::of::<V>()))?.downcast_ref::<V>()
    }
 
    pub fn get_mut<V: Any>(&mut self, key: K) -> Option<&mut V> {
        self.0
            .get_mut(&(key, TypeId::of::<V>()))?
            .downcast_mut::<V>()
    }
 
    pub fn remove<V: Any>(&mut self, key: K) -> Option<V> {
        let boxed = self
            .0
            .remove(&(key, TypeId::of::<V>()))?
            .downcast::<V>()
            .ok()?;
 
        Some(Box::into_inner(boxed))
    }
}