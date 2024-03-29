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
pub struct Attribute(ffi::xed_attribute_enum_t);

impl Attribute {
    pub const INVALID: Attribute = Attribute(ffi::XED_ATTRIBUTE_INVALID);

    pub const AMDONLY: Attribute = Attribute(ffi::XED_ATTRIBUTE_AMDONLY);

    pub const APX_NDD: Attribute = Attribute(ffi::XED_ATTRIBUTE_APX_NDD);

    pub const APX_NF: Attribute = Attribute(ffi::XED_ATTRIBUTE_APX_NF);

    pub const ATOMIC: Attribute = Attribute(ffi::XED_ATTRIBUTE_ATOMIC);

    pub const ATT_OPERAND_ORDER_EXCEPTION: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_ATT_OPERAND_ORDER_EXCEPTION);

    pub const BROADCAST_ENABLED: Attribute = Attribute(ffi::XED_ATTRIBUTE_BROADCAST_ENABLED);

    pub const BYTEOP: Attribute = Attribute(ffi::XED_ATTRIBUTE_BYTEOP);

    pub const DISP8_EIGHTHMEM: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_EIGHTHMEM);

    pub const DISP8_FULL: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_FULL);

    pub const DISP8_FULLMEM: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_FULLMEM);

    pub const DISP8_GPR_READER: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_READER);

    pub const DISP8_GPR_READER_BYTE: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_READER_BYTE);

    pub const DISP8_GPR_READER_WORD: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_READER_WORD);

    pub const DISP8_GPR_WRITER_LDOP_D: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_WRITER_LDOP_D);

    pub const DISP8_GPR_WRITER_LDOP_Q: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_WRITER_LDOP_Q);

    pub const DISP8_GPR_WRITER_STORE: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE);

    pub const DISP8_GPR_WRITER_STORE_BYTE: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE_BYTE);

    pub const DISP8_GPR_WRITER_STORE_WORD: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE_WORD);

    pub const DISP8_GSCAT: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_GSCAT);

    pub const DISP8_HALF: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_HALF);

    pub const DISP8_HALFMEM: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_HALFMEM);

    pub const DISP8_MEM128: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_MEM128);

    pub const DISP8_MOVDDUP: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_MOVDDUP);

    pub const DISP8_NO_SCALE: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_NO_SCALE);

    pub const DISP8_QUARTER: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_QUARTER);

    pub const DISP8_QUARTERMEM: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_QUARTERMEM);

    pub const DISP8_SCALAR: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_SCALAR);

    pub const DISP8_TUPLE1: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE1);

    pub const DISP8_TUPLE1_4X: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE1_4X);

    pub const DISP8_TUPLE1_BYTE: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE1_BYTE);

    pub const DISP8_TUPLE1_WORD: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE1_WORD);

    pub const DISP8_TUPLE2: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE2);

    pub const DISP8_TUPLE4: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE4);

    pub const DISP8_TUPLE8: Attribute = Attribute(ffi::XED_ATTRIBUTE_DISP8_TUPLE8);

    pub const DOUBLE_WIDE_MEMOP: Attribute = Attribute(ffi::XED_ATTRIBUTE_DOUBLE_WIDE_MEMOP);

    pub const DOUBLE_WIDE_OUTPUT: Attribute = Attribute(ffi::XED_ATTRIBUTE_DOUBLE_WIDE_OUTPUT);

    pub const DWORD_INDICES: Attribute = Attribute(ffi::XED_ATTRIBUTE_DWORD_INDICES);

    pub const ELEMENT_SIZE_D: Attribute = Attribute(ffi::XED_ATTRIBUTE_ELEMENT_SIZE_D);

    pub const ELEMENT_SIZE_Q: Attribute = Attribute(ffi::XED_ATTRIBUTE_ELEMENT_SIZE_Q);

    pub const EXCEPTION_BR: Attribute = Attribute(ffi::XED_ATTRIBUTE_EXCEPTION_BR);

    pub const FAR_XFER: Attribute = Attribute(ffi::XED_ATTRIBUTE_FAR_XFER);

    pub const FIXED_BASE0: Attribute = Attribute(ffi::XED_ATTRIBUTE_FIXED_BASE0);

    pub const FIXED_BASE1: Attribute = Attribute(ffi::XED_ATTRIBUTE_FIXED_BASE1);

    pub const FIXED_ROUNDING_RNE: Attribute = Attribute(ffi::XED_ATTRIBUTE_FIXED_ROUNDING_RNE);

    pub const FLUSH_INPUT_DENORM: Attribute = Attribute(ffi::XED_ATTRIBUTE_FLUSH_INPUT_DENORM);

    pub const FLUSH_OUTPUT_DENORM: Attribute = Attribute(ffi::XED_ATTRIBUTE_FLUSH_OUTPUT_DENORM);

    pub const GATHER: Attribute = Attribute(ffi::XED_ATTRIBUTE_GATHER);

    pub const HALF_WIDE_OUTPUT: Attribute = Attribute(ffi::XED_ATTRIBUTE_HALF_WIDE_OUTPUT);

    pub const HLE_ACQ_ABLE: Attribute = Attribute(ffi::XED_ATTRIBUTE_HLE_ACQ_ABLE);

    pub const HLE_REL_ABLE: Attribute = Attribute(ffi::XED_ATTRIBUTE_HLE_REL_ABLE);

    pub const IGNORES_OSFXSR: Attribute = Attribute(ffi::XED_ATTRIBUTE_IGNORES_OSFXSR);

    pub const IMPLICIT_ONE: Attribute = Attribute(ffi::XED_ATTRIBUTE_IMPLICIT_ONE);

    pub const INDEX_REG_IS_POINTER: Attribute = Attribute(ffi::XED_ATTRIBUTE_INDEX_REG_IS_POINTER);

    pub const INDIRECT_BRANCH: Attribute = Attribute(ffi::XED_ATTRIBUTE_INDIRECT_BRANCH);

    pub const KMASK: Attribute = Attribute(ffi::XED_ATTRIBUTE_KMASK);

    pub const LOCKABLE: Attribute = Attribute(ffi::XED_ATTRIBUTE_LOCKABLE);

    pub const LOCKED: Attribute = Attribute(ffi::XED_ATTRIBUTE_LOCKED);

    pub const MASKOP: Attribute = Attribute(ffi::XED_ATTRIBUTE_MASKOP);

    pub const MASKOP_EVEX: Attribute = Attribute(ffi::XED_ATTRIBUTE_MASKOP_EVEX);

    pub const MASK_AS_CONTROL: Attribute = Attribute(ffi::XED_ATTRIBUTE_MASK_AS_CONTROL);

    pub const MASK_VARIABLE_MEMOP: Attribute = Attribute(ffi::XED_ATTRIBUTE_MASK_VARIABLE_MEMOP);

    pub const MEMORY_FAULT_SUPPRESSION: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_MEMORY_FAULT_SUPPRESSION);

    pub const MMX_EXCEPT: Attribute = Attribute(ffi::XED_ATTRIBUTE_MMX_EXCEPT);

    pub const MPX_PREFIX_ABLE: Attribute = Attribute(ffi::XED_ATTRIBUTE_MPX_PREFIX_ABLE);

    pub const MULTIDEST2: Attribute = Attribute(ffi::XED_ATTRIBUTE_MULTIDEST2);

    pub const MULTISOURCE4: Attribute = Attribute(ffi::XED_ATTRIBUTE_MULTISOURCE4);

    pub const MXCSR: Attribute = Attribute(ffi::XED_ATTRIBUTE_MXCSR);

    pub const MXCSR_RD: Attribute = Attribute(ffi::XED_ATTRIBUTE_MXCSR_RD);

    pub const NONTEMPORAL: Attribute = Attribute(ffi::XED_ATTRIBUTE_NONTEMPORAL);

    pub const NOP: Attribute = Attribute(ffi::XED_ATTRIBUTE_NOP);

    pub const NOTSX: Attribute = Attribute(ffi::XED_ATTRIBUTE_NOTSX);

    pub const NOTSX_COND: Attribute = Attribute(ffi::XED_ATTRIBUTE_NOTSX_COND);

    pub const NO_RIP_REL: Attribute = Attribute(ffi::XED_ATTRIBUTE_NO_RIP_REL);

    pub const NO_SRC_DEST_MATCH: Attribute = Attribute(ffi::XED_ATTRIBUTE_NO_SRC_DEST_MATCH);

    pub const PREFETCH: Attribute = Attribute(ffi::XED_ATTRIBUTE_PREFETCH);

    pub const PROTECTED_MODE: Attribute = Attribute(ffi::XED_ATTRIBUTE_PROTECTED_MODE);

    pub const QWORD_INDICES: Attribute = Attribute(ffi::XED_ATTRIBUTE_QWORD_INDICES);

    pub const REP: Attribute = Attribute(ffi::XED_ATTRIBUTE_REP);

    pub const REQUIRES_ALIGNMENT: Attribute = Attribute(ffi::XED_ATTRIBUTE_REQUIRES_ALIGNMENT);

    pub const REQUIRES_ALIGNMENT_4B: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_REQUIRES_ALIGNMENT_4B);

    pub const REQUIRES_ALIGNMENT_8B: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_REQUIRES_ALIGNMENT_8B);

    pub const RING0: Attribute = Attribute(ffi::XED_ATTRIBUTE_RING0);

    pub const SCALABLE: Attribute = Attribute(ffi::XED_ATTRIBUTE_SCALABLE);

    pub const SCATTER: Attribute = Attribute(ffi::XED_ATTRIBUTE_SCATTER);

    pub const SIMD_SCALAR: Attribute = Attribute(ffi::XED_ATTRIBUTE_SIMD_SCALAR);

    pub const SKIPLOW32: Attribute = Attribute(ffi::XED_ATTRIBUTE_SKIPLOW32);

    pub const SKIPLOW64: Attribute = Attribute(ffi::XED_ATTRIBUTE_SKIPLOW64);

    pub const SPECIAL_AGEN_REQUIRED: Attribute =
        Attribute(ffi::XED_ATTRIBUTE_SPECIAL_AGEN_REQUIRED);

    pub const STACKPOP0: Attribute = Attribute(ffi::XED_ATTRIBUTE_STACKPOP0);

    pub const STACKPOP1: Attribute = Attribute(ffi::XED_ATTRIBUTE_STACKPOP1);

    pub const STACKPUSH0: Attribute = Attribute(ffi::XED_ATTRIBUTE_STACKPUSH0);

    pub const STACKPUSH1: Attribute = Attribute(ffi::XED_ATTRIBUTE_STACKPUSH1);

    pub const UNDOCUMENTED: Attribute = Attribute(ffi::XED_ATTRIBUTE_UNDOCUMENTED);

    pub const USES_DAZ: Attribute = Attribute(ffi::XED_ATTRIBUTE_USES_DAZ);

    pub const USES_FTZ: Attribute = Attribute(ffi::XED_ATTRIBUTE_USES_FTZ);

    pub const X87_CONTROL: Attribute = Attribute(ffi::XED_ATTRIBUTE_X87_CONTROL);

    pub const X87_MMX_STATE_CW: Attribute = Attribute(ffi::XED_ATTRIBUTE_X87_MMX_STATE_CW);

    pub const X87_MMX_STATE_R: Attribute = Attribute(ffi::XED_ATTRIBUTE_X87_MMX_STATE_R);

    pub const X87_MMX_STATE_W: Attribute = Attribute(ffi::XED_ATTRIBUTE_X87_MMX_STATE_W);

    pub const X87_NOWAIT: Attribute = Attribute(ffi::XED_ATTRIBUTE_X87_NOWAIT);

    pub const XMM_STATE_CW: Attribute = Attribute(ffi::XED_ATTRIBUTE_XMM_STATE_CW);

    pub const XMM_STATE_R: Attribute = Attribute(ffi::XED_ATTRIBUTE_XMM_STATE_R);

    pub const XMM_STATE_W: Attribute = Attribute(ffi::XED_ATTRIBUTE_XMM_STATE_W);
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_attribute_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Attribute {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(Attribute(unsafe {
            ffi::str2xed_attribute_enum_t(s.as_ptr())
        }))
    }
}
