#![allow(dead_code)]
use std::rc::{Rc, Weak};
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};


fn push_ref<T>(items: &mut Vec<T>, value: T) -> &T {
    items.push(value);
    &items[items.len() - 1]
}


struct DependentRc<T> {
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

#[macro_export]
macro_rules! to_view {
    ($dep:tt) => {
        ($dep.into_view_internal::<_, _,_, _, _>(|item| item.clone() as Rc<_>, |item| Rc::downgrade(item), |item| unsafe { transmute(item) }));
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

pub fn run() {
    let mut dancers : Vec<Weak<Dance>> = Vec::new();
    let mut prancers : Vec<Weak<Prance>> = Vec::new();

    {
        let mut reference = DependentRc::new(Dancer { id: 0 });

        dancers.push(to_view!(reference));
        prancers.push(to_view!(reference));

        for refr in dancers.iter() {
            if let Some(refr) = refr.upgrade() {
                refr.dance();
            }
        }
        for refr in prancers.iter() {
            if let Some(refr) = refr.upgrade() {
                refr.prance();
            }
        }

    }


    for refr in dancers.iter() {
        if let Some(refr) = refr.upgrade() {
            refr.dance();
        } else {
            println!("Reference dropped");
        }
    }
    for refr in prancers.iter() {
        if let Some(refr) = refr.upgrade() {
            refr.prance();
        } else {
            println!("Reference dropped");
        }
    }


    println!("Hello world");

}
