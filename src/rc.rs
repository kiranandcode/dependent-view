#![allow(dead_code)]

//! Module defining DependentRc, a wrapper around the Rc type.
//!
//! This module both defines the `DependentRc` struct, as well as the corresponding `to_view!` macro, which can be used to obtain views from an instance of `DependentRc`.
//!


use super::push_ref;

use std::rc::Rc;
use std::convert::*;
use std::any::Any;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};


/// Macro for obtaining views from DependentRc
///
/// # Error
/// It is a compile time error to use this macro to produce a view for a trait that the underlying struct does not implement.
/// 
/// # Examples
///
/// ```
/// # use std::rc::Weak;
/// # #[macro_use] extern crate dependent_view;
/// # use dependent_view::rc::DependentRc;
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
/// let mut item : DependentRc<ExampleStruct> = DependentRc::new(ExampleStruct::new());
/// let view : Weak<ExampleTrait> = to_view!(item);
/// # }
/// ```
#[macro_export]
macro_rules! to_view {
    ($dep:tt) => {
        (unsafe { $dep.into_view_internal::<_, _,_, _, _>(|item| item.clone() as ::std::rc::Rc<_>, |item| ::std::rc::Rc::downgrade(item), |item| unsafe { ::std::mem::transmute(item) })});
    }
}



/// `DependentRc<T>` is a simple wrapper around the `Rc<T>`  type, imbuing it with the capability to provide "views" (`Weak<Trait>`) of non-owned structs to separate components of a system. 
///
/// Internally, it does this by retaining an `Rc<Trait>` for each view you make - thus when the
/// `DependentRc` is dropped, all of the weak references are automatically invalidated.
pub struct DependentRc<T> {
    item: Rc<T>,
    dependants: Vec<Rc<Any>>
}



impl<T> DependentRc<T> {
    /// Constructs a `DependentRc` by wrapping an underlying type
    pub fn new(item: T) -> DependentRc<T> {
        DependentRc {
            item: Rc::new(item),
            dependants: Vec::new()
        }
    }

    /// internal hidden function used to produce a Weak reference
    /// # Warn
    /// This function should only be called through the `to_view!` macro. It is not intended for direct use.
    /// # Remarks
    /// This struct works by cloning the underlying Rc, using the fact that when cloning, it is possible
    /// to upcast an `Rc<Concrete>` to an `Rc<Trait>`.
    /// From this point, the `Rc<Trait>` is transmuted into an `Rc<Any>`. While this is an unsafe,
    /// unchecked cast, we just want to keep the on Drop functionality of `Rc<Any>` and don't provide    /// any ways to use the trait, so it shouldn't be a problem
    #[doc(hidden)]
    pub unsafe fn into_view_internal<X,Y,G,F, H>(&mut self, conversion: F, downgrade: G, to_any: H) -> Y
    where F : FnOnce(&Rc<T>) -> X,
          G : FnOnce(&X) -> Y,
          H : FnOnce(X) -> Rc<Any>
    {
        let reference : X = conversion(&self.item);
        let reference : Rc<Any> = to_any(reference);
        let reference : &Rc<Any> = push_ref(&mut self.dependants, reference);
        let reference : &X =  transmute(reference);
        downgrade(reference)
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


/// Returns a reference to the underlying `Rc` struct
impl<T> AsRef<Rc<T>> for DependentRc<T> {
    fn as_ref(&self) -> &Rc<T> {
        &self.item
    }
}

/// Returns a mutable reference to the underlying `Rc` struct
impl<T> AsMut<Rc<T>> for DependentRc<T> {
    fn as_mut(&mut self) -> &mut Rc<T> {
        &mut self.item
    }
}


/// Constructs a DependentRc from an `Rc`, imbuing it with the capability to produce views.
impl <T> From<Rc<T>> for DependentRc<T> {
    fn from(item: Rc<T>) -> DependentRc<T> {
        DependentRc {
            item,
            dependants: Vec::new()
        }
    }
}

/// Unwraps the `DependentRc`, returning it's internal `Rc`
///
/// Note: This will invalidate all `Weak<Trait>` views you have constructed from this object.
impl <T> Into<Rc<T>> for DependentRc<T> {
    fn into(self) -> Rc<T> {
        self.item
    }
}
