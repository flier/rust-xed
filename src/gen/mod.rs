use lazy_static::lazy_static;

use crate::{enum_properties, ffi};

mod attribute;
pub use self::attribute::*;

impl Attribute {
    pub fn tables() -> &'static [Attribute] {
        &*TABLE
    }
}

lazy_static! {
    static ref TABLE: Vec<Attribute> = unsafe {
        (0..ffi::xed_attribute_max())
            .map(|i| ffi::xed_attribute(i).into())
            .collect()
    };
}

mod category;
pub use self::category::*;

mod chip;
pub use self::chip::*;

mod cpuid_bit;
pub use self::cpuid_bit::*;

mod error;
pub use self::error::Error as Errno;

impl Errno {
    pub fn is_none(self) -> bool {
        self == Self::NONE
    }
}

impl std::error::Error for Errno {}

mod exception;
pub use self::exception::*;

mod extension;
pub use self::extension::*;

mod flag_action;
pub use self::flag_action::*;

mod flag;
pub use self::flag::*;

mod iclass;
pub use self::iclass::*;

impl Iclass {
    enum_properties! {
        iform_max: u32 { xed_iform_max_per_iclass }
        iform_first: u32 { xed_iform_first_per_iclass }
    }
}

mod iform;
pub use self::iform::*;

mod iformfl;
pub use self::iformfl::*;

mod isa_set;
pub use self::isa_set::*;

mod nonterminal;
pub use self::nonterminal::*;

mod operand_action;
pub use self::operand_action::*;

mod operand_convert;
pub use self::operand_convert::*;

mod operand_element_xtype;
pub use self::operand_element_xtype::*;

mod operand_type;
pub use self::operand_type::*;

mod operand_visibility;
pub use self::operand_visibility::*;

mod operand_width;
pub use self::operand_width::*;

mod operand;
pub use self::operand::Operand as Op;

impl Op {
    enum_properties! {
        is_register: bool { xed_operand_is_register }
    }
}

mod reg_class;
pub use self::reg_class::*;

mod reg;
pub use self::reg::Reg;

impl Reg {
    enum_properties! {
        /// Returns the register class of the given input register.
        class: RegClass? { xed_reg_class }

        /// Returns the specific width GPR reg class
        gpr_class: RegClass? { xed_gpr_reg_class }

        /// Returns the largest enclosing register for any kind of register
        ///
        /// This is mostly useful for GPRs. (64b mode assumed)
        largest_enclosing_register: Reg { xed_get_largest_enclosing_register }

        /// Returns the largest enclosing register for any kind of register
        ///
        /// This is mostly useful for GPRs in 32b mode.
        largest_enclosing_register32: Reg { xed_get_largest_enclosing_register32 }

        /// Returns the width, in bits, of the named register. 32b mode
        width_bits: u32 { xed_get_register_width_bits }

        /// Returns the width, in bits, of the named register. 64b mode
        width_bits64: u32 { xed_get_register_width_bits64 }
    }
}

mod syntax;
pub use self::syntax::*;

mod operand_element_type;
pub use self::operand_element_type::*;
