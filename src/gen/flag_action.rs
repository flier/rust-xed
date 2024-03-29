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
pub struct FlagAction(ffi::xed_flag_action_enum_t);

impl FlagAction {
    pub const INVALID: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_INVALID);

    /// undefined (treated as a write)
    pub const U: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_u);

    /// test (read)
    pub const TST: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_tst);

    /// modification (write)
    pub const MOD: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_mod);

    /// value will be zero (write)
    pub const _0: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_0);

    /// value comes from the stack (write)
    pub const POP: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_pop);

    /// value comes from AH (write)
    pub const AH: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_ah);

    /// value will be 1 (write)
    pub const _1: FlagAction = FlagAction(ffi::XED_FLAG_ACTION_1);
}

impl fmt::Display for FlagAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_flag_action_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for FlagAction {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(FlagAction(unsafe {
            ffi::str2xed_flag_action_enum_t(s.as_ptr())
        }))
    }
}
