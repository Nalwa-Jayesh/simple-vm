#![feature(trace_macros)]

pub mod binfmt;
pub mod io;
pub mod memory;
pub mod op;
pub mod op_fields;
pub mod pp;
pub mod register;
pub mod resolve;
pub mod vm;

trace_macros!(true);

pub use crate::io::*;
pub use crate::memory::*;
pub use crate::op::*;
pub use crate::op_fields::*;
pub use crate::register::*;
pub use crate::vm::*;

#[cfg(target_family = "wasm")]
pub mod wasm;
