//! automatically generated by enum_gen.py, DON't EDIT IT

use std::{
    ffi::{CStr, CString, NulError},
    fmt,
    str::FromStr,
};

use derive_more::{From, Into};

use crate::ffi;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, From, Into)]
pub struct OperandType(ffi::xed_operand_type_enum_t);

impl OperandType {
    pub const INVALID: OperandType = OperandType(ffi::XED_OPERAND_TYPE_INVALID);

    pub const ERROR: OperandType = OperandType(ffi::XED_OPERAND_TYPE_ERROR);

    pub const IMM: OperandType = OperandType(ffi::XED_OPERAND_TYPE_IMM);

    pub const IMM_CONST: OperandType = OperandType(ffi::XED_OPERAND_TYPE_IMM_CONST);

    pub const NT_LOOKUP_FN: OperandType = OperandType(ffi::XED_OPERAND_TYPE_NT_LOOKUP_FN);

    pub const NT_LOOKUP_FN2: OperandType = OperandType(ffi::XED_OPERAND_TYPE_NT_LOOKUP_FN2);

    pub const NT_LOOKUP_FN4: OperandType = OperandType(ffi::XED_OPERAND_TYPE_NT_LOOKUP_FN4);

    pub const REG: OperandType = OperandType(ffi::XED_OPERAND_TYPE_REG);
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_operand_type_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for OperandType {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(OperandType(unsafe {
            ffi::str2xed_operand_type_enum_t(s.as_ptr())
        }))
    }
}