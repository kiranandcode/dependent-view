#[macro_use]
extern crate dependent_view;

use dependent_view::rc;

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




fn main() {
    println!("Hello world");
}
