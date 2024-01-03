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
pub struct OperandAction(ffi::xed_operand_action_enum_t);

impl OperandAction {
    pub const INVALID: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_INVALID);

    /// Read and written (must write)
    pub const RW: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_RW);

    /// Read-only
    pub const R: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_R);

    /// Write-only (must write)
    pub const W: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_W);

    /// Read and conditionlly written (may write)
    pub const RCW: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_RCW);

    /// Conditionlly written (may write)
    pub const CW: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_CW);

    /// Conditionlly read, always written (must write)
    pub const CRW: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_CRW);

    /// Conditional read
    pub const CR: OperandAction = OperandAction(ffi::XED_OPERAND_ACTION_CR);
}

impl fmt::Display for OperandAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_operand_action_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for OperandAction {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(OperandAction(unsafe {
            ffi::str2xed_operand_action_enum_t(s.as_ptr())
        }))
    }
}