use std::{ffi::CStr, fmt, mem::MaybeUninit};

use derive_more::From;

use crate::{
    dec::Inst as DecodedInst,
    dec::Operands,
    ffi,
    raw::{AsMutPtr, AsPtr},
    Errno, Iclass, Op, Reg, Result, State,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, From)]
pub struct Request<I>(I);

impl AsPtr for Request<DecodedInst> {
    type CType = ffi::xed_encoder_request_t;

    fn as_ptr(&self) -> *const Self::CType {
        self.0.as_ptr().cast()
    }
}

impl AsMutPtr for Request<DecodedInst> {
    fn as_mut_ptr(&mut self) -> *mut Self::CType {
        self.0.as_mut_ptr().cast()
    }
}

impl<I> fmt::Display for Request<I>
where
    Self: AsPtr<CType = ffi::xed_encoder_request_t>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = MaybeUninit::<[u8; 4096]>::zeroed();

        unsafe {
            ffi::xed_encode_request_print(self.as_ptr(), buf.as_mut_ptr().cast(), 4096);

            let buf = buf.assume_init();

            f.write_str(
                &CStr::from_bytes_until_nul(&buf[..])
                    .unwrap()
                    .to_string_lossy(),
            )
        }
    }
}

impl<I> Request<I>
where
    Self: AsPtr<CType = ffi::xed_encoder_request_t>,
{
    properties! { prefix = xed_encoder_request ;
    }
}

impl<I> Request<I>
where
    Self: AsMutPtr<CType = ffi::xed_encoder_request_t>,
{
    properties! { prefix = xed_encoder_request ;
        iclass: Iclass { get; set; }

        effective_operand_width: u32 { set; }
        effective_address_size: u32 { set; }

        relbr { set; }
        ptr { set; }
        agen { set; }
        mem0 { set; }
        mem1 { set; }
        memory_operand_length: u32 { set; }
        seg0: Reg { set; }
        seg1: Reg { set; }
        base0: Reg { set; }
        base1: Reg { set; }
        index: Reg { set; }
        scale: u32 { set; }
    }

    pub fn clear(&mut self) {
        unsafe { ffi::xed_encoder_request_zero(self.as_mut_ptr()) }
    }

    pub fn reset<S: Into<State>>(&mut self, state: S) {
        unsafe { ffi::xed_encoder_request_zero_set_mode(self.as_mut_ptr(), state.into().as_ptr()) }
    }

    /// Set the operands array element indexed by operand to the actual register name reg.
    pub fn set_reg(&mut self, operand: Op, reg: Reg) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_reg(self.as_mut_ptr(), operand.into(), reg.into());
        }

        self
    }

    /// Retrieve the name of the n'th operand in the operand order.
    pub fn operand_order(&mut self, i: u32) -> Op {
        unsafe { ffi::xed_encoder_request_get_operand_order(self.as_mut_ptr(), i) }.into()
    }

    /// Specify the name as the n'th operand in the operand order.
    ///
    /// The complication of this function is that the register operand names are
    /// specific to the position of the operand (REG0, REG1, REG2...).
    /// One can use this function for registers or one can use the
    ///  xed_encoder_request_set_operand_name_reg() which takes integers instead of operand names.
    pub fn set_operand_order(&mut self, i: u32, name: Op) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_operand_order(self.as_mut_ptr(), i, name.into());
        }

        self
    }

    /// Retrieve the number of entries in the encoder operand order array
    pub fn operand_order_entries(&self) -> u32 {
        unsafe { ffi::xed_encoder_request_operand_order_entries(self.as_ptr().cast_mut()) }
    }

    /// clear the operand order array
    pub fn clear_operand_order(&mut self) {
        unsafe { ffi::xed_encoder_request_zero_operand_order(self.as_mut_ptr()) }
    }

    pub fn set_branch_displacement(&mut self, brdisp: i64, nbytes: u32) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_branch_displacement(self.as_mut_ptr(), brdisp, nbytes);
        }

        self
    }

    /// Set the uimm0 using a BYTE  width.
    pub fn uimm0(&mut self, uimm: u64, nbytes: u32) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_uimm0(self.as_mut_ptr(), uimm, nbytes);
        }

        self
    }

    /// Set the uimm0 using a BIT  width.
    pub fn set_uimm0_bits(&mut self, uimm: u64, nbits: u32) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_uimm0_bits(self.as_mut_ptr(), uimm, nbits);
        }

        self
    }

    pub fn set_uimm1(&mut self, uimm: u8) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_uimm1(self.as_mut_ptr(), uimm);
        }

        self
    }

    pub fn set_simm(&mut self, simm: i32, nbytes: u32) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_simm(self.as_mut_ptr(), simm, nbytes);
        }

        self
    }

    pub fn set_memory_displacement(&mut self, memdisp: i64, nbytes: u32) -> &mut Self {
        unsafe {
            ffi::xed_encoder_request_set_memory_displacement(self.as_mut_ptr(), memdisp, nbytes);
        }

        self
    }

    /// Encode the instruction to bytes
    pub fn encode(&mut self) -> Result<Vec<u8>> {
        let mut buf = MaybeUninit::<[u8; MAX_LEN]>::zeroed();
        let mut n = MaybeUninit::zeroed();

        let err = Errno::from(unsafe {
            ffi::xed_encode(
                self.as_mut_ptr(),
                buf.as_mut_ptr().cast(),
                MAX_LEN as u32,
                n.as_mut_ptr(),
            )
        });

        Result::from(err).map(|_| unsafe {
            let buf = buf.assume_init();
            let len = n.assume_init() as usize;

            buf[..len].into()
        })
    }
}

/// This function will attempt to encode a NOP of exactly bytes.
///
/// If such a NOP is not encodeable, then false will be returned.
pub fn nop(buf: &mut [u8]) -> Result<()> {
    Errno::from(unsafe { ffi::xed_encode_nop(buf.as_mut_ptr(), buf.len() as u32) }).into()
}

const MAX_LEN: usize = 15;

impl Request<&DecodedInst> {
    pub fn operands(&self) -> Operands<&DecodedInst> {
        self.0.into()
    }
}

impl Request<&mut DecodedInst> {
    pub fn operands_mut(&mut self) -> Operands<&mut DecodedInst> {
        self.0.into()
    }
}

impl Default for Request<DecodedInst> {
    fn default() -> Self {
        Self::new()
    }
}

impl Request<DecodedInst> {
    pub fn new() -> Self {
        Self(DecodedInst::new())
    }

    pub fn with_mode(state: State) -> Self {
        let mut req = Self::new();

        req.reset(state);
        req
    }
}
