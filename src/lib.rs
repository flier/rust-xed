pub extern crate xed_sys as ffi;

#[macro_use]
mod macros;

mod addr;
mod chip;
pub mod dec;
pub mod enc;
mod error;
pub mod flag;
pub mod fmt;
mod gen;
mod iclass;
mod iform;
mod inst;
mod machine;
mod operand;
mod raw;
mod state;
pub mod tables;
mod util;

pub use self::addr::Width as AddressWidth;
pub use self::chip::Features as ChipFeatures;
pub use self::error::{Error, Result};
pub use self::flag::SimpleFlag;
pub use self::gen::*;
pub use self::iform::Info as IformInfo;
pub use self::inst::Inst;
pub use self::machine::Mode as MachineMode;
pub use self::operand::Operand;
pub use self::state::State;
pub use self::util::*;
