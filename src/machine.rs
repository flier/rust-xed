use std::{
    ffi::{CStr, CString},
    fmt,
    str::FromStr,
};

use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};

use crate::ffi;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Mode {
    /// 64b operating mode
    Long64 = ffi::XED_MACHINE_MODE_LONG_64,
    /// 32b protected mode
    LongCompat32 = ffi::XED_MACHINE_MODE_LONG_COMPAT_32,
    /// 16b protected mode
    LongCompat16 = ffi::XED_MACHINE_MODE_LONG_COMPAT_16,
    /// 32b protected mode
    Legacy32 = ffi::XED_MACHINE_MODE_LEGACY_32,
    /// 16b protected mode
    Legacy16 = ffi::XED_MACHINE_MODE_LEGACY_16,
    /// 16b real mode
    Real16 = ffi::XED_MACHINE_MODE_REAL_16,
    /// 32b real mode (CS.D bit = 1)
    Real32 = ffi::XED_MACHINE_MODE_REAL_32,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_address_width_enum_t2str(*self as i32))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Mode {
    type Err = TryFromPrimitiveError<Mode>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s).unwrap();

        Mode::try_from(unsafe { ffi::str2xed_address_width_enum_t(s.as_ptr()) })
    }
}
