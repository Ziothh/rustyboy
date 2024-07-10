#![allow(dead_code)] // TODO: Dev only
#![feature(generic_arg_infer)]
// #![feature(inherent_associated_types)] // Cool feature but rust-analyzer doesn't support
// autocomplete for const values with these types

pub mod hardware;
pub mod io;
pub mod prelude;
pub mod program;
pub mod utils;
