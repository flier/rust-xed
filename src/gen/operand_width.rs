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
pub struct OperandWidth(ffi::xed_operand_width_enum_t);

impl OperandWidth {
    pub const INVALID: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_INVALID);

    pub const ASZ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ASZ);

    pub const SSZ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SSZ);

    pub const PSEUDO: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PSEUDO);

    pub const PSEUDOX87: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PSEUDOX87);

    pub const A16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_A16);

    pub const A32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_A32);

    pub const B: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_B);

    pub const D: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_D);

    pub const I8: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I8);

    pub const U8: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_U8);

    pub const I16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I16);

    pub const U16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_U16);

    pub const I32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I32);

    pub const U32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_U32);

    pub const I64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I64);

    pub const U64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_U64);

    pub const F16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_F16);

    pub const F32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_F32);

    pub const F64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_F64);

    pub const DQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_DQ);

    pub const XUB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XUB);

    pub const XUW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XUW);

    pub const XUD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XUD);

    pub const XUQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XUQ);

    pub const X128: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_X128);

    pub const XB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XB);

    pub const XW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XW);

    pub const XD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XD);

    pub const XQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_XQ);

    pub const ZB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZB);

    pub const ZW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZW);

    pub const ZD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZD);

    pub const ZQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZQ);

    pub const MB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MB);

    pub const MW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MW);

    pub const MD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MD);

    pub const MQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MQ);

    pub const M64INT: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_M64INT);

    pub const M64REAL: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_M64REAL);

    pub const MEM108: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM108);

    pub const MEM14: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM14);

    pub const MEM16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM16);

    pub const MEM16INT: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM16INT);

    pub const MEM28: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM28);

    pub const MEM32INT: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM32INT);

    pub const MEM32REAL: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM32REAL);

    pub const MEM80DEC: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM80DEC);

    pub const MEM80REAL: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM80REAL);

    pub const F80: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_F80);

    pub const MEM94: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MEM94);

    pub const MFPXENV: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MFPXENV);

    pub const MXSAVE: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MXSAVE);

    pub const MPREFETCH: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MPREFETCH);

    pub const P: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_P);

    pub const P2: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_P2);

    pub const PD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PD);

    pub const PS: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PS);

    pub const PI: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PI);

    pub const Q: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_Q);

    pub const S: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_S);

    pub const S64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_S64);

    pub const SD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SD);

    pub const SI: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SI);

    pub const SS: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SS);

    pub const V: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_V);

    pub const Y: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_Y);

    pub const W: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_W);

    pub const Z: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_Z);

    pub const SPW8: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SPW8);

    pub const SPW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SPW);

    pub const SPW5: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SPW5);

    pub const SPW3: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SPW3);

    pub const SPW2: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_SPW2);

    pub const I1: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I1);

    pub const I2: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I2);

    pub const I3: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I3);

    pub const I4: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I4);

    pub const I5: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I5);

    pub const I6: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I6);

    pub const I7: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_I7);

    pub const VAR: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_VAR);

    pub const BND32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_BND32);

    pub const BND64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_BND64);

    pub const PMMSZ16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PMMSZ16);

    pub const PMMSZ32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PMMSZ32);

    pub const QQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_QQ);

    pub const YUB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YUB);

    pub const YUW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YUW);

    pub const YUD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YUD);

    pub const YUQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YUQ);

    pub const Y128: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_Y128);

    pub const YB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YB);

    pub const YW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YW);

    pub const YD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YD);

    pub const YQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YQ);

    pub const YPS: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YPS);

    pub const YPD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_YPD);

    pub const ZBF16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZBF16);

    pub const VV: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_VV);

    pub const ZV: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZV);

    pub const WRD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_WRD);

    pub const MSKW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_MSKW);

    pub const ZMSKW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZMSKW);

    pub const ZF32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZF32);

    pub const ZF64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZF64);

    pub const ZUB: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZUB);

    pub const ZUW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZUW);

    pub const ZUD: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZUD);

    pub const ZUQ: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZUQ);

    pub const ZI8: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZI8);

    pub const ZI16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZI16);

    pub const ZI32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZI32);

    pub const ZI64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZI64);

    pub const ZU8: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZU8);

    pub const ZU16: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZU16);

    pub const ZU32: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZU32);

    pub const ZU64: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZU64);

    pub const ZU128: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_ZU128);

    pub const M384: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_M384);

    pub const M512: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_M512);

    pub const PTR: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_PTR);

    pub const TMEMROW: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_TMEMROW);

    pub const TMEMCOL: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_TMEMCOL);

    pub const TV: OperandWidth = OperandWidth(ffi::XED_OPERAND_WIDTH_TV);
}

impl fmt::Display for OperandWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_operand_width_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for OperandWidth {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(OperandWidth(unsafe {
            ffi::str2xed_operand_width_enum_t(s.as_ptr())
        }))
    }
}