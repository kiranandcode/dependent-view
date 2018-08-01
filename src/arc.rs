use super::push_ref;

use std::sync::{Arc, Weak};
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};


#[macro_export]
macro_rules! to_view_sync {
    ($dep:tt) => {
        ($dep.into_view_internal_sync::<_, _,_, _, _>(|item| item.clone() as ::std::sync::Arc<_>, |item| ::std::sync::Arc::downgrade(item), |item| unsafe { ::std::mem::transmute(item) }));
    }
}




struct DependentArc<T> {
    item: Arc<T>,
    dependants: Vec<Arc<Any>>
}



impl<T> DependentArc<T> {
    fn new(item: T) -> DependentArc<T> {
        DependentArc {
            item: Arc::new(item),
            dependants: Vec::new()
        }
    }
    pub fn into_view_internal_sync<X,Y,G,F, H>(&mut self, conversion: F, downgrade: G, to_any: H) -> Y
    where F : FnOnce(&Arc<T>) -> X,
    G : FnOnce(&X) -> Y,
    H : FnOnce(X) -> Arc<Any>
    {
        let reference : X = conversion(&self.item);
        let reference : Arc<Any> = to_any(reference);
        let reference : &Arc<Any> = push_ref(&mut self.dependants, reference);
        let reference : &X = unsafe { transmute(reference) };
        downgrade(reference)
    }

}

impl<T> Deref for DependentArc<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.item
    }
}
impl<T> DerefMut for DependentArc<T> {

    fn deref_mut(&mut self) -> &mut Arc<T> {
        &mut self.item
    }
}


