#![deny(clippy::all)]
#![deny(unsafe_code)]

// mod core;
// pub mod handler;
mod server;

mod db;

pub use server::*;
