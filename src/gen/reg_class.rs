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
pub struct RegClass(ffi::xed_reg_class_enum_t);

impl RegClass {
    pub const INVALID: RegClass = RegClass(ffi::XED_REG_CLASS_INVALID);

    pub const BNDCFG: RegClass = RegClass(ffi::XED_REG_CLASS_BNDCFG);

    pub const BNDSTAT: RegClass = RegClass(ffi::XED_REG_CLASS_BNDSTAT);

    pub const BOUND: RegClass = RegClass(ffi::XED_REG_CLASS_BOUND);

    pub const CR: RegClass = RegClass(ffi::XED_REG_CLASS_CR);

    pub const DR: RegClass = RegClass(ffi::XED_REG_CLASS_DR);

    pub const FLAGS: RegClass = RegClass(ffi::XED_REG_CLASS_FLAGS);

    pub const GPR: RegClass = RegClass(ffi::XED_REG_CLASS_GPR);

    pub const GPR16: RegClass = RegClass(ffi::XED_REG_CLASS_GPR16);

    pub const GPR32: RegClass = RegClass(ffi::XED_REG_CLASS_GPR32);

    pub const GPR64: RegClass = RegClass(ffi::XED_REG_CLASS_GPR64);

    pub const GPR8: RegClass = RegClass(ffi::XED_REG_CLASS_GPR8);

    pub const IP: RegClass = RegClass(ffi::XED_REG_CLASS_IP);

    pub const MASK: RegClass = RegClass(ffi::XED_REG_CLASS_MASK);

    pub const MMX: RegClass = RegClass(ffi::XED_REG_CLASS_MMX);

    pub const MSR: RegClass = RegClass(ffi::XED_REG_CLASS_MSR);

    pub const MXCSR: RegClass = RegClass(ffi::XED_REG_CLASS_MXCSR);

    pub const PSEUDO: RegClass = RegClass(ffi::XED_REG_CLASS_PSEUDO);

    pub const PSEUDOX87: RegClass = RegClass(ffi::XED_REG_CLASS_PSEUDOX87);

    pub const SR: RegClass = RegClass(ffi::XED_REG_CLASS_SR);

    pub const TMP: RegClass = RegClass(ffi::XED_REG_CLASS_TMP);

    pub const TREG: RegClass = RegClass(ffi::XED_REG_CLASS_TREG);

    pub const UIF: RegClass = RegClass(ffi::XED_REG_CLASS_UIF);

    pub const X87: RegClass = RegClass(ffi::XED_REG_CLASS_X87);

    pub const XCR: RegClass = RegClass(ffi::XED_REG_CLASS_XCR);

    pub const XMM: RegClass = RegClass(ffi::XED_REG_CLASS_XMM);

    pub const YMM: RegClass = RegClass(ffi::XED_REG_CLASS_YMM);

    pub const ZMM: RegClass = RegClass(ffi::XED_REG_CLASS_ZMM);
}

impl fmt::Display for RegClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_reg_class_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for RegClass {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(RegClass(unsafe {
            ffi::str2xed_reg_class_enum_t(s.as_ptr())
        }))
    }
}
