pub extern crate xed_sys as ffi;

#[macro_use]
mod macros;

mod address;
mod chip_features;
mod decoded;
pub mod enc;
mod error;
mod flags;
mod gen;
mod inst;
mod machine;
mod operand;
mod raw;
mod state;
pub mod tables;
mod util;

pub use self::address::Width as AddressWidth;
pub use self::decoded::{
    Inst as DecodedInst, Operand as DecodedOperand, Operands as DecodedOperands,
};
pub use self::error::{Error, Result};
pub use self::flags::{FlagAction, FlagSet, SimpleFlag};
pub use self::gen::*;
pub use self::inst::Inst;
pub use self::machine::Mode as MachineMode;
pub use self::operand::Operand;
pub use self::state::State;
pub use self::util::*;
