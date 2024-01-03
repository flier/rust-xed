use std::mem::MaybeUninit;

use crate::{ffi, AddressWidth, Errno, MachineMode, Result};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Inst(ffi::xed_decoded_inst_t);

impl Default for Inst {
    fn default() -> Self {
        Self::new()
    }
}

impl Inst {
    /// Create and zero the decode structure completely.
    pub fn new() -> Self {
        let mut inst = MaybeUninit::zeroed();

        Self(unsafe {
            ffi::xed_decoded_inst_zero(inst.as_mut_ptr());

            inst.assume_init()
        })
    }

    /// Create with the machine mode and stack addressing width directly.
    pub fn with_mode(mode: MachineMode, width: AddressWidth) -> Self {
        let mut inst = Self::new();

        inst.set_mode(mode, width);

        inst
    }

    /// Set the machine mode and stack addressing width directly.
    pub fn set_mode(&mut self, mode: MachineMode, width: AddressWidth) {
        unsafe { ffi::xed_decoded_inst_set_mode(&mut self.0 as *mut _, mode.into(), width.into()) }
    }

    pub fn decode<T: AsRef<[u8]>>(&mut self, bytes: T) -> Result<()> {
        let b = bytes.as_ref();

        let errno = Errno::from(unsafe {
            ffi::xed_decode(&mut self.0 as *mut _, b.as_ptr(), b.len() as u32)
        });

        if errno.is_none() {
            Ok(())
        } else {
            Err(errno.into())
        }
    }
}
