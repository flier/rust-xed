use crate::{
    ffi, properties, raw::AsPtr, Nonterminal, Op, OperandAction, OperandElementXtype, OperandType,
    OperandVisibility, OperandWidth, Reg,
};

/// Constant information about an individual generic operand, like an operand template, describing the operand properties.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Operand(ffi::xed_operand_t);

impl_as_ptr!(Operand(ffi::xed_operand_t));

impl Operand {
    properties! {

        /// Template Operands Access
        name: Op { xed_operand_name }

        /// Template Operand Read/Written
        action: OperandAction { xed_operand_rw }

        visibility: OperandVisibility { xed_operand_operand_visibility }

        ty: OperandType { xed_operand_type }

        xtype: OperandElementXtype { xed_operand_xtype }

        width: OperandWidth { xed_operand_width }

        nonterminal: Nonterminal? { xed_operand_nonterminal_name }

        reg: Reg? { xed_operand_reg }

        template_is_register: bool { xed_operand_template_is_register }

        imm: u32? { xed_operand_imm }
    }

    pub fn width_bits(&self, eosz: u32) -> u32 {
        unsafe { ffi::xed_operand_width_bits(self.as_ptr(), eosz) }
    }
}
