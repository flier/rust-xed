use std::{
    mem::MaybeUninit,
    ptr::{null_mut, NonNull},
    sync::RwLock,
};

use crate::{
    dec::{mem::MemOperand, Inst},
    error::ToResult,
    raw::AsPtr,
    Error, Reg,
};

/// A function for obtaining register values.
///
/// 32b return values should be zero extended to 64b.
pub type RegisterCallbackFn = fn(reg: Reg, context: Option<NonNull<()>>) -> Result<u64, Error>;

/// A function for obtaining the segment base values.
///
/// 32b return values should be zero extended zero extended to 64b.
pub type SegmentBaseCallbackFn = fn(reg: Reg, context: Option<NonNull<()>>) -> Result<u64, Error>;

/// Initialize the callback functions.
///
/// Tell XED what to call when using `agen``.
pub fn register_callback(
    register_fn: Option<RegisterCallbackFn>,
    segment_fn: Option<SegmentBaseCallbackFn>,
) {
    let mut f = REGISTER_FN.write().unwrap();
    *f = register_fn;

    let mut f = SEGMENT_FN.write().unwrap();
    *f = segment_fn;

    unsafe {
        ffi::xed_agen_register_callback(
            if register_fn.is_some() {
                Some(xed_register_callback_fn)
            } else {
                None
            },
            if segment_fn.is_some() {
                Some(xed_segment_base_callback_fn)
            } else {
                None
            },
        )
    }
}

static REGISTER_FN: RwLock<Option<RegisterCallbackFn>> = RwLock::new(None);
static SEGMENT_FN: RwLock<Option<SegmentBaseCallbackFn>> = RwLock::new(None);

#[no_mangle]
unsafe extern "C" fn xed_register_callback_fn(
    reg: ffi::xed_reg_enum_t,
    context: *mut ::std::os::raw::c_void,
    error: *mut ffi::xed_bool_t,
) -> u64 {
    let f = REGISTER_FN.read().unwrap();

    if let Some(f) = *f {
        match f(reg.into(), NonNull::new(context.cast())) {
            Ok(n) => return n,
            Err(_) => {
                *error = 1;
            }
        }
    }

    0
}

#[no_mangle]
unsafe extern "C" fn xed_segment_base_callback_fn(
    reg: ffi::xed_reg_enum_t,
    context: *mut ::std::os::raw::c_void,
    error: *mut ffi::xed_bool_t,
) -> u64 {
    let f = SEGMENT_FN.read().unwrap();

    if let Some(f) = *f {
        match f(reg.into(), NonNull::new(context.cast())) {
            Ok(n) => return n,
            Err(_) => {
                *error = 1;
            }
        }
    }

    0
}

impl MemOperand<&Inst> {
    /// Using the registered callbacks, compute the memory address for a specified memop in a decoded instruction.
    ///
    /// memop_index can have the value 0 for XED_OPERAND_MEM0, XED_OPERAND_AGEN, or 1 for XED_OPERAND_MEM1.
    /// Any other value results in an error being returned. The context parameter which is passed to the registered
    /// callbacks can be used to identify which thread's state is being referenced.
    /// The context parameter can also be used to specify which element of a vector register
    /// should be returned for gather an scatter operations.
    pub fn agen(&self, context: Option<NonNull<()>>) -> Result<u64, Error> {
        let mut addr = MaybeUninit::zeroed();

        unsafe {
            ffi::xed_agen(
                self.0.as_ptr().cast_mut(),
                self.1,
                context.map_or_else(null_mut, |p| p.as_ptr().cast()),
                addr.as_mut_ptr(),
            )
            .result()
            .map(|_| addr.assume_init())
        }
    }
}
