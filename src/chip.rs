use std::mem::MaybeUninit;

use derive_more::{From, Into};

use crate::{ffi, impl_as_ptr, raw::AsMutPtr, Chip, IsaSet};

#[repr(transparent)]
#[derive(Clone, Debug, From, Into)]
pub struct Features(ffi::xed_chip_features_t);

impl_as_ptr!(Features(ffi::xed_chip_features_t));

impl Features {
    pub fn modify(&mut self, isa_set: IsaSet, enable: bool) {
        unsafe {
            ffi::xed_modify_chip_features(
                self.as_mut_ptr(),
                isa_set.into(),
                if enable { 1 } else { 0 },
            )
        }
    }
}

impl Chip {
    pub fn features(&self) -> Features {
        let mut features = MaybeUninit::zeroed();

        unsafe {
            ffi::xed_get_chip_features(features.as_mut_ptr(), (*self).into());

            features.assume_init().into()
        }
    }
}
