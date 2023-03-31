#![feature(format_args_nl)]
#![feature(allow_internal_unstable)]
#![feature(core_panic)]

//mod iterator2d;
mod assert;
mod grammar;
mod tree;
mod combination;
mod ctx;
mod parse;
mod ffi;
mod atend;  

pub use grammar::*;
pub use tree::*;

pub use parse::{
    parse,
    make_ctx,
    Error
};

pub mod internal {
    pub use crate::combination::{
        generate_combinations,
        expand_combinations,
        expand_combinations_iter
    };

    pub use crate::ctx::{
        Ctx
    };
}

pub use assert::init_assert_contains_tree;

pub use ffi::*;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;