use std::mem::{self, MaybeUninit};

use derive_more::{Deref, DerefMut, From, Into};

use crate::{
    dec::Inst as DecodedInst,
    ffi,
    raw::{AsMutPtr, AsPtr, ToBool},
    Iclass, State,
};

use super::{Operand, Request};

#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, From, Into)]
pub struct Inst(ffi::xed_encoder_instruction_t);

impl_as_ptr!(Inst(ffi::xed_encoder_instruction_t));

impl Default for Inst {
    fn default() -> Self {
        Self(unsafe { mem::zeroed() })
    }
}

impl Inst {
    pub const MAX_BYTES: usize = ffi::XED_MAX_INSTRUCTION_BYTES as usize;

    pub fn mode(&self) -> State {
        self.0.mode.into()
    }

    /// specify effective address size different than the default.
    pub fn addr(&mut self, width_bits: u32) -> &mut Self {
        unsafe {
            ffi::xed_addr(self.as_mut_ptr(), width_bits);
        }

        self
    }

    /// To add a REP (0xF3) prefix.
    pub fn rep(&mut self) -> &mut Self {
        unsafe {
            ffi::xed_rep(self.as_mut_ptr());
        }

        self
    }

    /// To add a REPNE (0xF2) prefix.
    pub fn repne(&mut self) -> &mut Self {
        unsafe {
            ffi::xed_repne(self.as_mut_ptr());
        }

        self
    }

    /// convert a `Inst` to a `Request` for encoding
    pub fn request(&self) -> Option<Request<DecodedInst>> {
        let mut p = MaybeUninit::<Request<DecodedInst>>::zeroed();

        unsafe {
            ffi::xed_convert_to_encoder_request(p.as_mut_ptr().cast(), self.as_ptr().cast_mut())
        }
        .bool()
        .then(|| unsafe { p.assume_init() })
    }

    pub fn convert_to(&self, req: &mut Request<DecodedInst>) -> bool {
        unsafe {
            ffi::xed_convert_to_encoder_request(req.as_mut_ptr().cast(), self.as_ptr().cast_mut())
        }
        .bool()
    }

    /// instruction with no operands
    pub fn inst0<W>(&mut self, mode: State, iclass: Iclass, effective_operand_width: W) -> &mut Self
    where
        W: Into<Option<u32>>,
    {
        unsafe {
            ffi::xed_inst0(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
            );
        }

        self
    }

    /// instruction with one operand
    pub fn inst1<W, O>(
        &mut self,
        mode: State,
        iclass: Iclass,
        effective_operand_width: W,
        op: O,
    ) -> &mut Self
    where
        W: Into<Option<u32>>,
        O: Into<Operand>,
    {
        unsafe {
            ffi::xed_inst1(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
                op.into().into(),
            );
        }

        self
    }

    /// instruction with two operands
    pub fn inst2<W, O1, O2>(
        &mut self,
        mode: State,
        iclass: Iclass,
        effective_operand_width: W,
        op0: O1,
        op1: O2,
    ) -> &mut Self
    where
        W: Into<Option<u32>>,
        O1: Into<Operand>,
        O2: Into<Operand>,
    {
        unsafe {
            ffi::xed_inst2(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
                op0.into().into(),
                op1.into().into(),
            );
        }

        self
    }

    /// instruction with three operands
    pub fn inst3<W, O1, O2, O3>(
        &mut self,
        mode: State,
        iclass: Iclass,
        effective_operand_width: W,
        op0: O1,
        op1: O2,
        op2: O3,
    ) -> &mut Self
    where
        W: Into<Option<u32>>,
        O1: Into<Operand>,
        O2: Into<Operand>,
        O3: Into<Operand>,
    {
        unsafe {
            ffi::xed_inst3(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
                op0.into().into(),
                op1.into().into(),
                op2.into().into(),
            );
        }

        self
    }

    /// instruction with three operands
    #[allow(clippy::too_many_arguments)]
    pub fn inst4<W, O1, O2, O3, O4>(
        &mut self,
        mode: State,
        iclass: Iclass,
        effective_operand_width: W,
        op0: O1,
        op1: O2,
        op2: O3,
        op3: O4,
    ) -> &mut Self
    where
        W: Into<Option<u32>>,
        O1: Into<Operand>,
        O2: Into<Operand>,
        O3: Into<Operand>,
        O4: Into<Operand>,
    {
        unsafe {
            ffi::xed_inst4(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
                op0.into().into(),
                op1.into().into(),
                op2.into().into(),
                op3.into().into(),
            );
        }

        self
    }

    /// instruction with five operands
    #[allow(clippy::too_many_arguments)]
    pub fn inst5<W, O1, O2, O3, O4, O5>(
        &mut self,
        mode: State,
        iclass: Iclass,
        effective_operand_width: W,
        op0: O1,
        op1: O2,
        op2: O3,
        op3: O4,
        op4: O5,
    ) -> &mut Self
    where
        W: Into<Option<u32>>,
        O1: Into<Operand>,
        O2: Into<Operand>,
        O3: Into<Operand>,
        O4: Into<Operand>,
        O5: Into<Operand>,
    {
        unsafe {
            ffi::xed_inst5(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
                op0.into().into(),
                op1.into().into(),
                op2.into().into(),
                op3.into().into(),
                op4.into().into(),
            );
        }

        self
    }

    /// instruction with an array of operands.
    pub fn inst<W>(
        &mut self,
        mode: State,
        iclass: Iclass,
        effective_operand_width: W,
        ops: &[Operand],
    ) -> &mut Self
    where
        W: Into<Option<u32>>,
    {
        unsafe {
            ffi::xed_inst(
                self.as_mut_ptr(),
                mode.into(),
                iclass.into(),
                effective_operand_width.into().unwrap_or_default(),
                ops.len() as u32,
                ops.as_ptr().cast(),
            );
        }

        self
    }
}
