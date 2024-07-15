#![deny(clippy::all)]
#![deny(unsafe_code)]

// mod core;
// pub mod handler;
mod server;

mod db;

use src_lang::compiler;

pub use server::*;

pub use src_lang::Jar;
