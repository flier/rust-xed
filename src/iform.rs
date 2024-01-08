use crate::{ffi, Category, Extension, Iclass, Iform, IsaSet};

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Info(ffi::xed_iform_info_t);

impl Info {
    /// Return the iclass for a given iform.
    pub fn iclass(&self) -> Iclass {
        (self.0.iclass() as i32).into()
    }

    /// Return the category for a given iform.
    pub fn category(&self) -> Category {
        (self.0.category() as i32).into()
    }

    /// Return the extension for a given iform.
    pub fn extension(&self) -> Extension {
        (self.0.extension() as i32).into()
    }

    /// Return the isa_set for a given iform.
    pub fn isa_set(&self) -> IsaSet {
        (self.0.isa_set() as i32).into()
    }
}

impl Iform {
    pub fn info(self) -> Option<&'static Info> {
        unsafe { ffi::xed_iform_map(self.into()).cast::<Info>().as_ref() }
    }

    /// Return the iclass for a given iform.
    pub fn iclass(self) -> Iclass {
        unsafe { ffi::xed_iform_to_iclass(self.into()) }.into()
    }

    /// Return the category for a given iform.
    pub fn category(self) -> Category {
        unsafe { ffi::xed_iform_to_category(self.into()) }.into()
    }

    /// Return the extension for a given iform.
    pub fn extension(self) -> Extension {
        unsafe { ffi::xed_iform_to_extension(self.into()) }.into()
    }

    /// Return the isa_set for a given iform.
    pub fn isa_set(self) -> IsaSet {
        unsafe { ffi::xed_iform_to_isa_set(self.into()) }.into()
    }
}
