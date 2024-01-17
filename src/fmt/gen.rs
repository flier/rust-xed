use std::mem::MaybeUninit;

use crate::raw::{AsPtr, ToBool};
use crate::{dec::Inst, ffi, Syntax};

use super::{disas, Options};

/// Disassemble the instruction information to a buffer.
pub struct Formatter {
    pi: ffi::xed_print_info_t,
    ctx: disas::Context<()>,
}

impl Formatter {
    pub fn new() -> Formatter {
        let pi = unsafe {
            let mut pi = MaybeUninit::zeroed();

            ffi::xed_init_print_info(pi.as_mut_ptr());

            pi.assume_init()
        };

        Formatter {
            pi,
            ctx: disas::Context::default(),
        }
    }

    /// program counter location.
    pub fn runtime_address(&mut self, addr: u64) -> &mut Self {
        self.pi.runtime_address = addr;
        self
    }

    pub fn disassembly_callback(
        &mut self,
        callback: disas::Callback<()>,
        context: Option<()>,
    ) -> &mut Self {
        self.ctx = disas::context(Some(callback), context);
        self.pi.disassembly_callback = self.ctx.callback();
        self.pi.context = self.ctx.context();
        self
    }

    pub fn syntax(&mut self, syntax: Syntax) -> &mut Self {
        self.pi.syntax = syntax.into();
        self
    }

    pub fn options(&mut self, opts: Options) -> &mut Self {
        self.pi.format_options_valid = 1;
        self.pi.format_options = opts.into();
        self
    }

    /// Disassemble the instruction information to a buffer.
    ///
    /// The output buffer must be at least 25 bytes long.
    pub fn format(&mut self, xedd: &Inst, buf: &mut [u8]) -> bool {
        self.pi.p = xedd.as_ptr();
        self.pi.buf = buf.as_mut_ptr().cast();
        self.pi.blen = buf.len() as i32;

        unsafe { ffi::xed_format_generic(&mut self.pi as *mut _) }.bool()
    }
}
