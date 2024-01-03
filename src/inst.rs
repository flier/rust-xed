use std::{mem, slice};

use lazy_static::lazy_static;

use crate::ffi;

/// constant information about a decoded instruction form,
/// including the pointer to the constant operand properties `Operand` for this instruction form.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Inst(ffi::xed_inst_t);

lazy_static! {
    pub static ref TABLE: &'static [Inst] = unsafe {
        mem::transmute(slice::from_raw_parts(
            ffi::xed_inst_table_base(),
            ffi::XED_MAX_INST_TABLE_NODES as usize,
        ))
    };
}
