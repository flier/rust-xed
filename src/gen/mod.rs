use lazy_static::lazy_static;

use crate::ffi;

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
pub use self::operand::*;

impl Operand {
    pub fn is_register(&self) -> bool {
        unsafe { ffi::xed_operand_is_register((*self).into()) != 0 }
    }
}

mod reg_class;
pub use self::reg_class::*;

mod reg;
pub use self::reg::*;

mod syntax;
pub use self::syntax::*;
