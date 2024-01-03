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
pub struct Operand(ffi::xed_operand_enum_t);

impl Operand {
    pub const INVALID: Operand = Operand(ffi::XED_OPERAND_INVALID);

    pub const AGEN: Operand = Operand(ffi::XED_OPERAND_AGEN);

    pub const AMD3DNOW: Operand = Operand(ffi::XED_OPERAND_AMD3DNOW);

    pub const ASZ: Operand = Operand(ffi::XED_OPERAND_ASZ);

    pub const BASE0: Operand = Operand(ffi::XED_OPERAND_BASE0);

    pub const BASE1: Operand = Operand(ffi::XED_OPERAND_BASE1);

    pub const BCAST: Operand = Operand(ffi::XED_OPERAND_BCAST);

    pub const BCRC: Operand = Operand(ffi::XED_OPERAND_BCRC);

    pub const BRDISP_WIDTH: Operand = Operand(ffi::XED_OPERAND_BRDISP_WIDTH);

    pub const CET: Operand = Operand(ffi::XED_OPERAND_CET);

    pub const CHIP: Operand = Operand(ffi::XED_OPERAND_CHIP);

    pub const CLDEMOTE: Operand = Operand(ffi::XED_OPERAND_CLDEMOTE);

    pub const DEFAULT_SEG: Operand = Operand(ffi::XED_OPERAND_DEFAULT_SEG);

    pub const DF32: Operand = Operand(ffi::XED_OPERAND_DF32);

    pub const DF64: Operand = Operand(ffi::XED_OPERAND_DF64);

    pub const DISP: Operand = Operand(ffi::XED_OPERAND_DISP);

    pub const DISP_WIDTH: Operand = Operand(ffi::XED_OPERAND_DISP_WIDTH);

    pub const DUMMY: Operand = Operand(ffi::XED_OPERAND_DUMMY);

    pub const EASZ: Operand = Operand(ffi::XED_OPERAND_EASZ);

    pub const ELEMENT_SIZE: Operand = Operand(ffi::XED_OPERAND_ELEMENT_SIZE);

    pub const ENCODER_PREFERRED: Operand = Operand(ffi::XED_OPERAND_ENCODER_PREFERRED);

    pub const ENCODE_FORCE: Operand = Operand(ffi::XED_OPERAND_ENCODE_FORCE);

    pub const EOSZ: Operand = Operand(ffi::XED_OPERAND_EOSZ);

    pub const ERROR: Operand = Operand(ffi::XED_OPERAND_ERROR);

    pub const ESRC: Operand = Operand(ffi::XED_OPERAND_ESRC);

    pub const FIRST_F2F3: Operand = Operand(ffi::XED_OPERAND_FIRST_F2F3);

    pub const HAS_MODRM: Operand = Operand(ffi::XED_OPERAND_HAS_MODRM);

    pub const HAS_SIB: Operand = Operand(ffi::XED_OPERAND_HAS_SIB);

    pub const HINT: Operand = Operand(ffi::XED_OPERAND_HINT);

    pub const ICLASS: Operand = Operand(ffi::XED_OPERAND_ICLASS);

    pub const ILD_F2: Operand = Operand(ffi::XED_OPERAND_ILD_F2);

    pub const ILD_F3: Operand = Operand(ffi::XED_OPERAND_ILD_F3);

    pub const ILD_SEG: Operand = Operand(ffi::XED_OPERAND_ILD_SEG);

    pub const IMM0: Operand = Operand(ffi::XED_OPERAND_IMM0);

    pub const IMM0SIGNED: Operand = Operand(ffi::XED_OPERAND_IMM0SIGNED);

    pub const IMM1: Operand = Operand(ffi::XED_OPERAND_IMM1);

    pub const IMM1_BYTES: Operand = Operand(ffi::XED_OPERAND_IMM1_BYTES);

    pub const IMM_WIDTH: Operand = Operand(ffi::XED_OPERAND_IMM_WIDTH);

    pub const INDEX: Operand = Operand(ffi::XED_OPERAND_INDEX);

    pub const LAST_F2F3: Operand = Operand(ffi::XED_OPERAND_LAST_F2F3);

    pub const LLRC: Operand = Operand(ffi::XED_OPERAND_LLRC);

    pub const LOCK: Operand = Operand(ffi::XED_OPERAND_LOCK);

