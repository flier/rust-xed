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
pub struct Extension(ffi::xed_extension_enum_t);

impl Extension {
    pub const INVALID: Extension = Extension(ffi::XED_EXTENSION_INVALID);

    pub const _3DNOW: Extension = Extension(ffi::XED_EXTENSION_3DNOW);

    pub const _3DNOW_PREFETCH: Extension = Extension(ffi::XED_EXTENSION_3DNOW_PREFETCH);

    pub const ADOX_ADCX: Extension = Extension(ffi::XED_EXTENSION_ADOX_ADCX);

    pub const AES: Extension = Extension(ffi::XED_EXTENSION_AES);

    pub const AMD_INVLPGB: Extension = Extension(ffi::XED_EXTENSION_AMD_INVLPGB);

    pub const AMX_FP16: Extension = Extension(ffi::XED_EXTENSION_AMX_FP16);

    pub const AMX_TILE: Extension = Extension(ffi::XED_EXTENSION_AMX_TILE);

    pub const APXEVEX: Extension = Extension(ffi::XED_EXTENSION_APXEVEX);

    pub const APXLEGACY: Extension = Extension(ffi::XED_EXTENSION_APXLEGACY);

    pub const AVX: Extension = Extension(ffi::XED_EXTENSION_AVX);

    pub const AVX2: Extension = Extension(ffi::XED_EXTENSION_AVX2);

    pub const AVX2GATHER: Extension = Extension(ffi::XED_EXTENSION_AVX2GATHER);

    pub const AVX512EVEX: Extension = Extension(ffi::XED_EXTENSION_AVX512EVEX);

    pub const AVX512VEX: Extension = Extension(ffi::XED_EXTENSION_AVX512VEX);

    pub const AVXAES: Extension = Extension(ffi::XED_EXTENSION_AVXAES);

    pub const AVX_IFMA: Extension = Extension(ffi::XED_EXTENSION_AVX_IFMA);

    pub const AVX_NE_CONVERT: Extension = Extension(ffi::XED_EXTENSION_AVX_NE_CONVERT);

    pub const AVX_VNNI: Extension = Extension(ffi::XED_EXTENSION_AVX_VNNI);

    pub const AVX_VNNI_INT16: Extension = Extension(ffi::XED_EXTENSION_AVX_VNNI_INT16);

    pub const AVX_VNNI_INT8: Extension = Extension(ffi::XED_EXTENSION_AVX_VNNI_INT8);

    pub const BASE: Extension = Extension(ffi::XED_EXTENSION_BASE);

    pub const BMI1: Extension = Extension(ffi::XED_EXTENSION_BMI1);

    pub const BMI2: Extension = Extension(ffi::XED_EXTENSION_BMI2);

    pub const CET: Extension = Extension(ffi::XED_EXTENSION_CET);

    pub const CLDEMOTE: Extension = Extension(ffi::XED_EXTENSION_CLDEMOTE);

    pub const CLFLUSHOPT: Extension = Extension(ffi::XED_EXTENSION_CLFLUSHOPT);

    pub const CLFSH: Extension = Extension(ffi::XED_EXTENSION_CLFSH);

    pub const CLWB: Extension = Extension(ffi::XED_EXTENSION_CLWB);

    pub const CLZERO: Extension = Extension(ffi::XED_EXTENSION_CLZERO);

    pub const CMPCCXADD: Extension = Extension(ffi::XED_EXTENSION_CMPCCXADD);

    pub const ENQCMD: Extension = Extension(ffi::XED_EXTENSION_ENQCMD);

    pub const F16C: Extension = Extension(ffi::XED_EXTENSION_F16C);

    pub const FMA: Extension = Extension(ffi::XED_EXTENSION_FMA);

    pub const FMA4: Extension = Extension(ffi::XED_EXTENSION_FMA4);

    pub const FRED: Extension = Extension(ffi::XED_EXTENSION_FRED);

    pub const GFNI: Extension = Extension(ffi::XED_EXTENSION_GFNI);

    pub const HRESET: Extension = Extension(ffi::XED_EXTENSION_HRESET);

    pub const ICACHE_PREFETCH: Extension = Extension(ffi::XED_EXTENSION_ICACHE_PREFETCH);

    pub const INVPCID: Extension = Extension(ffi::XED_EXTENSION_INVPCID);

    pub const KEYLOCKER: Extension = Extension(ffi::XED_EXTENSION_KEYLOCKER);

    pub const KEYLOCKER_WIDE: Extension = Extension(ffi::XED_EXTENSION_KEYLOCKER_WIDE);

    pub const LKGS: Extension = Extension(ffi::XED_EXTENSION_LKGS);

    pub const LONGMODE: Extension = Extension(ffi::XED_EXTENSION_LONGMODE);

    pub const LZCNT: Extension = Extension(ffi::XED_EXTENSION_LZCNT);

    pub const MCOMMIT: Extension = Extension(ffi::XED_EXTENSION_MCOMMIT);

    pub const MMX: Extension = Extension(ffi::XED_EXTENSION_MMX);

    pub const MONITOR: Extension = Extension(ffi::XED_EXTENSION_MONITOR);

    pub const MONITORX: Extension = Extension(ffi::XED_EXTENSION_MONITORX);

    pub const MOVBE: Extension = Extension(ffi::XED_EXTENSION_MOVBE);

    pub const MOVDIR: Extension = Extension(ffi::XED_EXTENSION_MOVDIR);

    pub const MPX: Extension = Extension(ffi::XED_EXTENSION_MPX);

    pub const MSRLIST: Extension = Extension(ffi::XED_EXTENSION_MSRLIST);

