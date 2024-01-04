pub trait AsPtr {
    type CType;

    fn as_ptr(&self) -> *const Self::CType;
}

pub trait AsMutPtr: AsPtr {
    fn as_mut_ptr(&mut self) -> *mut Self::CType;
}
