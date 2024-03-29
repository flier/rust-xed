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
pub struct Flag(ffi::xed_flag_enum_t);

impl Flag {
    pub const INVALID: Flag = Flag(ffi::XED_FLAG_INVALID);

    /// sign flag
    pub const SF: Flag = Flag(ffi::XED_FLAG_sf);

    /// zero flag
    pub const ZF: Flag = Flag(ffi::XED_FLAG_zf);

    /// auxiliary flag
    pub const AF: Flag = Flag(ffi::XED_FLAG_af);

    /// parity flag
    pub const PF: Flag = Flag(ffi::XED_FLAG_pf);

    /// carry flag
    pub const CF: Flag = Flag(ffi::XED_FLAG_cf);

    /// direction flag
    pub const DF: Flag = Flag(ffi::XED_FLAG_df);

    /// virtual interrupt flag
    pub const VIF: Flag = Flag(ffi::XED_FLAG_vif);

    /// I/O privilege level
    pub const IOPL: Flag = Flag(ffi::XED_FLAG_iopl);

    /// interrupt flag
    pub const IF: Flag = Flag(ffi::XED_FLAG_if);

    /// alignment check
    pub const AC: Flag = Flag(ffi::XED_FLAG_ac);

    /// virtual-8086 mode
    pub const VM: Flag = Flag(ffi::XED_FLAG_vm);

    /// resume flag
    pub const RF: Flag = Flag(ffi::XED_FLAG_rf);

    /// nested task
    pub const NT: Flag = Flag(ffi::XED_FLAG_nt);

    /// traf flag
    pub const TF: Flag = Flag(ffi::XED_FLAG_tf);

    /// ID flag
    pub const ID: Flag = Flag(ffi::XED_FLAG_id);

    /// virtual interrupt pending
    pub const VIP: Flag = Flag(ffi::XED_FLAG_vip);

    /// x87 FC0 flag
    pub const FC0: Flag = Flag(ffi::XED_FLAG_fc0);

    /// x87 FC1 flag
    pub const FC1: Flag = Flag(ffi::XED_FLAG_fc1);

    /// x87 FC2 flag
    pub const FC2: Flag = Flag(ffi::XED_FLAG_fc2);

    /// x87 FC3 flag
    pub const FC3: Flag = Flag(ffi::XED_FLAG_fc3);
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(unsafe {
            CStr::from_ptr(ffi::xed_flag_enum_t2str(self.0))
                .to_str()
                .unwrap()
        })
    }
}

impl FromStr for Flag {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = CString::new(s)?;

        Ok(Flag(unsafe { ffi::str2xed_flag_enum_t(s.as_ptr()) }))
    }
}
