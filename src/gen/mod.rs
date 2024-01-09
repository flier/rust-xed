use std::mem::MaybeUninit;

use derive_more::{Deref, From, Into};
use lazy_static::lazy_static;

use crate::raw::ToBool;
use crate::{enum_properties, ffi};

mod attribute;
pub use self::attribute::*;

impl Attribute {
    pub fn tables() -> &'static [Attribute] {
        &TABLE
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
pub use self::flag_action::FlagAction;

impl FlagAction {
    enum_properties! {
        /// returns true if the specified action is invalid. Only the 2nd flag might be invalid.
        invalid: bool { xed_flag_action_action_invalid }

        /// test to see if the specific action is a read
        read: bool { xed_flag_action_read_action }

        /// test to see if a specific action is a write
        write: bool { xed_flag_action_write_action }
    }
}

mod flag;
pub use self::flag::*;

mod iclass;
pub use self::iclass::Iclass;

mod iform;
pub use self::iform::Iform;

mod iformfl;
pub use self::iformfl::*;

mod isa_set;
pub use self::isa_set::IsaSet;

impl IsaSet {
    /// Returns the name of the cpuid bit associated with this isa-set.
    pub fn cpuid_groups(self) -> impl Iterator<Item = CpuidGroup> {
        unsafe {
            (0..ffi::XED_MAX_CPUID_GROUPS_PER_ISA_SET).flat_map(move |i| {
                let g = ffi::xed_get_cpuid_group_enum_for_isa_set(self.into(), i);

                (g != 0).then(|| g.into())
            })
        }
    }

    /// Returns the name of the cpuid record associated with the given cpuid group.
    pub fn cpuid_recs(self) -> impl Iterator<Item = CpuidRec> {
        unsafe {
            (0..ffi::XED_MAX_CPUID_RECS_PER_GROUP).flat_map(move |i| {
                let r = ffi::xed_get_cpuid_rec_enum_for_group(self.into(), i);

                (r != 0).then(|| r.into())
            })
        }
    }
}

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

impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i32::from(*self).partial_cmp(&i32::from(*other))
    }
}

impl Op {
    /// Tests the operand for inclusion in XED_OPERAND_REG0 through XED_OPERAND_REG9.
    pub fn is_register(self) -> bool {
        unsafe { ffi::xed_operand_is_register(self.into()) }.bool()
    }

    /// Tests the operand for inclusion in XED_OPERAND_{BASE0,BASE1,INDEX,SEG0,SEG1}
    pub fn is_memory_addressing_register(self) -> bool {
        unsafe { ffi::xed_operand_is_memory_addressing_register(self.into()) }.bool()
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

mod cpuid_group;
pub use self::cpuid_group::*;

mod cpuid_rec;
pub use self::cpuid_rec::*;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Deref, From, Into)]
pub struct CpuidRecords(ffi::xed_cpuid_rec_t);

impl CpuidRecords {
    pub fn reg(self) -> Reg {
        self.0.reg.into()
    }
}

impl CpuidRec {
    /// provides the details of the CPUID specification, if the enumeration value is not sufficient.
    pub fn rec(self) -> Option<CpuidRecords> {
        let mut rec = MaybeUninit::zeroed();

        unsafe {
            ffi::xed_get_cpuid_rec(self.into(), rec.as_mut_ptr())
                .bool()
                .then(|| rec.assume_init().into())
        }
    }
}