    pub const LZCNT: Operand = Operand(ffi::XED_OPERAND_LZCNT);

    pub const MAP: Operand = Operand(ffi::XED_OPERAND_MAP);

    pub const MASK: Operand = Operand(ffi::XED_OPERAND_MASK);

    pub const MAX_BYTES: Operand = Operand(ffi::XED_OPERAND_MAX_BYTES);

    pub const MEM0: Operand = Operand(ffi::XED_OPERAND_MEM0);

    pub const MEM1: Operand = Operand(ffi::XED_OPERAND_MEM1);

    pub const MEM_WIDTH: Operand = Operand(ffi::XED_OPERAND_MEM_WIDTH);

    pub const MOD: Operand = Operand(ffi::XED_OPERAND_MOD);

    pub const MODE: Operand = Operand(ffi::XED_OPERAND_MODE);

    pub const MODEP5: Operand = Operand(ffi::XED_OPERAND_MODEP5);

    pub const MODEP55C: Operand = Operand(ffi::XED_OPERAND_MODEP55C);

    pub const MODE_FIRST_PREFIX: Operand = Operand(ffi::XED_OPERAND_MODE_FIRST_PREFIX);

    pub const MODE_SHORT_UD0: Operand = Operand(ffi::XED_OPERAND_MODE_SHORT_UD0);

    pub const MODRM_BYTE: Operand = Operand(ffi::XED_OPERAND_MODRM_BYTE);

    pub const MPXMODE: Operand = Operand(ffi::XED_OPERAND_MPXMODE);

    pub const MUST_USE_EVEX: Operand = Operand(ffi::XED_OPERAND_MUST_USE_EVEX);

    pub const NEEDREX: Operand = Operand(ffi::XED_OPERAND_NEEDREX);

    pub const NEED_MEMDISP: Operand = Operand(ffi::XED_OPERAND_NEED_MEMDISP);

    pub const NEED_SIB: Operand = Operand(ffi::XED_OPERAND_NEED_SIB);

    pub const NELEM: Operand = Operand(ffi::XED_OPERAND_NELEM);

    pub const NOMINAL_OPCODE: Operand = Operand(ffi::XED_OPERAND_NOMINAL_OPCODE);

    pub const NOREX: Operand = Operand(ffi::XED_OPERAND_NOREX);

    pub const NO_SCALE_DISP8: Operand = Operand(ffi::XED_OPERAND_NO_SCALE_DISP8);

    pub const NPREFIXES: Operand = Operand(ffi::XED_OPERAND_NPREFIXES);

    pub const NREXES: Operand = Operand(ffi::XED_OPERAND_NREXES);

    pub const NSEG_PREFIXES: Operand = Operand(ffi::XED_OPERAND_NSEG_PREFIXES);

    pub const OSZ: Operand = Operand(ffi::XED_OPERAND_OSZ);

    pub const OUTREG: Operand = Operand(ffi::XED_OPERAND_OUTREG);

    pub const OUT_OF_BYTES: Operand = Operand(ffi::XED_OPERAND_OUT_OF_BYTES);

    pub const P4: Operand = Operand(ffi::XED_OPERAND_P4);

    pub const POS_DISP: Operand = Operand(ffi::XED_OPERAND_POS_DISP);

    pub const POS_IMM: Operand = Operand(ffi::XED_OPERAND_POS_IMM);

    pub const POS_IMM1: Operand = Operand(ffi::XED_OPERAND_POS_IMM1);

    pub const POS_MODRM: Operand = Operand(ffi::XED_OPERAND_POS_MODRM);

    pub const POS_NOMINAL_OPCODE: Operand = Operand(ffi::XED_OPERAND_POS_NOMINAL_OPCODE);

    pub const POS_SIB: Operand = Operand(ffi::XED_OPERAND_POS_SIB);

    pub const PREFIX66: Operand = Operand(ffi::XED_OPERAND_PREFIX66);

    pub const PTR: Operand = Operand(ffi::XED_OPERAND_PTR);

    pub const REALMODE: Operand = Operand(ffi::XED_OPERAND_REALMODE);

    pub const REG: Operand = Operand(ffi::XED_OPERAND_REG);

    pub const REG0: Operand = Operand(ffi::XED_OPERAND_REG0);

