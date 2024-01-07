use std::mem;

use crate::ffi;

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
