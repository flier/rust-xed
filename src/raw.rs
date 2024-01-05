use crate::ffi;

pub trait AsPtr {
    type CType;

    fn as_ptr(&self) -> *const Self::CType;
}

pub trait AsMutPtr: AsPtr {
    fn as_mut_ptr(&mut self) -> *mut Self::CType;
}

pub trait ToBool {
    fn bool(self) -> bool;
}

impl ToBool for ffi::xed_bool_t {
    fn bool(self) -> bool {
        self != 0
    }
}
