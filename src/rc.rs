#![allow(dead_code)]
use std::rc::{Rc};
use std::rc;
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! dependable_register_trait {
    ($type: tt) => {
        impl<T : $type> Dependable<rc::Weak<$type>> for DependentRc<T> {
            fn retrieve_dependancy(&mut self) -> rc::Weak<$type> {
                let reference : Rc<$type> = self.item.clone();
                let reference : Rc<Any> = unsafe { transmute(reference) } ; 
                let reference : &Rc<Any> = push_ref(&mut self.dependants, reference);
                let reference : &Rc<$type> = unsafe { transmute(reference) };

                Rc::downgrade(reference)
            }
        }
    };
    ($type: tt, $($rest: tt),+) => {
        dependable_register_trait!($type);

        dependable_register_trait!($($rest),*);
    };
}

fn push_ref<T>(items: &mut Vec<T>, value: T) -> &T {
    items.push(value);
    &items[items.len() - 1]
}


struct DependentRc<T> {
    item: Rc<T>,
    dependants: Vec<Rc<Any>>
}



impl<T> DependentRc<T> {
    fn new(item: T) -> DependentRc<T> {
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

trait Dependable<T> {
    fn retrieve_dependancy(&mut self) -> T;
}

dependable_register_trait!(Prance, Dance);




trait Dance {
    fn dance(&self);
}

trait Prance {
    fn prance(&self);
}

struct Dancer {
    id: usize
}

impl Dance for Dancer {
    fn dance(&self) {
        println!("Dancing {:?}", self.id);
    }
}

impl Prance for Dancer {
    fn prance(&self)  {
        println!("Prancing {:?}", self.id + 1);
    }
}


