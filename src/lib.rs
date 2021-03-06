#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(pub_restricted)]

#[macro_use] extern crate lazy_static;
#[macro_use] mod macros;


extern crate gobject_sys;
extern crate glib_sys;

mod g;
mod gobject;
mod ptr;
mod mock;

pub mod prelude {
    pub use ptr::Ptr;
}

pub use mock::{
    counter_add,
    counter_get,
    counter_get_type,
    counter_new,
};
