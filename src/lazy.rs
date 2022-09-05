// Allow closures that capture variables
// //https://github.com/matklad/once_cell/issues/156

use std::ops::{Deref, DerefMut};
type OnceCellLazy<T> = once_cell::unsync::Lazy<T, Box<dyn FnOnce() -> T>>;

#[derive(Debug)]
pub struct Lazy<T>(OnceCellLazy<T>);

impl<T> Lazy<T> {
    pub fn new<F: FnOnce() -> T + 'static>(init: F) -> Self {
        Self(once_cell::unsync::Lazy::new(Box::new(init)))
    }
}

impl<T> Deref for Lazy<T> {
    type Target = OnceCellLazy<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Lazy<T> {
    fn deref_mut(&mut self) -> &mut OnceCellLazy<T> {
        &mut self.0
    }
}