    pub const REG1: Operand = Operand(ffi::XED_OPERAND_REG1);

    pub const REG2: Operand = Operand(ffi::XED_OPERAND_REG2);

    pub const REG3: Operand = Operand(ffi::XED_OPERAND_REG3);

    pub const REG4: Operand = Operand(ffi::XED_OPERAND_REG4);

    pub const REG5: Operand = Operand(ffi::XED_OPERAND_REG5);

    pub const REG6: Operand = Operand(ffi::XED_OPERAND_REG6);

    pub const REG7: Operand = Operand(ffi::XED_OPERAND_REG7);

    pub const REG8: Operand = Operand(ffi::XED_OPERAND_REG8);

    pub const REG9: Operand = Operand(ffi::XED_OPERAND_REG9);

    pub const RELBR: Operand = Operand(ffi::XED_OPERAND_RELBR);

    pub const REP: Operand = Operand(ffi::XED_OPERAND_REP);

    pub const REX: Operand = Operand(ffi::XED_OPERAND_REX);

    pub const REXB: Operand = Operand(ffi::XED_OPERAND_REXB);

    pub const REXR: Operand = Operand(ffi::XED_OPERAND_REXR);

    pub const REXRR: Operand = Operand(ffi::XED_OPERAND_REXRR);

    pub const REXW: Operand = Operand(ffi::XED_OPERAND_REXW);

    pub const REXX: Operand = Operand(ffi::XED_OPERAND_REXX);

    pub const RM: Operand = Operand(ffi::XED_OPERAND_RM);

    pub const ROUNDC: Operand = Operand(ffi::XED_OPERAND_ROUNDC);

    pub const SAE: Operand = Operand(ffi::XED_OPERAND_SAE);

    pub const SCALE: Operand = Operand(ffi::XED_OPERAND_SCALE);

    pub const SEG0: Operand = Operand(ffi::XED_OPERAND_SEG0);

    pub const SEG1: Operand = Operand(ffi::XED_OPERAND_SEG1);

    pub const SEG_OVD: Operand = Operand(ffi::XED_OPERAND_SEG_OVD);

    pub const SIBBASE: Operand = Operand(ffi::XED_OPERAND_SIBBASE);

    pub const SIBINDEX: Operand = Operand(ffi::XED_OPERAND_SIBINDEX);

    pub const SIBSCALE: Operand = Operand(ffi::XED_OPERAND_SIBSCALE);

    pub const SMODE: Operand = Operand(ffi::XED_OPERAND_SMODE);

    pub const SRM: Operand = Operand(ffi::XED_OPERAND_SRM);

    pub const TZCNT: Operand = Operand(ffi::XED_OPERAND_TZCNT);

    pub const UBIT: Operand = Operand(ffi::XED_OPERAND_UBIT);

    pub const UIMM0: Operand = Operand(ffi::XED_OPERAND_UIMM0);

    pub const UIMM1: Operand = Operand(ffi::XED_OPERAND_UIMM1);

    pub const USING_DEFAULT_SEGMENT0: Operand = Operand(ffi::XED_OPERAND_USING_DEFAULT_SEGMENT0);

    pub const USING_DEFAULT_SEGMENT1: Operand = Operand(ffi::XED_OPERAND_USING_DEFAULT_SEGMENT1);

    pub const VEXDEST210: Operand = Operand(ffi::XED_OPERAND_VEXDEST210);

    pub const VEXDEST3: Operand = Operand(ffi::XED_OPERAND_VEXDEST3);

    pub const VEXDEST4: Operand = Operand(ffi::XED_OPERAND_VEXDEST4);

    pub const VEXVALID: Operand = Operand(ffi::XED_OPERAND_VEXVALID);

    pub const VEX_C4: Operand = Operand(ffi::XED_OPERAND_VEX_C4);

    pub const VEX_PREFIX: Operand = Operand(ffi::XED_OPERAND_VEX_PREFIX);

    pub const VL: Operand = Operand(ffi::XED_OPERAND_VL);

    pub const WBNOINVD: Operand = Operand(ffi::XED_OPERAND_WBNOINVD);

    pub const ZEROING: Operand = Operand(ffi::XED_OPERAND_ZEROING);
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_operand_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Operand {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(Operand(unsafe { ffi::str2xed_operand_enum_t(s.as_ptr()) }))
    }
}