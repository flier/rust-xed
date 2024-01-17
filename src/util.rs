use std::ffi::CStr;

use crate::ffi;

pub trait SignExtend {
    type Signed;

    /// arbitrary sign extension from a qty of "bits" length
    fn arbitrary(self, bits: u32) -> Self::Signed;
}

impl SignExtend for u64 {
    type Signed = i64;

    fn arbitrary(self, bits: u32) -> Self::Signed {
        unsafe { ffi::xed_sign_extend_arbitrary_to_64(self, bits) }
    }
}

impl SignExtend for u32 {
    type Signed = i32;

    fn arbitrary(self, bits: u32) -> Self::Signed {
        unsafe { ffi::xed_sign_extend_arbitrary_to_32(self, bits) }
    }
}

/// Set the verbosity level for XED
pub fn set_verbosity(level: i32) {
    unsafe { ffi::xed_set_verbosity(level) }
}

/// Returns a string representing XED svn commit revision and time stamp.
pub fn version() -> &'static str {
    unsafe { CStr::from_ptr(ffi::xed_get_version()).to_str().unwrap() }
}

/// Returns a copyright string.
pub fn copyright() -> &'static str {
    unsafe { CStr::from_ptr(ffi::xed_get_copyright()).to_str().unwrap() }
}
