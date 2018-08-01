#![allow(dead_code)]

//! dependent_view is a rust library providing simple wrappers around the `Rc` and `Arc` types, imbuing them with the capability to provide "views" of non-owned structs to separate components of a system. 
//! 
//! ## Usage
//! Add this to your `Cargo.toml`
//! ```ignore,text
//! [dependencies]
//! dependent_view="1.0.1"
//! ```
//! and this to your crate root:
//! ```ignore
//! #[macro_use]
//! extern crate dependent_view;
//! ```
//! 
//! The library provides two main structs `DependentRc` and `DependentArc` for normal and thread-safe views.
//!
//! These change the result of the view type (between `std::rc::Weak` or `std::sync::Weak`).
//! 
//! To obtain a `Weak<Trait>` from a dependent struct, use the macros `to_view!()` or `to_view_sync()` respectively.
//! 
//! The compiler will check at compile time that the type `T` within `DependentRc<T>` impl's the trait you want to obtain a view for. 
//! 
//! These dependent types provide a different kind of ownership delegation as compared to standard `Rc`'s or `Box`'s.
//! 
//! A `DependentRc` should be viewed as the single owner of it's contained type, however unlike a `Box`, it allows users to generate multiple runtime managed `Weak<Trait>` references to the object (for each `Trait` impl'd by the contained entity) - these `Weak` references cease to be upgradable once the source `DependantRc` is dropped.
//! 
//! 
//! ## Example
//! Assume we have the following traits:
//! ```
//! trait Dance {
//!     fn dance(&self);
//! }
//! 
//! trait Prance {
//!     fn prance(&self);
//! }
//! ```
//! and some structs which impl the traits:
//! ```
//! # trait Dance {
//! #    fn dance(&self);
//! # }
//! # trait Prance {
//! #     fn prance(&self);
//! # }
//! struct Dancer {id: usize}
//! impl Dance for Dancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! impl Prance for Dancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! 
//! struct Prancer {id: usize}
//! impl Dance for Prancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! impl Prance for Prancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! ```
//! We can create `DependentRc` using the new function:
//! ```
//! # #[macro_use] extern crate dependent_view;
//! use dependent_view::rc::*;
//! # trait Dance {
//! #    fn dance(&self);
//! # }
//! # trait Prance {
//! #     fn prance(&self);
//! # }
//! # struct Dancer {id: usize}
//! # impl Dance for Dancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! # impl Prance for Dancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! # struct Prancer {id: usize}
//! # impl Dance for Prancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! # impl Prance for Prancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! let mut dancer = DependentRc::new(Dancer { id: 0 });
//! let mut prancer = DependentRc::new(Prancer { id: 0 });
//! ```
//! 
//! We can use these `DependentRc`'s to create non-owned views of our structs:
//! 
//! ```
//! # #[macro_use] extern crate dependent_view;
//! # use std::rc::Weak;
//! # use dependent_view::rc::*;
//! # trait Dance {
//! #    fn dance(&self);
//! # }
//! # trait Prance {
//! #     fn prance(&self);
//! # }
//! # struct Dancer {id: usize}
//! # impl Dance for Dancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! # impl Prance for Dancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! # struct Prancer {id: usize}
//! # impl Dance for Prancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! # impl Prance for Prancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! # let mut dancer = DependentRc::new(Dancer { id: 0 });
//! # let mut prancer = DependentRc::new(Prancer { id: 0 });
//! let dancer_dance_view : Weak<Dance> = to_view!(dancer);
//! let dancer_prance_view : Weak<Prance> = to_view!(dancer);
//! 
//! let prancer_dance_view : Weak<Dance> = to_view!(prancer);
//! let prancer_prance_view : Weak<Prance> = to_view!(prancer);
//! ```
//! 
//! We can then share these views to other components, and not have to worry about managing their deletion:
//! ```
//! # #[macro_use] extern crate dependent_view;
//! # use dependent_view::rc::DependentRc;
//! # use std::rc::{Rc, Weak};
//! # trait Dance {
//! #    fn dance(&self);
//! # }
//! # trait Prance {
//! #     fn prance(&self);
//! # }
//! # struct Dancer {id: usize}
//! # impl Dance for Dancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! # impl Prance for Dancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! # struct Prancer {id: usize}
//! # impl Dance for Prancer {fn dance(&self) {println!("D{:?}", self.id);}}
//! # impl Prance for Prancer {fn prance(&self)  {println!("P{:?}", self.id);}}
//! # pub fn main() {
//! let mut dancers : Vec<Weak<Dance>> = Vec::new();
//! let mut prancers : Vec<Weak<Prance>> = Vec::new();
//! 
//! {
//!     let mut dancer = DependentRc::new(Dancer { id: 0 });
//!     let mut prancer = DependentRc::new(Prancer { id: 0 });
//! 
//!     dancers.push(to_view!(dancer));
//!     prancers.push(to_view!(dancer));
//!     dancers.push(to_view!(prancer));
//!     prancers.push(to_view!(prancer));
//! 
//!     for (dancer_ref, prancer_ref) in dancers.iter().zip(prancers.iter()) {
//!         dancer_ref.upgrade().unwrap().dance(); 
//!         prancer_ref.upgrade().unwrap().prance(); 
//!     }
//! 
//!     // at this point, dancer and prancer are dropped, invalidating the views
//! }
//! 
//! 
//! for (dancer_ref, prancer_ref) in dancers.iter().zip(prancers.iter()) {
//!     assert!(dancer_ref.upgrade().is_none());
//!     assert!(prancer_ref.upgrade().is_none());
//! }
//! # }
//! ```
//! Also, it is a compile time error to attempt to produce a trait view of a struct when the underlying struct doesn't implement the trait:
//! ```ignore
//! struct Bad { id: usize }
//! let bad = DependentRc::new(Bad { id: 0 });
//! let bad_view : Weak<Dance> = to_view!(bad); // compile time error
//! ```
//! See [`example.rs`](https://github.com/Gopiandcode/dependent-view/blob/master/example.rs) for the full source.


#[macro_use]
pub mod rc;


#[macro_use]
pub mod arc;


fn push_ref<T>(items: &mut Vec<T>, value: T) -> &T {
    items.push(value);
    &items[items.len() - 1]
}

