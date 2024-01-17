use std::{ffi::CStr, mem::MaybeUninit};

use crate::{dec::Inst, ffi, raw::AsPtr, Syntax};

use super::disas;

/// Disassemble the decoded instruction using the specified syntax.
///
/// The output buffer must be at least 25 bytes long.
pub fn context<T>(
    syntax: Syntax,
    xedd: &Inst,
    runtime_instruction_address: Option<u64>,
    context: Option<T>,
    symbolic_callback: Option<disas::Callback<T>>,
) -> Option<String> {
    let mut buf = MaybeUninit::<[u8; 1024]>::zeroed();
    let ctx = disas::context(symbolic_callback, context);

    let res = unsafe {
        ffi::xed_format_context(
            syntax.into(),
            xedd.as_ptr(),
            buf.as_mut_ptr().cast(),
            1024,
            runtime_instruction_address.unwrap_or_default(),
            ctx.context(),
            ctx.callback(),
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
