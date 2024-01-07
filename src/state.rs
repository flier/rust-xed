use std::{fmt, mem::MaybeUninit};

use derive_more::{From, Into};

use crate::{ffi, raw::AsMutPtr, AddressWidth, MachineMode};

/// Encapsulates machine modes for decoder/encoder requests.
///
/// It specifies the machine operating mode as a MachineMode for decoding and encoding.
/// The machine mode corresponds to the default data operand width for that mode.
/// For all modes other than the 64b long mode (MachineMode::Long64), a default addressing width,
/// and a stack addressing width must be supplied of type AddressWidth.
#[repr(transparent)]
#[derive(Clone, From, Into)]
pub struct State(ffi::xed_state_t);

impl_as_ptr!(State(ffi::xed_state_t));

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MachineMode: {} AddrWidth: {} StackAddrWidth: {}",
            self.machine_mode(),
            self.address_width(),
            self.stack_address_width()
        )
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("machine_mode", &self.machine_mode())
            .field("address_width", &self.address_width())
            .field("stack_address_width", &self.stack_address_width())
            .finish()
    }
}

impl State {
    properties! { prefix = xed_state;
        machine_mode: MachineMode {
            /// return the machine mode
            get;

            /// Set the machine mode which corresponds to the default data operand size
            set;
        }

        /// true iff the machine is in LONG_64 mode
        long64_mode: bool { xed_state_long64_mode }

        real_mode: bool { xed_state_real_mode }

        mode_width_16: bool { xed_state_mode_width_16 }

        mode_width_32: bool { xed_state_mode_width_32 }

        /// return the address width
        address_width: AddressWidth { get; }

        stack_address_width: AddressWidth {
            /// Return the STACK address width
            get;

            /// set the STACK address width
            set;
        }
    }

    /// Constructor.
    ///
    /// The mode, and addresses widths are enumerations that specify the number of bits.  
    /// In 64b mode (MachineMode::Long64) the address width and stack address widths are 64b (AddressWidth::Qword).
    /// In other machine modes, you must specify valid addressing widths.
    pub fn new(mode: MachineMode, addr_width: AddressWidth) -> Self {
        let mut s = MaybeUninit::zeroed();

        unsafe {
            ffi::xed_state_init2(s.as_mut_ptr(), mode.into(), addr_width.into());

            State(s.assume_init())
        }
    }

    /// clear the xed_state_t
    pub fn clear(&mut self) {
        unsafe { ffi::xed_state_zero(self.as_mut_ptr()) }
    }
}
