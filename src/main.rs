#[macro_use]
extern crate dependent_view;

use dependent_view::rc::DependentRc;
use std::rc::{Rc, Weak};



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

pub fn main() {
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

}

