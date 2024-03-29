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
pub struct IsaSet(ffi::xed_isa_set_enum_t);

impl IsaSet {
    pub const INVALID: IsaSet = IsaSet(ffi::XED_ISA_SET_INVALID);

    pub const _3DNOW: IsaSet = IsaSet(ffi::XED_ISA_SET_3DNOW);

    pub const ADOX_ADCX: IsaSet = IsaSet(ffi::XED_ISA_SET_ADOX_ADCX);

    pub const AES: IsaSet = IsaSet(ffi::XED_ISA_SET_AES);

    pub const AMD: IsaSet = IsaSet(ffi::XED_ISA_SET_AMD);

    pub const AMD_INVLPGB: IsaSet = IsaSet(ffi::XED_ISA_SET_AMD_INVLPGB);

    pub const AMX_BF16: IsaSet = IsaSet(ffi::XED_ISA_SET_AMX_BF16);

    pub const AMX_COMPLEX: IsaSet = IsaSet(ffi::XED_ISA_SET_AMX_COMPLEX);

    pub const AMX_FP16: IsaSet = IsaSet(ffi::XED_ISA_SET_AMX_FP16);

    pub const AMX_INT8: IsaSet = IsaSet(ffi::XED_ISA_SET_AMX_INT8);

    pub const AMX_TILE: IsaSet = IsaSet(ffi::XED_ISA_SET_AMX_TILE);

