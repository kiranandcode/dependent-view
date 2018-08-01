#![allow(dead_code)]

use super::push_ref;

use std::rc::Rc;
use std::convert::{AsMut,AsRef};
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};


#[macro_export]
macro_rules! to_view {
    ($dep:tt) => {
        ($dep.into_view_internal::<_, _,_, _, _>(|item| item.clone() as ::std::rc::Rc<_>, |item| ::std::rc::Rc::downgrade(item), |item| unsafe { ::std::mem::transmute(item) }));
    }
}



pub struct DependentRc<T> {
    item: Rc<T>,
    dependants: Vec<Rc<Any>>
}



impl<T> DependentRc<T> {
    pub fn new(item: T) -> DependentRc<T> {
        DependentRc {
            item: Rc::new(item),
            dependants: Vec::new()
        }
    }

    pub fn into_view_internal<X,Y,G,F, H>(&mut self, conversion: F, downgrade: G, to_any: H) -> Y
    where F : FnOnce(&Rc<T>) -> X,
          G : FnOnce(&X) -> Y,
          H : FnOnce(X) -> Rc<Any>
    {
        let reference : X = conversion(&self.item);
        let reference : Rc<Any> = to_any(reference);
        let reference : &Rc<Any> = push_ref(&mut self.dependants, reference);
        let reference : &X = unsafe { transmute(reference) };
        downgrade(reference)
    }
}



impl<T> Deref for DependentRc<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Rc<T> {
        &self.item
    }
}
impl<T> DerefMut for DependentRc<T> {
    fn deref_mut(&mut self) -> &mut Rc<T> {
        &mut self.item
    }
}

