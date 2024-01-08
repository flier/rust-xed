use std::{mem, slice};

use lazy_static::lazy_static;

use crate::{
    ffi, properties,
    raw::{AsPtr, ToBool},
    Attribute, Category, Exception, Extension, Iclass, Iform, IsaSet, Operand,
};

/// constant information about a decoded instruction form,
/// including the pointer to the constant operand properties `Operand` for this instruction form.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Inst(ffi::xed_inst_t);

impl_as_ptr!(Inst(ffi::xed_inst_t));

impl Inst {
    properties! {
        iform: Iform { xed_inst_iform_enum }
        iclass: Iclass { xed_inst_iclass }
        category: Category { xed_inst_category }
        extension: Extension { xed_inst_extension }
        isa_set: IsaSet { xed_inst_isa_set }

        /// Return #xed_exception_enum_t if present for the specified instruction.
        ///
        /// This is currently only used for SSE and AVX instructions.
        exception: Exception? { xed_inst_exception }
    }

    pub fn tables() -> &'static [Inst] {
        &TABLE
    }

    pub fn has_attr(&self, attr: Attribute) -> bool {
        unsafe { ffi::xed_inst_get_attribute(self.as_ptr(), attr.into()) }.bool()
    }

    pub fn attrs(&self) -> impl Iterator<Item = Attribute> + '_ {
        Attribute::tables()
            .iter()
            .filter(|&&attr| self.has_attr(attr))
            .cloned()
    }

    pub fn operand(&self, i: u32) -> Option<&Operand> {
        unsafe {
            if i < ffi::xed_inst_noperands(self.as_ptr()) {
                ffi::xed_inst_operand(self.as_ptr(), i)
                    .cast::<Operand>()
                    .as_ref()
            } else {
                None
            }
        }
    }

    pub fn operands(&self) -> impl Iterator<Item = &Operand> {
        unsafe {
            (0..ffi::xed_inst_noperands(self.as_ptr())).flat_map(|i| {
                ffi::xed_inst_operand(self.as_ptr(), i)
                    .cast::<Operand>()
                    .as_ref()
            })
        }
    }
}

lazy_static! {
    static ref TABLE: &'static [Inst] = unsafe {
        mem::transmute(slice::from_raw_parts(
            ffi::xed_inst_table_base(),
            ffi::XED_MAX_INST_TABLE_NODES as usize,
        ))
    };
}