    pub const PAUSE: Extension = Extension(ffi::XED_EXTENSION_PAUSE);

    pub const PBNDKB: Extension = Extension(ffi::XED_EXTENSION_PBNDKB);

    pub const PCLMULQDQ: Extension = Extension(ffi::XED_EXTENSION_PCLMULQDQ);

    pub const PCONFIG: Extension = Extension(ffi::XED_EXTENSION_PCONFIG);

    pub const PKU: Extension = Extension(ffi::XED_EXTENSION_PKU);

    pub const PREFETCHWT1: Extension = Extension(ffi::XED_EXTENSION_PREFETCHWT1);

    pub const PTWRITE: Extension = Extension(ffi::XED_EXTENSION_PTWRITE);

    pub const RAO_INT: Extension = Extension(ffi::XED_EXTENSION_RAO_INT);

    pub const RDPID: Extension = Extension(ffi::XED_EXTENSION_RDPID);

    pub const RDPRU: Extension = Extension(ffi::XED_EXTENSION_RDPRU);

    pub const RDRAND: Extension = Extension(ffi::XED_EXTENSION_RDRAND);

    pub const RDSEED: Extension = Extension(ffi::XED_EXTENSION_RDSEED);

    pub const RDTSCP: Extension = Extension(ffi::XED_EXTENSION_RDTSCP);

    pub const RDWRFSGS: Extension = Extension(ffi::XED_EXTENSION_RDWRFSGS);

    pub const RTM: Extension = Extension(ffi::XED_EXTENSION_RTM);

    pub const SERIALIZE: Extension = Extension(ffi::XED_EXTENSION_SERIALIZE);

    pub const SGX: Extension = Extension(ffi::XED_EXTENSION_SGX);

    pub const SGX_ENCLV: Extension = Extension(ffi::XED_EXTENSION_SGX_ENCLV);

    pub const SHA: Extension = Extension(ffi::XED_EXTENSION_SHA);

    pub const SHA512: Extension = Extension(ffi::XED_EXTENSION_SHA512);

    pub const SM3: Extension = Extension(ffi::XED_EXTENSION_SM3);

    pub const SM4: Extension = Extension(ffi::XED_EXTENSION_SM4);

    pub const SMAP: Extension = Extension(ffi::XED_EXTENSION_SMAP);

    pub const SMX: Extension = Extension(ffi::XED_EXTENSION_SMX);

    pub const SNP: Extension = Extension(ffi::XED_EXTENSION_SNP);

    pub const SSE: Extension = Extension(ffi::XED_EXTENSION_SSE);

    pub const SSE2: Extension = Extension(ffi::XED_EXTENSION_SSE2);

    pub const SSE3: Extension = Extension(ffi::XED_EXTENSION_SSE3);

    pub const SSE4: Extension = Extension(ffi::XED_EXTENSION_SSE4);

    pub const SSE4A: Extension = Extension(ffi::XED_EXTENSION_SSE4A);

    pub const SSSE3: Extension = Extension(ffi::XED_EXTENSION_SSSE3);

    pub const SVM: Extension = Extension(ffi::XED_EXTENSION_SVM);

    pub const TBM: Extension = Extension(ffi::XED_EXTENSION_TBM);

    pub const TDX: Extension = Extension(ffi::XED_EXTENSION_TDX);

    pub const TSX_LDTRK: Extension = Extension(ffi::XED_EXTENSION_TSX_LDTRK);

    pub const UINTR: Extension = Extension(ffi::XED_EXTENSION_UINTR);

    pub const USER_MSR: Extension = Extension(ffi::XED_EXTENSION_USER_MSR);

    pub const VAES: Extension = Extension(ffi::XED_EXTENSION_VAES);

    pub const VIA_PADLOCK_AES: Extension = Extension(ffi::XED_EXTENSION_VIA_PADLOCK_AES);

    pub const VIA_PADLOCK_MONTMUL: Extension = Extension(ffi::XED_EXTENSION_VIA_PADLOCK_MONTMUL);

    pub const VIA_PADLOCK_RNG: Extension = Extension(ffi::XED_EXTENSION_VIA_PADLOCK_RNG);

    pub const VIA_PADLOCK_SHA: Extension = Extension(ffi::XED_EXTENSION_VIA_PADLOCK_SHA);

    pub const VMFUNC: Extension = Extension(ffi::XED_EXTENSION_VMFUNC);

    pub const VPCLMULQDQ: Extension = Extension(ffi::XED_EXTENSION_VPCLMULQDQ);

    pub const VTX: Extension = Extension(ffi::XED_EXTENSION_VTX);

    pub const WAITPKG: Extension = Extension(ffi::XED_EXTENSION_WAITPKG);

    pub const WBNOINVD: Extension = Extension(ffi::XED_EXTENSION_WBNOINVD);

    pub const WRMSRNS: Extension = Extension(ffi::XED_EXTENSION_WRMSRNS);

    pub const X87: Extension = Extension(ffi::XED_EXTENSION_X87);

    pub const XOP: Extension = Extension(ffi::XED_EXTENSION_XOP);

    pub const XSAVE: Extension = Extension(ffi::XED_EXTENSION_XSAVE);

    pub const XSAVEC: Extension = Extension(ffi::XED_EXTENSION_XSAVEC);

    pub const XSAVEOPT: Extension = Extension(ffi::XED_EXTENSION_XSAVEOPT);

    pub const XSAVES: Extension = Extension(ffi::XED_EXTENSION_XSAVES);
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_extension_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Extension {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(Extension(unsafe {
            ffi::str2xed_extension_enum_t(s.as_ptr())
        }))
    }
}
