//! An opaque handler for a scene and its state.

use super::errors::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait BoxedScene<C> {
    fn tick(&mut self, core: Rc<RefCell<C>>) -> Result<()>;
}
