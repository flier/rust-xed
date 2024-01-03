use crate::{
    ffi, gen, Nonterminal, OperandAction, OperandElementXtype, OperandType, OperandVisibility,
    OperandWidth, Reg,
};

/// Constant information about an individual generic operand, like an operand template, describing the operand properties.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Operand(ffi::xed_operand_t);

impl Operand {
    fn as_ptr(&self) -> *const ffi::xed_operand_t {
        &self.0 as *const _
    }

    /// Template Operands Access
    pub fn name(&self) -> gen::Operand {
        unsafe { ffi::xed_operand_name(self.as_ptr()) }.into()
    }

    /// Template Operand Read/Written
    pub fn action(&self) -> OperandAction {
        unsafe { ffi::xed_operand_rw(self.as_ptr()) }.into()
    }

    pub fn visibility(&self) -> OperandVisibility {
        unsafe { ffi::xed_operand_operand_visibility(self.as_ptr()) }.into()
    }

    pub fn ty(&self) -> OperandType {
        unsafe { ffi::xed_operand_type(self.as_ptr()) }.into()
    }

    pub fn xtype(&self) -> OperandElementXtype {
        unsafe { ffi::xed_operand_xtype(self.as_ptr()) }.into()
    }

    pub fn width(&self) -> OperandWidth {
        unsafe { ffi::xed_operand_width(self.as_ptr()) }.into()
    }

    pub fn width_bits(&self, eosz: u32) -> u32 {
        unsafe { ffi::xed_operand_width_bits(self.as_ptr(), eosz) }
    }

    pub fn nonterminal(&self) -> Option<Nonterminal> {
        let nt = unsafe { ffi::xed_operand_nonterminal_name(self.as_ptr()) }.into();

        (nt != Nonterminal::INVALID).then_some(nt)
    }

    pub fn reg(&self) -> Option<Reg> {
        let reg = unsafe { ffi::xed_operand_reg(self.as_ptr()) }.into();

        (reg != Reg::INVALID).then_some(reg)
    }

    pub fn template_is_register(&self) -> bool {
        unsafe { ffi::xed_operand_template_is_register(self.as_ptr()) != 0 }
    }

    pub fn imm(&self) -> Option<u32> {
        let imm = unsafe { ffi::xed_operand_imm(self.as_ptr()) };

        (imm == 0).then_some(imm)
    }
}
