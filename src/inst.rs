use std::{mem, slice};

use lazy_static::lazy_static;

use crate::{ffi, Attribute, Category, Extension, Iclass, Iform, IsaSet, Operand};

/// constant information about a decoded instruction form,
/// including the pointer to the constant operand properties `Operand` for this instruction form.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Inst(ffi::xed_inst_t);

impl Inst {
    pub fn tables() -> &'static [Inst] {
        &*TABLE
    }

    fn as_ptr(&self) -> *const ffi::xed_inst_t {
        &self.0 as *const _
    }

    pub fn iclass(&self) -> Iclass {
        unsafe { ffi::xed_inst_iclass(self.as_ptr()) }.into()
    }

    pub fn iform(&self) -> Iform {
        unsafe { ffi::xed_inst_iform_enum(self.as_ptr()) }.into()
    }

    pub fn category(&self) -> Category {
        unsafe { ffi::xed_inst_category(self.as_ptr()) }.into()
    }

    pub fn extension(&self) -> Extension {
        unsafe { ffi::xed_inst_extension(self.as_ptr()) }.into()
    }

    pub fn isa_set(&self) -> IsaSet {
        unsafe { ffi::xed_inst_isa_set(self.as_ptr()) }.into()
    }

    pub fn attrs(&self) -> Vec<Attribute> {
        Attribute::tables()
            .iter()
            .filter(|&&attr| unsafe {
                ffi::xed_inst_get_attribute(self.as_ptr(), attr.into()) != 0
            })
            .cloned()
            .collect()
    }

    pub fn operands(&self) -> Vec<&Operand> {
        unsafe {
            (0..ffi::xed_inst_noperands(self.as_ptr()))
                .map(|i| {
                    ffi::xed_inst_operand(self.as_ptr(), i)
                        .cast::<Operand>()
                        .as_ref()
                        .unwrap()
                })
                .collect()
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
