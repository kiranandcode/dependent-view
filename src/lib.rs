#![allow(dead_code)]


#[macro_use]
pub mod rc;
#[macro_use]
pub mod arc;


fn push_ref<T>(items: &mut Vec<T>, value: T) -> &T {
    items.push(value);
    &items[items.len() - 1]
}
