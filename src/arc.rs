#![allow(dead_code)]

//! Module defining DependentArc, a wrapper around the Arc type.
//!
//! This module both defines the `DependentArc` struct, as well as the corresponding `to_view_sync!` macro, which can be used to obtain thread safe views from an instance of `DependentArc`.
//!
//! # Examples

//! ```
//! # #[macro_use] extern crate dependent_view;
//! # use dependent_view::arc::DependentArc;
//! # use std::thread;
//! # use std::sync::{Weak, Arc};
//! # use std::time;
//! # trait Dance : Send + Sync {
//! #    fn dance(&self);
//! # }
//! # trait Prance : Send + Sync {
//! #     fn prance(&self);
//! # }
//! # struct Dancer {id: usize}
//! # impl Dance for Dancer {fn dance(&self) {print!("D{:?}", self.id);}}
//! # impl Prance for Dancer {fn prance(&self)  {print!("P{:?}", self.id);}}
//! # struct Prancer {id: usize}
//! # impl Dance for Prancer {fn dance(&self) {print!("D{:?}", self.id);}}
//! # impl Prance for Prancer {fn prance(&self)  {print!("P{:?}", self.id);}}
//! # pub fn main() {
//! let mut dancers : Vec<DependentArc<Dancer>> = Vec::new();
//! let mut prancers : Vec<DependentArc<Prancer>> = Vec::new();
//! 
//! let mut dance_refs : Vec<Weak<Dance>> = Vec::new();
//! let mut prance_refs : Vec<Weak<Prance>> = Vec::new();
//! 
//! for i in 0..10 {
//!     let mut dancer = DependentArc::new(Dancer { id: i });
//!     let mut prancer = DependentArc::new(Prancer { id: i+10 });
//! 
//!     dance_refs.push(to_view_sync!(dancer));
//!     prance_refs.push(to_view_sync!(prancer));
//! 
//!     dancers.push(dancer);
//!     prancers.push(prancer);
//! }
//! 
//! // owning thread
//! let h1 = thread::spawn(move|| {
//!         let mut count = dancers.len() + prancers.len();
//!         let mut seed = 13;
//!         while count > 0 {
//!             // drop a random reference
//! #            seed = (seed * 29)%71;
//! #            if (seed > dancers.len() && prancers.len() > 0) || dancers.len() == 0 {
//! #                let length = prancers.len();
//! #                prancers.swap_remove((seed - dancers.len())%length);
//! #            } else {
//! #                let length = dancers.len();
//! #                dancers.swap_remove(seed%length);
//! #            }
//!             count -= 1;
//!             println!("\nRemaining items {:?}", count);
//!         }
//! });
//! 
//! let h2 = thread::spawn(move || {
//!     loop {
//! #        let mut count = 0;
//! 
//!         for (dancer_ref, prancer_ref) in dance_refs.iter().zip(prance_refs.iter()) {
//!             if let Some(dref) = dancer_ref.upgrade() {
//!                 dref.dance();
//! #                count += 1;
//!             }
//!             if let Some(pref) = prancer_ref.upgrade() {
//!                 pref.prance();
//! #                count += 1;
//!             }
//!         }
//! #        if count == 0 { break; }
//! #        println!("\nRemaining references {:?}", count);
//!     }
//! });
//! 
//! h1.join();
//! h2.join();
//! # }
//! ```


use super::push_ref;

use std::sync::Arc;
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::convert::*;


