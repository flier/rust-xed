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
pub enum Width {
    /// 16b addressing
    Word = ffi::XED_ADDRESS_WIDTH_16b,
    /// 32b addressing
    Dword = ffi::XED_ADDRESS_WIDTH_32b,
    /// 64b addressing
    Qword = ffi::XED_ADDRESS_WIDTH_64b,
}

impl fmt::Display for Width {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_address_width_enum_t2str(*self as i32))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Width {
    type Err = TryFromPrimitiveError<Width>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s).unwrap();

        Width::try_from(unsafe { ffi::str2xed_address_width_enum_t(s.as_ptr()) })
    }
}
