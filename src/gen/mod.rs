use std::mem::MaybeUninit;

use derive_more::Deref;
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

mod cpuid_bit;
pub use self::cpuid_bit::CpuidBit;

impl CpuidBit {
    /// This provides the details of the CPUID bit specification, if the enumeration value is not sufficient.  
    pub fn rec(self) -> Option<CpuidRec> {
        let mut rec = MaybeUninit::zeroed();

        unsafe {
            if ffi::xed_get_cpuid_rec(self.into(), rec.as_mut_ptr()) != 0 {
                Some(CpuidRec(rec.assume_init()))
            } else {
                None
            }
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Deref)]
pub struct CpuidRec(ffi::xed_cpuid_rec_t);

impl CpuidRec {
    /// the register containing the bit (EAX,EBX,ECX,EDX)
    pub fn reg(&self) -> Reg {
        self.0.reg.into()
    }
}

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

impl Iclass {
    enum_properties! {
        iform_max: u32 { xed_iform_max_per_iclass }
        iform_first: u32 { xed_iform_first_per_iclass }

        /// Take an instruction with a REP/REPE/REPNE prefix and return the corresponding xed_iclass_enum_t without that prefix.
        ///
        /// The return value differs from the other functions in this group:
        /// If the input iclass does not have REP/REPNE/REPE prefix, the function returns the original instruction.
        rep_remove: Iclass { xed_rep_remove }

        /// Take an `Iclass` value without a REPE prefix and return the corresponding #xed_iclass_enum_t with a REPE prefix.
        ///
        /// If the input instruction cannot have have a REPE prefix, this function returns None.
        repe: Iclass? { xed_repe_map }

        /// Take an `Iclass` value without a REPNE prefix and return the corresponding `Iclass` with a REPNE prefix.
        ///
        /// If the input instruction cannot have a REPNE prefix, this function returns None.
        repne: Iclass? { xed_repne_map }

        /// Take an `Iclass` value without a REP prefix and return the corresponding `Iclass` with a REP prefix.
        ///
        /// If the input instruction cannot have a REP prefix, this function returns None.
        rep: Iclass? { xed_rep_map }

        /// Take an `Iclass` value for an instruction with a REP/REPNE/REPE prefix and return the corresponding `Iclass` without that prefix.
        ///
        /// If the input instruction does not have a REP/REPNE/REPE prefix, this function returns None.
        norep: Iclass? { xed_norep_map }
    }
}

mod iform;
pub use self::iform::*;

mod iformfl;
pub use self::iformfl::*;

mod isa_set;
pub use self::isa_set::IsaSet;

impl IsaSet {
    /// Returns the name of the i'th cpuid bit associated with this isa-set.
    pub fn cpuid_bits(self) -> impl Iterator<Item = CpuidBit> {
        unsafe {
            (0..ffi::XED_MAX_CPUID_BITS_PER_ISA_SET).flat_map(move |i| {
                let bit = ffi::xed_get_cpuid_bit_for_isa_set(self.into(), i);

                (bit != 0).then(|| bit.into())
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
