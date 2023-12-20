use std::cell::RefCell;
use std::collections::HashMap;
pub struct Cache {
    cached_values: RefCell<HashMap<u32, bool>>,
}
impl Cache {
    pub fn new() -> Self {
        Self {
            cached_values: RefCell::new(HashMap::new()),
        }
    }
    pub fn is_cached(&self, x: u32) -> Option<bool> {
        let map = &*self.cached_values.borrow();
        if let Some(p) = map.get(&x) {
            return Some(p.clone());
        } else {
            return None;
        }
    }
    pub fn insert(&self, x: u32, p: bool) -> Option<bool> {
        println!("Inserting result for {x} in cache");
        let map = &mut *self.cached_values.borrow_mut();
        map.insert(x, p)
    }
}
