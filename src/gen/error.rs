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
pub struct Error(ffi::xed_error_enum_t);

impl Error {
    /// There was no error
    pub const NONE: Error = Error(ffi::XED_ERROR_NONE);

    /// There were not enough bytes in the given buffer
    pub const BUFFER_TOO_SHORT: Error = Error(ffi::XED_ERROR_BUFFER_TOO_SHORT);

    /// XED could not decode the given instruction
    pub const GENERAL_ERROR: Error = Error(ffi::XED_ERROR_GENERAL_ERROR);

    /// The instruciton is not valid for the specified chip
    pub const INVALID_FOR_CHIP: Error = Error(ffi::XED_ERROR_INVALID_FOR_CHIP);

    /// XED could not decode the given instruction because an invalid register encoding was used.
    pub const BAD_REGISTER: Error = Error(ffi::XED_ERROR_BAD_REGISTER);

    /// A lock prefix was found where none is allowed.
    pub const BAD_LOCK_PREFIX: Error = Error(ffi::XED_ERROR_BAD_LOCK_PREFIX);

    /// An F2 or F3 prefix was found where none is allowed.
    pub const BAD_REP_PREFIX: Error = Error(ffi::XED_ERROR_BAD_REP_PREFIX);

    /// A 66, F2 or F3 prefix was found where none is allowed.
    pub const BAD_LEGACY_PREFIX: Error = Error(ffi::XED_ERROR_BAD_LEGACY_PREFIX);

    /// A REX prefix was found where none is allowed.
    pub const BAD_REX_PREFIX: Error = Error(ffi::XED_ERROR_BAD_REX_PREFIX);

    /// An illegal value for the EVEX.U bit was present in the instruction.
    pub const BAD_EVEX_UBIT: Error = Error(ffi::XED_ERROR_BAD_EVEX_UBIT);

    /// An illegal value for the MAP field was detected in the instruction.
    pub const BAD_MAP: Error = Error(ffi::XED_ERROR_BAD_MAP);

    /// EVEX.V'=0 was detected in a non-64b mode instruction.
    pub const BAD_EVEX_V_PRIME: Error = Error(ffi::XED_ERROR_BAD_EVEX_V_PRIME);

    /// EVEX.Z!=0 when EVEX.aaa==0
    pub const BAD_EVEX_Z_NO_MASKING: Error = Error(ffi::XED_ERROR_BAD_EVEX_Z_NO_MASKING);

    /// The output pointer for xed_agen was zero
    pub const NO_OUTPUT_POINTER: Error = Error(ffi::XED_ERROR_NO_OUTPUT_POINTER);

    /// One or both of the callbacks for xed_agen were missing.
    pub const NO_AGEN_CALL_BACK_REGISTERED: Error =
        Error(ffi::XED_ERROR_NO_AGEN_CALL_BACK_REGISTERED);

    /// Memop indices must be 0 or 1.
    pub const BAD_MEMOP_INDEX: Error = Error(ffi::XED_ERROR_BAD_MEMOP_INDEX);

    /// The register or segment callback for xed_agen experienced a problem
    pub const CALLBACK_PROBLEM: Error = Error(ffi::XED_ERROR_CALLBACK_PROBLEM);

    /// The index, dest and mask regs for AVX2 gathers must be different.
    pub const GATHER_REGS: Error = Error(ffi::XED_ERROR_GATHER_REGS);

    /// Full decode of instruction would exeed 15B.
    pub const INSTR_TOO_LONG: Error = Error(ffi::XED_ERROR_INSTR_TOO_LONG);

    /// The instruction was not valid for the specified mode
    pub const INVALID_MODE: Error = Error(ffi::XED_ERROR_INVALID_MODE);

    /// EVEX.LL must not ==3 unless using embedded rounding
    pub const BAD_EVEX_LL: Error = Error(ffi::XED_ERROR_BAD_EVEX_LL);

    /// Source registers must not match the destination register for this instruction.
    pub const BAD_REG_MATCH: Error = Error(ffi::XED_ERROR_BAD_REG_MATCH);
}

impl Error {
    pub fn is_none(self) -> bool {
        self == Self::NONE
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_error_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Error {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(Error(unsafe { ffi::str2xed_error_enum_t(s.as_ptr()) }))
    }
}
