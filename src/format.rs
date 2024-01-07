use std::{
    ffi::CStr,
    mem::{self, MaybeUninit},
    ptr, slice,
};

use crate::{dec::Inst, ffi, raw::AsPtr, Syntax};

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Options(ffi::xed_format_options_t);

impl Default for Options {
    fn default() -> Self {
        Self(unsafe { mem::zeroed() })
    }
}

impl Options {
    /// XED prints the hex address before any symbolic name for branch targets.
    ///
    /// If set to false, then XED will not print the hex address before a valid symbolic name.
    pub fn hex_address_before_symbolic_name(&mut self) -> &mut Self {
        self.0.hex_address_before_symbolic_name = 1;
        self
    }

    /// Simple XML output format for the Intel syntax disassembly.
    pub fn xml_output(&mut self) -> &mut Self {
        self.0.xml_a = 1;
        self
    }

    /// Include flags in the XML formatting (must also supply xml_a)
    pub fn xml_flags(&mut self) -> &mut Self {
        self.0.xml_a = 1;
        self.0.xml_f = 1;
        self
    }

    /// omit unit scale "*1"
    pub fn omit_unit_scale(&mut self) -> &mut Self {
        self.0.omit_unit_scale = 1;
        self
    }

    /// do not sign extend signed immediates
    pub fn no_sign_extend_signed_immediates(&mut self) -> &mut Self {
        self.0.no_sign_extend_signed_immediates = 1;
        self
    }

    /// write-mask-with-curly-brackets, omit k0
    pub fn write_mask_curly_k0(&mut self) -> &mut Self {
        self.0.write_mask_curly_k0 = 1;
        self
    }

    /// lowercase hexadecimal
    pub fn lowercase_hex(&mut self) -> &mut Self {
        self.0.lowercase_hex = 1;
        self
    }

    /// Show negative memory displacements as positive numbers.
    pub fn positive_memory_displacements(&mut self) -> &mut Self {
        self.0.positive_memory_displacements = 1;
        self
    }

    /// Customize the disassembly formatting options by passing  in a `Options` structure.
    pub fn apply(self) {
        unsafe { ffi::xed_format_set_options(self.0) }
    }
}

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
