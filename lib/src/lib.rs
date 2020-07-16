#![feature(const_fn)]
#![feature(const_generics)]
#![feature(negative_impls)]

mod colour;
pub use colour::*;

mod fn_writer;
pub use fn_writer::*;

mod init;
pub use init::*;

mod logger;
pub use logger::*;

mod matrix;
pub use matrix::*;
