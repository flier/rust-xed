use crate::{ffi, indexed_properties, raw::AsPtr, DecodedInst, OperandAction, OperandElementType};

#[derive(Clone, Copy, Debug)]
pub struct Operand<I>(I, u32);

impl<I> From<(I, u32)> for Operand<I> {
    fn from((inst, idx): (I, u32)) -> Self {
        Operand(inst, idx)
    }
}

impl Operand<&DecodedInst> {
    indexed_properties! {
        /// Interpret the operand action in light of AVX512 masking and zeroing/merging.
        action: OperandAction { xed_decoded_inst_operand_action }

        /// Return the length in bits of the  self.idx'th operand.
        length_bits: u32 { xed_decoded_inst_operand_length_bits }

        /// Return the number of element in the operand (for SSE and AVX operands)
        elements: u32 { xed_decoded_inst_operand_elements }

        /// Return the size of an element in bits  (for SSE and AVX operands)
        element_size_bits: u32 { xed_decoded_inst_operand_element_size_bits }

        /// Return the type of an element of type #xed_operand_element_type_enum_t (for SSE and AVX operands)
        element_type: OperandElementType { xed_decoded_inst_operand_element_type }
    }
}
