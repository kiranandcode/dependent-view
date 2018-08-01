use super::DependableView;

use std::sync::{Arc, Weak};
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! dependable_sync_register_trait {
    ($type: tt) => {
        impl<T : $type> DependableView<Weak<$type>> for DependentArc<T> {
            fn into_view(&mut self) -> Weak<$type> {
                let reference : Arc<$type> = self.item.clone();
                let reference : Arc<Any> = unsafe { transmute(reference) } ; 
                let reference : &Arc<Any> = push_ref(&mut self.dependants, reference);
                let reference : &Arc<$type> = unsafe { transmute(reference) };

                Arc::downgrade(reference)
            }
        }
    };
    ($type: tt, $($rest: tt),+) => {
        dependable_sync_register_trait!($type);

        dependable_sync_register_trait!($($rest),*);
    };
}

fn push_ref<T>(items: &mut Vec<T>, value: T) -> &T {
    items.push(value);
    &items[items.len() - 1]
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