    pub const APX_F: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F);

    pub const APX_F_ADX: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_ADX);

    pub const APX_F_AMX: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_AMX);

    pub const APX_F_BMI1: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_BMI1);

    pub const APX_F_BMI2: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_BMI2);

    pub const APX_F_CET: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_CET);

    pub const APX_F_CMPCCXADD: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_CMPCCXADD);

    pub const APX_F_ENQCMD: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_ENQCMD);

    pub const APX_F_INVPCID: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_INVPCID);

    pub const APX_F_KEYLOCKER: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_KEYLOCKER);

    pub const APX_F_KEYLOCKER_WIDE: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_KEYLOCKER_WIDE);

    pub const APX_F_KOPB: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_KOPB);

    pub const APX_F_KOPD: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_KOPD);

    pub const APX_F_KOPQ: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_KOPQ);

    pub const APX_F_KOPW: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_KOPW);

    pub const APX_F_LZCNT: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_LZCNT);

    pub const APX_F_MOVBE: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_MOVBE);

    pub const APX_F_MOVDIR64B: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_MOVDIR64B);

    pub const APX_F_MOVDIRI: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_MOVDIRI);

    pub const APX_F_RAO_INT: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_RAO_INT);

    pub const APX_F_SHA: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_SHA);

    pub const APX_F_USER_MSR: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_USER_MSR);

    pub const APX_F_VMX: IsaSet = IsaSet(ffi::XED_ISA_SET_APX_F_VMX);

    pub const AVX: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX);

    pub const AVX2: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX2);

    pub const AVX2GATHER: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX2GATHER);

    pub const AVX512BW_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512BW_128);

    pub const AVX512BW_128N: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512BW_128N);

    pub const AVX512BW_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512BW_256);

    pub const AVX512BW_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512BW_512);

    pub const AVX512BW_KOPD: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512BW_KOPD);

    pub const AVX512BW_KOPQ: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512BW_KOPQ);

    pub const AVX512CD_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512CD_128);

    pub const AVX512CD_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512CD_256);

    pub const AVX512CD_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512CD_512);

    pub const AVX512DQ_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_128);

    pub const AVX512DQ_128N: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_128N);

    pub const AVX512DQ_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_256);

    pub const AVX512DQ_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_512);

    pub const AVX512DQ_KOPB: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_KOPB);

    pub const AVX512DQ_KOPW: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_KOPW);

    pub const AVX512DQ_SCALAR: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512DQ_SCALAR);

    pub const AVX512ER_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512ER_512);

    pub const AVX512ER_SCALAR: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512ER_SCALAR);

    pub const AVX512F_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512F_128);

    pub const AVX512F_128N: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512F_128N);

    pub const AVX512F_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512F_256);

    pub const AVX512F_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512F_512);

    pub const AVX512F_KOPW: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512F_KOPW);

    pub const AVX512F_SCALAR: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512F_SCALAR);

    pub const AVX512PF_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512PF_512);

    pub const AVX512_4FMAPS_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_4FMAPS_512);

    pub const AVX512_4FMAPS_SCALAR: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_4FMAPS_SCALAR);

    pub const AVX512_4VNNIW_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_4VNNIW_512);

    pub const AVX512_BF16_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_BF16_128);

    pub const AVX512_BF16_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_BF16_256);

    pub const AVX512_BF16_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_BF16_512);

    pub const AVX512_BITALG_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_BITALG_128);

    pub const AVX512_BITALG_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_BITALG_256);

    pub const AVX512_BITALG_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_BITALG_512);

    pub const AVX512_FP16_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_FP16_128);

    pub const AVX512_FP16_128N: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_FP16_128N);

    pub const AVX512_FP16_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_FP16_256);

    pub const AVX512_FP16_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_FP16_512);

    pub const AVX512_FP16_SCALAR: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_FP16_SCALAR);

    pub const AVX512_GFNI_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_GFNI_128);

    pub const AVX512_GFNI_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_GFNI_256);

    pub const AVX512_GFNI_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_GFNI_512);

    pub const AVX512_IFMA_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_IFMA_128);

    pub const AVX512_IFMA_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_IFMA_256);

    pub const AVX512_IFMA_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_IFMA_512);

    pub const AVX512_VAES_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VAES_128);

    pub const AVX512_VAES_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VAES_256);

    pub const AVX512_VAES_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VAES_512);

    pub const AVX512_VBMI2_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VBMI2_128);

    pub const AVX512_VBMI2_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VBMI2_256);

    pub const AVX512_VBMI2_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VBMI2_512);

    pub const AVX512_VBMI_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VBMI_128);

    pub const AVX512_VBMI_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VBMI_256);

    pub const AVX512_VBMI_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VBMI_512);

    pub const AVX512_VNNI_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VNNI_128);

    pub const AVX512_VNNI_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VNNI_256);

    pub const AVX512_VNNI_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VNNI_512);

    pub const AVX512_VP2INTERSECT_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VP2INTERSECT_128);

    pub const AVX512_VP2INTERSECT_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VP2INTERSECT_256);

    pub const AVX512_VP2INTERSECT_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VP2INTERSECT_512);

    pub const AVX512_VPCLMULQDQ_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VPCLMULQDQ_128);

    pub const AVX512_VPCLMULQDQ_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VPCLMULQDQ_256);

    pub const AVX512_VPCLMULQDQ_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VPCLMULQDQ_512);

    pub const AVX512_VPOPCNTDQ_128: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VPOPCNTDQ_128);

    pub const AVX512_VPOPCNTDQ_256: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VPOPCNTDQ_256);

    pub const AVX512_VPOPCNTDQ_512: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX512_VPOPCNTDQ_512);

    pub const AVXAES: IsaSet = IsaSet(ffi::XED_ISA_SET_AVXAES);

    pub const AVX_GFNI: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX_GFNI);

    pub const AVX_IFMA: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX_IFMA);

    pub const AVX_NE_CONVERT: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX_NE_CONVERT);

    pub const AVX_VNNI: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX_VNNI);

    pub const AVX_VNNI_INT16: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX_VNNI_INT16);

    pub const AVX_VNNI_INT8: IsaSet = IsaSet(ffi::XED_ISA_SET_AVX_VNNI_INT8);

    pub const BMI1: IsaSet = IsaSet(ffi::XED_ISA_SET_BMI1);

    pub const BMI2: IsaSet = IsaSet(ffi::XED_ISA_SET_BMI2);

    pub const CET: IsaSet = IsaSet(ffi::XED_ISA_SET_CET);

    pub const CLDEMOTE: IsaSet = IsaSet(ffi::XED_ISA_SET_CLDEMOTE);

    pub const CLFLUSHOPT: IsaSet = IsaSet(ffi::XED_ISA_SET_CLFLUSHOPT);

    pub const CLFSH: IsaSet = IsaSet(ffi::XED_ISA_SET_CLFSH);

    pub const CLWB: IsaSet = IsaSet(ffi::XED_ISA_SET_CLWB);

    pub const CLZERO: IsaSet = IsaSet(ffi::XED_ISA_SET_CLZERO);

    pub const CMOV: IsaSet = IsaSet(ffi::XED_ISA_SET_CMOV);

    pub const CMPCCXADD: IsaSet = IsaSet(ffi::XED_ISA_SET_CMPCCXADD);

    pub const CMPXCHG16B: IsaSet = IsaSet(ffi::XED_ISA_SET_CMPXCHG16B);

    pub const ENQCMD: IsaSet = IsaSet(ffi::XED_ISA_SET_ENQCMD);

    pub const F16C: IsaSet = IsaSet(ffi::XED_ISA_SET_F16C);

    pub const FAT_NOP: IsaSet = IsaSet(ffi::XED_ISA_SET_FAT_NOP);

    pub const FCMOV: IsaSet = IsaSet(ffi::XED_ISA_SET_FCMOV);

    pub const FCOMI: IsaSet = IsaSet(ffi::XED_ISA_SET_FCOMI);

    pub const FMA: IsaSet = IsaSet(ffi::XED_ISA_SET_FMA);

    pub const FMA4: IsaSet = IsaSet(ffi::XED_ISA_SET_FMA4);

    pub const FRED: IsaSet = IsaSet(ffi::XED_ISA_SET_FRED);

    pub const FXSAVE: IsaSet = IsaSet(ffi::XED_ISA_SET_FXSAVE);

    pub const FXSAVE64: IsaSet = IsaSet(ffi::XED_ISA_SET_FXSAVE64);

    pub const GFNI: IsaSet = IsaSet(ffi::XED_ISA_SET_GFNI);

    pub const HRESET: IsaSet = IsaSet(ffi::XED_ISA_SET_HRESET);

    pub const I186: IsaSet = IsaSet(ffi::XED_ISA_SET_I186);

    pub const I286PROTECTED: IsaSet = IsaSet(ffi::XED_ISA_SET_I286PROTECTED);

    pub const I286REAL: IsaSet = IsaSet(ffi::XED_ISA_SET_I286REAL);

    pub const I386: IsaSet = IsaSet(ffi::XED_ISA_SET_I386);

    pub const I486: IsaSet = IsaSet(ffi::XED_ISA_SET_I486);

    pub const I486REAL: IsaSet = IsaSet(ffi::XED_ISA_SET_I486REAL);

    pub const I86: IsaSet = IsaSet(ffi::XED_ISA_SET_I86);

    pub const ICACHE_PREFETCH: IsaSet = IsaSet(ffi::XED_ISA_SET_ICACHE_PREFETCH);

    pub const INVPCID: IsaSet = IsaSet(ffi::XED_ISA_SET_INVPCID);

    pub const KEYLOCKER: IsaSet = IsaSet(ffi::XED_ISA_SET_KEYLOCKER);

    pub const KEYLOCKER_WIDE: IsaSet = IsaSet(ffi::XED_ISA_SET_KEYLOCKER_WIDE);

    pub const LAHF: IsaSet = IsaSet(ffi::XED_ISA_SET_LAHF);

    pub const LKGS: IsaSet = IsaSet(ffi::XED_ISA_SET_LKGS);

    pub const LONGMODE: IsaSet = IsaSet(ffi::XED_ISA_SET_LONGMODE);

    pub const LWP: IsaSet = IsaSet(ffi::XED_ISA_SET_LWP);

    pub const LZCNT: IsaSet = IsaSet(ffi::XED_ISA_SET_LZCNT);

    pub const MCOMMIT: IsaSet = IsaSet(ffi::XED_ISA_SET_MCOMMIT);

    pub const MONITOR: IsaSet = IsaSet(ffi::XED_ISA_SET_MONITOR);

    pub const MONITORX: IsaSet = IsaSet(ffi::XED_ISA_SET_MONITORX);

    pub const MOVBE: IsaSet = IsaSet(ffi::XED_ISA_SET_MOVBE);

    pub const MOVDIR64B: IsaSet = IsaSet(ffi::XED_ISA_SET_MOVDIR64B);

    pub const MOVDIRI: IsaSet = IsaSet(ffi::XED_ISA_SET_MOVDIRI);

    pub const MPX: IsaSet = IsaSet(ffi::XED_ISA_SET_MPX);

    pub const MSRLIST: IsaSet = IsaSet(ffi::XED_ISA_SET_MSRLIST);

    pub const PAUSE: IsaSet = IsaSet(ffi::XED_ISA_SET_PAUSE);

    pub const PBNDKB: IsaSet = IsaSet(ffi::XED_ISA_SET_PBNDKB);

    pub const PCLMULQDQ: IsaSet = IsaSet(ffi::XED_ISA_SET_PCLMULQDQ);

    pub const PCONFIG: IsaSet = IsaSet(ffi::XED_ISA_SET_PCONFIG);

    pub const PENTIUMMMX: IsaSet = IsaSet(ffi::XED_ISA_SET_PENTIUMMMX);

    pub const PENTIUMREAL: IsaSet = IsaSet(ffi::XED_ISA_SET_PENTIUMREAL);

    pub const PKU: IsaSet = IsaSet(ffi::XED_ISA_SET_PKU);

    pub const POPCNT: IsaSet = IsaSet(ffi::XED_ISA_SET_POPCNT);

    pub const PPRO: IsaSet = IsaSet(ffi::XED_ISA_SET_PPRO);

    pub const PPRO_UD0_LONG: IsaSet = IsaSet(ffi::XED_ISA_SET_PPRO_UD0_LONG);

    pub const PPRO_UD0_SHORT: IsaSet = IsaSet(ffi::XED_ISA_SET_PPRO_UD0_SHORT);

    pub const PREFETCHW: IsaSet = IsaSet(ffi::XED_ISA_SET_PREFETCHW);

    pub const PREFETCHWT1: IsaSet = IsaSet(ffi::XED_ISA_SET_PREFETCHWT1);

    pub const PREFETCH_NOP: IsaSet = IsaSet(ffi::XED_ISA_SET_PREFETCH_NOP);

    pub const PTWRITE: IsaSet = IsaSet(ffi::XED_ISA_SET_PTWRITE);

    pub const RAO_INT: IsaSet = IsaSet(ffi::XED_ISA_SET_RAO_INT);

    pub const RDPID: IsaSet = IsaSet(ffi::XED_ISA_SET_RDPID);

    pub const RDPMC: IsaSet = IsaSet(ffi::XED_ISA_SET_RDPMC);

    pub const RDPRU: IsaSet = IsaSet(ffi::XED_ISA_SET_RDPRU);

    pub const RDRAND: IsaSet = IsaSet(ffi::XED_ISA_SET_RDRAND);

    pub const RDSEED: IsaSet = IsaSet(ffi::XED_ISA_SET_RDSEED);

    pub const RDTSCP: IsaSet = IsaSet(ffi::XED_ISA_SET_RDTSCP);

    pub const RDWRFSGS: IsaSet = IsaSet(ffi::XED_ISA_SET_RDWRFSGS);

    pub const RTM: IsaSet = IsaSet(ffi::XED_ISA_SET_RTM);

    pub const SERIALIZE: IsaSet = IsaSet(ffi::XED_ISA_SET_SERIALIZE);

    pub const SGX: IsaSet = IsaSet(ffi::XED_ISA_SET_SGX);

    pub const SGX_ENCLV: IsaSet = IsaSet(ffi::XED_ISA_SET_SGX_ENCLV);

    pub const SHA: IsaSet = IsaSet(ffi::XED_ISA_SET_SHA);

    pub const SHA512: IsaSet = IsaSet(ffi::XED_ISA_SET_SHA512);

    pub const SM3: IsaSet = IsaSet(ffi::XED_ISA_SET_SM3);

    pub const SM4: IsaSet = IsaSet(ffi::XED_ISA_SET_SM4);

    pub const SMAP: IsaSet = IsaSet(ffi::XED_ISA_SET_SMAP);

    pub const SMX: IsaSet = IsaSet(ffi::XED_ISA_SET_SMX);

    pub const SNP: IsaSet = IsaSet(ffi::XED_ISA_SET_SNP);

    pub const SSE: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE);

    pub const SSE2: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE2);

    pub const SSE2MMX: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE2MMX);

    pub const SSE3: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE3);

    pub const SSE3X87: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE3X87);

    pub const SSE4: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE4);

    pub const SSE42: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE42);

    pub const SSE4A: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE4A);

    pub const SSEMXCSR: IsaSet = IsaSet(ffi::XED_ISA_SET_SSEMXCSR);

    pub const SSE_PREFETCH: IsaSet = IsaSet(ffi::XED_ISA_SET_SSE_PREFETCH);

    pub const SSSE3: IsaSet = IsaSet(ffi::XED_ISA_SET_SSSE3);

    pub const SSSE3MMX: IsaSet = IsaSet(ffi::XED_ISA_SET_SSSE3MMX);

    pub const SVM: IsaSet = IsaSet(ffi::XED_ISA_SET_SVM);

    pub const TBM: IsaSet = IsaSet(ffi::XED_ISA_SET_TBM);

    pub const TDX: IsaSet = IsaSet(ffi::XED_ISA_SET_TDX);

    pub const TSX_LDTRK: IsaSet = IsaSet(ffi::XED_ISA_SET_TSX_LDTRK);

    pub const UINTR: IsaSet = IsaSet(ffi::XED_ISA_SET_UINTR);

    pub const USER_MSR: IsaSet = IsaSet(ffi::XED_ISA_SET_USER_MSR);

    pub const VAES: IsaSet = IsaSet(ffi::XED_ISA_SET_VAES);

    pub const VIA_PADLOCK_AES: IsaSet = IsaSet(ffi::XED_ISA_SET_VIA_PADLOCK_AES);

    pub const VIA_PADLOCK_MONTMUL: IsaSet = IsaSet(ffi::XED_ISA_SET_VIA_PADLOCK_MONTMUL);

    pub const VIA_PADLOCK_RNG: IsaSet = IsaSet(ffi::XED_ISA_SET_VIA_PADLOCK_RNG);

    pub const VIA_PADLOCK_SHA: IsaSet = IsaSet(ffi::XED_ISA_SET_VIA_PADLOCK_SHA);

    pub const VMFUNC: IsaSet = IsaSet(ffi::XED_ISA_SET_VMFUNC);

    pub const VPCLMULQDQ: IsaSet = IsaSet(ffi::XED_ISA_SET_VPCLMULQDQ);

    pub const VTX: IsaSet = IsaSet(ffi::XED_ISA_SET_VTX);

    pub const WAITPKG: IsaSet = IsaSet(ffi::XED_ISA_SET_WAITPKG);

    pub const WBNOINVD: IsaSet = IsaSet(ffi::XED_ISA_SET_WBNOINVD);

    pub const WRMSRNS: IsaSet = IsaSet(ffi::XED_ISA_SET_WRMSRNS);

    pub const X87: IsaSet = IsaSet(ffi::XED_ISA_SET_X87);

    pub const XOP: IsaSet = IsaSet(ffi::XED_ISA_SET_XOP);

    pub const XSAVE: IsaSet = IsaSet(ffi::XED_ISA_SET_XSAVE);

    pub const XSAVEC: IsaSet = IsaSet(ffi::XED_ISA_SET_XSAVEC);

    pub const XSAVEOPT: IsaSet = IsaSet(ffi::XED_ISA_SET_XSAVEOPT);

    pub const XSAVES: IsaSet = IsaSet(ffi::XED_ISA_SET_XSAVES);
}

impl fmt::Display for IsaSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_isa_set_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for IsaSet {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(IsaSet(unsafe { ffi::str2xed_isa_set_enum_t(s.as_ptr()) }))
    }
}
