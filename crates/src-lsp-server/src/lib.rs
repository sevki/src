#![deny(clippy::all)]
#![deny(unsafe_code)]

// mod core;
// pub mod handler;
mod server;

mod db;

use srclang::compiler;

pub use server::*;

pub use srclang::Jar;