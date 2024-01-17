use std::{
    os::raw::{c_char, c_int, c_uint, c_ulonglong, c_void},
    ptr, slice,
};

/// A `DisassemblyCallback` takes an address, a mutable reference to a symbol buffer bytes, and return an offset.
///
/// The function fills in the `symbol` and return the offset to the desired offset for that symbol.  
/// The call back should return Err(..) if the buffer is not long enough to include the null termination.
/// If no symbolic information is located, the function returns Ok(None).
pub type Callback<T> = fn(addr: u64, symbol: &mut [u8], ctx: Option<&T>) -> Option<u64>;

#[derive(Clone, Debug, Default)]
pub struct Context<T> {
    callback: Option<Callback<T>>,
    context: Option<T>,
}

pub fn context<T>(callback: Option<Callback<T>>, context: Option<T>) -> Context<T> {
    Context { callback, context }
}

impl<T> Context<T> {
    pub fn callback(&self) -> ffi::xed_disassembly_callback_fn_t {
        if self.callback.is_some() {
            Some(callback_stub)
        } else {
            None
        }
    }

    pub fn context(&self) -> *mut c_void {
        if self.callback.is_some() {
            self as *const _ as *mut _
        } else {
            ptr::null_mut()
        }
    }
}

unsafe extern "C" fn callback_stub<T>(
    address: c_ulonglong,
    symbol_buffer: *mut c_char,
    buffer_length: c_uint,
    offset: *mut c_ulonglong,
    context: *mut T,
) -> c_int {
    let ctx = context.cast::<Context<T>>().as_ref().unwrap();

    if let Some(callback) = ctx.callback {
        match callback(
            address,
            slice::from_raw_parts_mut(symbol_buffer.cast(), buffer_length as usize),
            ctx.context.as_ref(),
        ) {
            Some(off) => {
                offset.write(off);
                1
            }
            _ => 0,
        }
    } else {
        0
    }
}
