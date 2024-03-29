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
pub struct CpuidRec(ffi::xed_cpuid_rec_enum_t);

impl CpuidRec {
    pub const INVALID: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_INVALID);

    pub const ADOXADCX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_ADOXADCX);

    pub const AES: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AES);

    pub const AMX_BF16: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AMX_BF16);

    pub const AMX_COMPLEX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AMX_COMPLEX);

    pub const AMX_FP16: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AMX_FP16);

    pub const AMX_INT8: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AMX_INT8);

    pub const AMX_TILES: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AMX_TILES);

    pub const APX_F: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_APX_F);

    pub const AVX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX);

    pub const AVX10_128VL: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX10_128VL);

    pub const AVX10_256VL: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX10_256VL);

    pub const AVX10_512VL: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX10_512VL);

    pub const AVX10_ENABLED: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX10_ENABLED);

    pub const AVX10_VER1: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX10_VER1);

    pub const AVX2: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX2);

    pub const AVX512BW: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512BW);

    pub const AVX512CD: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512CD);

    pub const AVX512DQ: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512DQ);

    pub const AVX512ER: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512ER);

    pub const AVX512F: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512F);

    pub const AVX512IFMA: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512IFMA);

    pub const AVX512PF: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512PF);

    pub const AVX512VBMI: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512VBMI);

    pub const AVX512VL: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512VL);

    pub const AVX512_4FMAPS: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_4FMAPS);

    pub const AVX512_4VNNIW: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_4VNNIW);

    pub const AVX512_BITALG: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_BITALG);

    pub const AVX512_FP16: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_FP16);

    pub const AVX512_VBMI2: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_VBMI2);

    pub const AVX512_VNNI: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_VNNI);

    pub const AVX512_VP2INTERSECT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_VP2INTERSECT);

    pub const AVX512_VPOPCNTDQ: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX512_VPOPCNTDQ);

    pub const AVX_IFMA: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX_IFMA);

    pub const AVX_NE_CONVERT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX_NE_CONVERT);

    pub const AVX_VNNI: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX_VNNI);

    pub const AVX_VNNI_INT16: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX_VNNI_INT16);

    pub const AVX_VNNI_INT8: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_AVX_VNNI_INT8);

    pub const BF16: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_BF16);

    pub const BMI1: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_BMI1);

    pub const BMI2: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_BMI2);

    pub const CET: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CET);

    pub const CLDEMOTE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CLDEMOTE);

    pub const CLFLUSH: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CLFLUSH);

    pub const CLFLUSHOPT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CLFLUSHOPT);

    pub const CLWB: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CLWB);

    pub const CMOV: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CMOV);

    pub const CMPCCXADD: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CMPCCXADD);

    pub const CMPXCHG16B: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_CMPXCHG16B);

    pub const ENQCMD: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_ENQCMD);

    pub const F16C: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_F16C);

    pub const FMA: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_FMA);

    pub const FPU: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_FPU);

    pub const FRED: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_FRED);

    pub const FXSAVE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_FXSAVE);

    pub const GFNI: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_GFNI);

    pub const HRESET: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_HRESET);

    pub const ICACHE_PREFETCH: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_ICACHE_PREFETCH);

    pub const INTEL64: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_INTEL64);

    pub const INTELPT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_INTELPT);

    pub const INVPCID: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_INVPCID);

    pub const KLENABLED: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_KLENABLED);

    pub const KLSUPPORTED: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_KLSUPPORTED);

    pub const KLWIDE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_KLWIDE);

    pub const LAHF: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_LAHF);

    pub const LKGS: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_LKGS);

    pub const LZCNT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_LZCNT);

    pub const MCOMMIT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MCOMMIT);

    pub const MMX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MMX);

    pub const MONITOR: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MONITOR);

    pub const MONITORX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MONITORX);

    pub const MOVDIR64B: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MOVDIR64B);

    pub const MOVDIRI: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MOVDIRI);

    pub const MOVEBE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MOVEBE);

    pub const MPX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MPX);

    pub const MSRLIST: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_MSRLIST);

    pub const OSPKU: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_OSPKU);

    pub const OSXSAVE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_OSXSAVE);

    pub const PBNDKB: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PBNDKB);

    pub const PCLMULQDQ: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PCLMULQDQ);

    pub const PCONFIG: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PCONFIG);

    pub const PKU: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PKU);

    pub const POPCNT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_POPCNT);

    pub const PREFETCHW: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PREFETCHW);

    pub const PREFETCHWT1: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PREFETCHWT1);

    pub const PTWRITE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_PTWRITE);

    pub const RAO_INT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RAO_INT);

    pub const RDP: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RDP);

    pub const RDPRU: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RDPRU);

    pub const RDRAND: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RDRAND);

    pub const RDSEED: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RDSEED);

    pub const RDTSCP: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RDTSCP);

    pub const RDWRFSGS: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RDWRFSGS);

    pub const RTM: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_RTM);

    pub const SERIALIZE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SERIALIZE);

    pub const SGX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SGX);

    pub const SHA: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SHA);

    pub const SHA512: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SHA512);

    pub const SM3: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SM3);

    pub const SM4: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SM4);

    pub const SMAP: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SMAP);

    pub const SMX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SMX);

    pub const SNP: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SNP);

    pub const SSE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSE);

    pub const SSE2: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSE2);

    pub const SSE3: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSE3);

    pub const SSE4: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSE4);

    pub const SSE42: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSE42);

    pub const SSE4A: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSE4A);

    pub const SSSE3: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_SSSE3);

    pub const TSX_LDTRK: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_TSX_LDTRK);

    pub const UINTR: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_UINTR);

    pub const USER_MSR: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_USER_MSR);

    pub const VAES: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VAES);

    pub const VIA_PADLOCK_AES: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_AES);

    pub const VIA_PADLOCK_AES_EN: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_AES_EN);

    pub const VIA_PADLOCK_PMM: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_PMM);

    pub const VIA_PADLOCK_PMM_EN: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_PMM_EN);

    pub const VIA_PADLOCK_RNG: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_RNG);

    pub const VIA_PADLOCK_RNG_EN: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_RNG_EN);

    pub const VIA_PADLOCK_SHA: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_SHA);

    pub const VIA_PADLOCK_SHA_EN: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VIA_PADLOCK_SHA_EN);

    pub const VMX: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VMX);

    pub const VPCLMULQDQ: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_VPCLMULQDQ);

    pub const WAITPKG: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_WAITPKG);

    pub const WBNOINVD: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_WBNOINVD);

    pub const WRMSRNS: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_WRMSRNS);

    pub const XSAVE: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_XSAVE);

    pub const XSAVEC: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_XSAVEC);

    pub const XSAVEOPT: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_XSAVEOPT);

    pub const XSAVES: CpuidRec = CpuidRec(ffi::XED_CPUID_REC_XSAVES);
}

impl fmt::Display for CpuidRec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_cpuid_rec_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for CpuidRec {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(CpuidRec(unsafe {
            ffi::str2xed_cpuid_rec_enum_t(s.as_ptr())
        }))
    }
}
