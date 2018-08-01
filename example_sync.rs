#[macro_use]
extern crate dependent_view;

use dependent_view::arc::DependentArc;
use std::thread;
use std::sync::{Weak, Arc};
use std::time;


trait Dance : Send + Sync {
    fn dance(&self);
}

trait Prance : Send + Sync {
    fn prance(&self);
}

struct Dancer {id: usize}
impl Dance for Dancer {fn dance(&self) {print!("D{:?}", self.id);}}
impl Prance for Dancer {fn prance(&self)  {print!("P{:?}", self.id);}}

struct Prancer {id: usize}
impl Dance for Prancer {fn dance(&self) {print!("D{:?}", self.id);}}
impl Prance for Prancer {fn prance(&self)  {print!("P{:?}", self.id);}}



pub fn main() {


    let mut dancers : Vec<DependentArc<Dancer>> = Vec::new();
    let mut prancers : Vec<DependentArc<Prancer>> = Vec::new();

    let mut dance_refs : Vec<Weak<Dance>> = Vec::new();
    let mut prance_refs : Vec<Weak<Prance>> = Vec::new();

    for i in 0..10 {
        let mut dancer = DependentArc::new(Dancer { id: i });
        let mut prancer = DependentArc::new(Prancer { id: i+10 });

        dance_refs.push(to_view_sync!(dancer));
        prance_refs.push(to_view_sync!(prancer));

        dancers.push(dancer);
        prancers.push(prancer);
    }




    // owning thread
    let h1 = thread::spawn(move|| {
            let mut count = dancers.len() + prancers.len();
            let mut seed = 13;


            while count > 0 {
                seed = (seed * 29)%71;
                if (seed > dancers.len() && prancers.len() > 0) || dancers.len() == 0 {
                    let length = prancers.len();
                    prancers.swap_remove((seed - dancers.len())%length);
                } else {
                    let length = dancers.len();
                    dancers.swap_remove(seed%length);
                }

                count -= 1;
                thread::sleep(time::Duration::from_millis(1000));
            }


            println!("\nRemaining items {:?}", count);
            thread::sleep(time::Duration::from_millis(1000));
    });



    let h2 = thread::spawn(move || {

        loop {
            let mut count = 0;

            for (dancer_ref, prancer_ref) in dance_refs.iter().zip(prance_refs.iter()) {
                if let Some(dref) = dancer_ref.upgrade() {
                    dref.dance(); 
                    count += 1;
                }
                if let Some(pref) = prancer_ref.upgrade() {
                    pref.prance();
                    count += 1;
                }

            }
            if count == 0 { break; }
            println!("\nRemaining references {:?}", count);
            thread::sleep(time::Duration::from_millis(1000));
            let dref = dance_refs.remove(0);
            thread::spawn(move || (dref.upgrade().unwrap().dance()));
        }

    });


    h1.join();
    h2.join();


}
