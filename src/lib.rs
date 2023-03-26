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

pub use grammar::*;
pub use tree::*;

pub use parse::{
    parse,
    make_ctx,
    Error
};

pub use assert::init_assert_contains_tree;

pub use ffi::*;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;