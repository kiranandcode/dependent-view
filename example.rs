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

struct Dancer {id: usize}
impl Dance for Dancer {fn dance(&self) {println!("D{:?}", self.id);}}
impl Prance for Dancer {fn prance(&self)  {println!("P{:?}", self.id);}}

struct Prancer {id: usize}
impl Dance for Prancer {fn dance(&self) {println!("D{:?}", self.id);}}
impl Prance for Prancer {fn prance(&self)  {println!("P{:?}", self.id);}}



pub fn main() {
    let mut dancers : Vec<Weak<Dance>> = Vec::new();
    let mut prancers : Vec<Weak<Prance>> = Vec::new();

    {
        let mut dancer = DependentRc::new(Dancer { id: 0 });
        let mut prancer = DependentRc::new(Prancer { id: 0 });

        dancers.push(to_view!(dancer));
        prancers.push(to_view!(dancer));
        dancers.push(to_view!(prancer));
        prancers.push(to_view!(prancer));

        for (dancer_ref, prancer_ref) in dancers.iter().zip(prancers.iter()) {
            dancer_ref.upgrade().unwrap().dance(); 
            prancer_ref.upgrade().unwrap().prance(); 
        }

        // at this point, dancer and prancer are dropped, invalidating the views
    }


    for (dancer_ref, prancer_ref) in dancers.iter().zip(prancers.iter()) {
        assert!(dancer_ref.upgrade().is_none());
        assert!(prancer_ref.upgrade().is_none());
    }

}