/// Macro for obtaining thread safe views from DependentArc
///
/// # Error
/// It is a compile time error to use this macro to produce a view for a trait that the underlying struct does not implement.
/// 
/// # Examples
///
/// ```
/// # use std::sync::Weak;
/// # #[macro_use] extern crate dependent_view;
/// # use dependent_view::arc::DependentArc;
/// struct ExampleStruct {
///    // arbitrary fields
/// # id: usize
/// }
/// trait ExampleTrait {
///   fn example_method(&self);
/// }
/// # impl ExampleStruct { fn new() -> Self { ExampleStruct {id: 0}}}
/// impl ExampleTrait for ExampleStruct {
/// #        fn example_method(&self) {
///              // some implementation...
/// #            println!("id: {:?}", self.id);
/// #          }
/// }
/// # fn main() {
/// let mut item : DependentArc<ExampleStruct> = DependentArc::new(ExampleStruct::new());
/// let view : Weak<ExampleTrait> = to_view_sync!(item);
/// # }
/// ```
#[macro_export]
macro_rules! to_view_sync {
    ($dep:tt) => {
        unsafe {($dep.into_view_internal_sync::<_, ::std::sync::Weak<_>,_, _, _>(|item| item.clone() as ::std::sync::Arc<_>, |item| ::std::sync::Arc::downgrade(item), |item| unsafe { ::std::mem::transmute(item) }))};
    }
}


/// `DependentArc<T>` is a simple wrapper around the `Arc<T>`  type, imbuing it with the capability to provide thread safe "views" (`Weak<Trait>`) of non-owned structs to separate components of a system. 
///
/// Internally, it does this by retaining an `Arc<Trait>` for each view you make - thus when the
/// `DependentArc` is dropped, all of the weak references are automatically invalidated.
pub struct DependentArc<T> {
    item: Arc<T>,
    dependants: Vec<Arc<Any + Send + Sync>>
}



impl<T> DependentArc<T> {
    pub fn new(item: T) -> DependentArc<T> {
        DependentArc {
            item: Arc::new(item),
            dependants: Vec::new()
        }
    }

    /// internal hidden function used to produce a Weak reference
    /// # Warn
    /// This function should only be called through the `to_view_sync!` macro. It is not intended for direct use.
    /// # Remarks
    /// This struct works by cloning the underlying Arc, using the fact that when cloning, it is possible
    /// to upcast an `Arc<Concrete>` to an `Arc<Trait>`.
    /// From this point, the `Arc<Trait>` is transmuted into an `Arc<Any>`. While this is an unsafe,
    /// unchecked cast, we just want to keep the on Drop functionality of `Arc<Any>` and don't provide
    /// any ways to use the trait, so it shouldn't be a problem
    #[doc(hidden)]
    pub unsafe fn into_view_internal_sync<X,Y,G,F, H>(&mut self, conversion: F, downgrade: G, to_any: H) -> Y
    where F : FnOnce(&Arc<T>) -> X,
    G : FnOnce(&X) -> Y,
    H : FnOnce(X) -> Arc<Any + Send + Sync>
    {
        let reference : X = conversion(&self.item);
        let reference : Arc<Any + Send + Sync> = to_any(reference);
        let reference : &Arc<Any + Send + Sync> = push_ref(&mut self.dependants, reference);
        let reference : &X =  transmute(reference) ;
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

/// Returns a reference to the underlying `Arc` struct
impl<T> AsRef<Arc<T>> for DependentArc<T> {
    fn as_ref(&self) -> &Arc<T> {
        &self.item
    }
}

/// Returns a mutable reference to the underlying `Arc` struct
impl<T> AsMut<Arc<T>> for DependentArc<T> {
    fn as_mut(&mut self) -> &mut Arc<T> {
        &mut self.item
    }
}


/// Constructs a DependentArc from a `Arc`, imbuing it with the capability to produce views.
impl <T> From<Arc<T>> for DependentArc<T> {
    fn from(item: Arc<T>) -> DependentArc<T> {
        DependentArc {
            item,
            dependants: Vec::new()
        }
    }
}

/// Unwraps the `DependentArc`, returning it's internal `Arc`
///
/// Note: This will invalidate all `Weak<Trait>` views you have constructed from this object.
impl <T> Into<Arc<T>> for DependentArc<T> {
    fn into(self) -> Arc<T> {
        self.item
    }
}
