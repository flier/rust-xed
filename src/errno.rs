use std::{
    error::Error,
    ffi::{CStr, CString},
    fmt,
    str::FromStr,
};

use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};

use crate::ffi;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Errno {
    ///< There was no error
    None = ffi::XED_ERROR_NONE,
    ///< There were not enough bytes in the given buffer
    BufferTooShort = ffi::XED_ERROR_BUFFER_TOO_SHORT,
    ///< XED could not decode the given instruction
    GeneralError = ffi::XED_ERROR_GENERAL_ERROR,
    ///< The instruciton is not valid for the specified chip
    InvalidForChip = ffi::XED_ERROR_INVALID_FOR_CHIP,
    ///< XED could not decode the given instruction because an invalid register encoding was used.
    BadRegister = ffi::XED_ERROR_BAD_REGISTER,
    ///< A lock prefix was found where none is allowed.
    BadLockPrefix = ffi::XED_ERROR_BAD_LOCK_PREFIX,
    ///< An F2 or F3 prefix was found where none is allowed.
    BadRepPrefix = ffi::XED_ERROR_BAD_REP_PREFIX,
    ///< A 66, F2 or F3 prefix was found where none is allowed.
    BadLegacyPrefix = ffi::XED_ERROR_BAD_LEGACY_PREFIX,
    ///< A REX prefix was found where none is allowed.
    BadRexPrefix = ffi::XED_ERROR_BAD_REX_PREFIX,
    ///< An illegal value for the EVEX.U bit was present in the instruction.
    BadEvexUbit = ffi::XED_ERROR_BAD_EVEX_UBIT,
    ///< An illegal value for the MAP field was detected in the instruction.
    BadMap = ffi::XED_ERROR_BAD_MAP,
    ///< EVEX.V'=0 was detected in a non-64b mode instruction.
    BadEvexVPrime = ffi::XED_ERROR_BAD_EVEX_V_PRIME,
    ///< EVEX.Z!=0 when EVEX.aaa==0
    BadEvexZNoMasking = ffi::XED_ERROR_BAD_EVEX_Z_NO_MASKING,
    ///< The output pointer for xed_agen was zero
    NoOutputPointer = ffi::XED_ERROR_NO_OUTPUT_POINTER,
    ///< One or both of the callbacks for xed_agen were missing.
    NoAgenCallBackRegistered = ffi::XED_ERROR_NO_AGEN_CALL_BACK_REGISTERED,
    ///< Memop indices must be 0 or 1.
    BadMemopIndex = ffi::XED_ERROR_BAD_MEMOP_INDEX,
    ///< The register or segment callback for xed_agen experienced a problem
    CallbackProblem = ffi::XED_ERROR_CALLBACK_PROBLEM,
    ///< The index, dest and mask regs for AVX2 gathers must be different.
    GatherRegs = ffi::XED_ERROR_GATHER_REGS,
    ///< Full decode of instruction would exeed 15B.
    InstrTooLong = ffi::XED_ERROR_INSTR_TOO_LONG,
    ///< The instruction was not valid for the specified mode
    InvalidMode = ffi::XED_ERROR_INVALID_MODE,
    ///< EVEX.LL must not ==3 unless using embedded rounding
    BadEvexLl = ffi::XED_ERROR_BAD_EVEX_LL,
    ///< Source registers must not match the destination register for this instruction.
    BadRegMatch = ffi::XED_ERROR_BAD_REG_MATCH,
}

impl Errno {
    pub fn is_none(self) -> bool {
        self == Self::None
    }
}

impl Error for Errno {}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_error_enum_t2str(*self as i32))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Errno {
    type Err = TryFromPrimitiveError<Errno>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s).unwrap();

        Errno::try_from(unsafe { ffi::str2xed_error_enum_t(s.as_ptr()) })
    }
}
