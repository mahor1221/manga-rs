// Allow closures to capture variables
//https://github.com/matklad/once_cell/issues/156

// This is an anti-pattern but There isn't any other way
// https://rust-unofficial.github.io/patterns/anti_patterns/deref.html

use std::fmt;
use std::ops::{Deref, DerefMut};

type Lazy<T> = once_cell::unsync::Lazy<T, Box<dyn FnOnce() -> T + 'static>>;
pub struct LazyBoxedInit<T>(Lazy<T>);

impl<T> LazyBoxedInit<T> {
    pub fn new<F: FnOnce() -> T + 'static>(init: F) -> Self {
        Self(Lazy::new(Box::new(init)))
    }
}

impl<T> Deref for LazyBoxedInit<T> {
    type Target = Lazy<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for LazyBoxedInit<T> {
    fn deref_mut(&mut self) -> &mut Lazy<T> {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for LazyBoxedInit<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
