use std::{ffi::CStr, mem::MaybeUninit, ptr, slice};

use crate::{dec::Inst, ffi, raw::AsPtr, Syntax};

/// A `DisassemblyCallback` takes an address, a mutable reference to a symbol buffer bytes, and return an offset.
///
/// The function fills in the `symbol` and return the offset to the desired offset for that symbol.  
/// The call back should return Err(..) if the buffer is not long enough to include the null termination.
/// If no symbolic information is located, the function returns Ok(None).
pub type DisassemblyCallback<T> = fn(addr: u64, symbol: &mut [u8], ctx: Option<&T>) -> Option<u64>;

/// Disassemble the decoded instruction using the specified syntax.
///
/// The output buffer must be at least 25 bytes long.
pub fn context<T>(
    syntax: Syntax,
    xedd: &Inst,
    runtime_instruction_address: Option<u64>,
    context: Option<T>,
    symbolic_callback: Option<DisassemblyCallback<T>>,
) -> Option<String> {
    let mut buf = MaybeUninit::<[u8; 1024]>::zeroed();
    let ctx = symbolic_callback.map(|callback| DisassemblyCallbackContext { callback, context });

    let res = unsafe {
        ffi::xed_format_context(
            syntax.into(),
            xedd.as_ptr(),
            buf.as_mut_ptr().cast(),
            1024,
            runtime_instruction_address.unwrap_or_default(),
            ctx.map_or_else(ptr::null_mut, |v| &v as *const _ as *mut _),
            if symbolic_callback.is_some() {
                Some(symbolic_callback_stub::<T>)
            } else {
                None
            },
        )
    };

    (res != 0).then(|| {
        let buf = unsafe { buf.assume_init() };

        CStr::from_bytes_until_nul(&buf[..])
            .unwrap()
            .to_string_lossy()
            .to_string()
    })
}

unsafe extern "C" fn symbolic_callback_stub<T>(
    address: ::std::os::raw::c_ulonglong,
    symbol_buffer: *mut ::std::os::raw::c_char,
    buffer_length: ::std::os::raw::c_uint,
    offset: *mut ::std::os::raw::c_ulonglong,
    context: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let ctx = context
        .cast::<DisassemblyCallbackContext<T>>()
        .as_ref()
        .unwrap();

    let callback = ctx.callback;
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
}

struct DisassemblyCallbackContext<T> {
    callback: DisassemblyCallback<T>,
    context: Option<T>,
}
