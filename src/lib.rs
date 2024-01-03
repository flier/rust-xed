pub extern crate xed_sys as ffi;

mod address;
mod decoded;
mod error;
mod gen;
mod inst;
mod machine;
pub mod tables;

pub use self::address::Width as AddressWidth;
pub use self::decoded::Inst as DecodedInst;
pub use self::error::{Error, Result};
pub use self::gen::{Error as Errno, *};
pub use self::machine::Mode as MachineMode;
