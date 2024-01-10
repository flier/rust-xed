use crate::{dec::Inst, ffi, indexed_properties, Reg};

#[derive(Clone, Copy, Debug)]
pub struct MemOperand<I>(pub(crate) I, pub(crate) u32);

impl<I> From<(I, u32)> for MemOperand<I> {
    fn from((inst, idx): (I, u32)) -> Self {
        MemOperand(inst, idx)
    }
}

impl MemOperand<&Inst> {
    indexed_properties! {
        seg: Reg? { xed_decoded_inst_get_seg_reg }
        base: Reg? { xed_decoded_inst_get_base_reg }
        index: Reg? { xed_decoded_inst_get_index_reg }
        scale: u32? { xed_decoded_inst_get_scale }

        memory_displacement: i64? { xed_decoded_inst_get_memory_displacement }
        memory_displacement_width: u32? { xed_decoded_inst_get_memory_displacement_width }
        memory_displacement_width_bits: u32? { xed_decoded_inst_get_memory_displacement_width_bits }

        read: bool { xed_decoded_inst_mem_read }
        written: bool { xed_decoded_inst_mem_written }
        written_only: bool { xed_decoded_inst_mem_written_only }

        length: u32 { xed_decoded_inst_get_memory_operand_length }

        /// Returns the addressing width in bits (16,32,64) for MEM0 (memop_idx==0) or MEM1 (memop_idx==1).
        ///
        /// This factors in things like whether or not the reference is an implicit stack push/pop reference,
        /// the machine mode and 67 prefixes if present.
        address_width: u32 { xed_decoded_inst_get_memop_address_width }
    }
}
