use super::DependableView;

use std::rc::{Rc, Weak};
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! dependable_register_trait {
    ($type: tt) => {
        impl<T : $type> dependent_view::DependableView<::std::rc::Weak<$type>> for dependent_view::rc::DependentRc<T> {
            fn into_view(&mut self) -> ::std::rc::Weak<$type> {
                let reference : ::std::rc::Rc<$type> = self.item.clone();
                let reference : ::std::rc::Rc<::std::any::Any> = unsafe { ::std::mem::transmute(reference) } ; 
                let reference : &::std::rc::Rc<::std::any::Any> = dependent_view::push_ref(&mut self.dependants, reference);
                let reference : &::std::rc::Rc<$type> = unsafe { ::std::mem::transmute(reference) };

                ::std::rc::Rc::downgrade(reference)
            }
        }
    };
    ($type: tt, $($rest: tt),+) => {
        dependable_register_trait!($type);

        dependable_register_trait!($($rest),*);
    };
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

