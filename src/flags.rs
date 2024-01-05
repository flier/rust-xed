use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;
use std::ops::Deref;

use crate::{
    ffi, gen,
    raw::{AsPtr, ToBool},
    Flag,
};

/// a union of flags bits
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FlagSet(ffi::xed_flag_set_t);

impl Deref for FlagSet {
    type Target = ffi::xed_flag_set_s__bindgen_ty_1;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.s }
    }
}

impl AsPtr for FlagSet {
    type CType = ffi::xed_flag_set_t;

    fn as_ptr(&self) -> *const Self::CType {
        &self.0 as *const _
    }
}

impl fmt::Display for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = MaybeUninit::<[u8; Self::BUF_LEN]>::zeroed();

        let s = unsafe {
            let n = ffi::xed_flag_set_print(
                self.as_ptr(),
                buf.as_mut_ptr().cast(),
                Self::BUF_LEN as i32,
            );
            let b = buf.assume_init();

            CStr::from_bytes_with_nul_unchecked(&b[..n as usize])
                .to_string_lossy()
                .to_string()
        };

        write!(f, "{}", s)
    }
}

impl FlagSet {
    const BUF_LEN: usize = 256;

    properties! {
        /// Return the flags as a mask
        mask: u32 { xed_flag_set_mask }
    }

    /// returns true if this object has a subset of the flags of the "other" object.
    pub fn subset_of(&self, other: &Self) -> bool {
        unsafe { ffi::xed_flag_set_is_subset_of(self.as_ptr(), other.as_ptr()) }.bool()
    }
}

/// Associated with each flag field there can be one action.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FlagAction(ffi::xed_flag_action_t);

impl AsPtr for FlagAction {
    type CType = ffi::xed_flag_action_t;

    fn as_ptr(&self) -> *const Self::CType {
        &self.0 as *const _
    }
}

impl fmt::Display for FlagAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())?;

        if let Some(action) = self.action(0) {
            write!(f, "-{}", action)?;
        }

        Ok(())
    }
}

impl FlagAction {
    properties! {
        /// get the name of the flag
        name: Flag { xed_flag_action_get_flag_name }

        /// returns true if either action is a read
        read: bool { xed_flag_action_read_flag }

        /// returns true if either action is a write
        write: bool { xed_flag_action_writes_flag }
    }

    pub fn action(&self, i: u32) -> Option<gen::FlagAction> {
        let action = unsafe { ffi::xed_flag_action_get_action(self.as_ptr(), i) };

        (action != 0).then(|| action.into())
    }
}

/// A collection of FlagAction's and unions of read and written flags
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct SimpleFlag(ffi::xed_simple_flag_t);

impl AsPtr for SimpleFlag {
    type CType = ffi::xed_simple_flag_t;

    fn as_ptr(&self) -> *const Self::CType {
        &self.0 as *const _
    }
}

impl fmt::Display for SimpleFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = MaybeUninit::<[u8; Self::BUF_LEN]>::zeroed();

        let s = unsafe {
            let n = ffi::xed_simple_flag_print(
                self.as_ptr(),
                buf.as_mut_ptr().cast(),
                Self::BUF_LEN as i32,
            );
            let b = buf.assume_init();

            CStr::from_bytes_with_nul_unchecked(&b[..n as usize])
                .to_string_lossy()
                .to_string()
        };

        write!(f, "{}", s)
    }
}

impl SimpleFlag {
    const BUF_LEN: usize = 256;

    properties! {
        /// returns the number of flag-actions
        nflags: u32 { xed_simple_flag_get_nflags }

        /// return union of bits for read flags
        read_set: &FlagSet { xed_simple_flag_get_read_flag_set }

        /// return union of bits for written flags
        written_set: &FlagSet { xed_simple_flag_get_written_flag_set }

        /// return union of bits for undefined flags
        undefined_set: &FlagSet { xed_simple_flag_get_undefined_flag_set }

        /// Indicates the flags are only conditionally written.
        ///
        /// Usually MAY-writes of the flags instructions that are dependent on a REP count.
        may_write: bool { xed_simple_flag_get_may_write }

        /// the flags always written
        must_write: bool { xed_simple_flag_get_must_write }

        /// boolean test to see if flags are read, scans the flags
        reads_flags: bool { xed_simple_flag_reads_flags }

        /// boolean test to see if flags are written, scans the flags
        writes_flags: bool { xed_simple_flag_writes_flags }
    }

    pub fn actions(&self) -> impl Iterator<Item = &FlagAction> {
        unsafe {
            (0..ffi::xed_simple_flag_get_nflags(self.as_ptr())).flat_map(|i| {
                ffi::xed_simple_flag_get_flag_action(self.as_ptr(), i)
                    .cast::<FlagAction>()
                    .as_ref()
            })
        }
    }

    /// return the specific flag-action. Very detailed low level information
    pub fn action(&self, i: u32) -> &FlagAction {
        unsafe {
            ffi::xed_simple_flag_get_flag_action(self.as_ptr(), i)
                .cast::<FlagAction>()
                .as_ref()
                .unwrap()
        }
    }
}
