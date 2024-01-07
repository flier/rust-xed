use derive_more::From;

use crate::{
    ffi, properties,
    raw::{AsMutPtr, AsPtr},
    DecodedInst, Iclass, Reg,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, From)]
pub struct Operands<I>(I);

impl AsPtr for Operands<&DecodedInst> {
    type CType = ffi::xed_operand_values_t;

    fn as_ptr(&self) -> *const Self::CType {
        self.0.as_ptr().cast()
    }
}

impl AsPtr for Operands<&mut DecodedInst> {
    type CType = ffi::xed_operand_values_t;

    fn as_ptr(&self) -> *const Self::CType {
        self.0.as_ptr().cast()
    }
}

impl AsMutPtr for Operands<&mut DecodedInst> {
    fn as_mut_ptr(&mut self) -> *mut Self::CType {
        self.0.as_mut_ptr().cast()
    }
}

impl<I> Operands<I>
where
    Self: AsPtr<CType = ffi::xed_operand_values_t>,
{
    properties! {
        /// True if the instruction has a real REP prefix.
        ///
        /// This returns false if there is no F2/F3 prefix or
        /// the F2/F3 prefix is used to refine the opcode as in some SSE operations.
        has_real_rep: bool { xed_operand_values_has_real_rep }

        /// True if the instruction as a F3 REP prefix
        ///
        /// used for opcode refining, for rep for string operations, or ignored.
        has_rep_prefix: bool { xed_operand_values_has_rep_prefix }

        /// True if the instruction as a F2 REP prefix
        ///
        /// used for opcode refining, for rep for string operations, or ignored.
        has_repne_prefix: bool { xed_operand_values_has_repne_prefix }

        /// Returns true if the memory operation has atomic read-modify-write semantics.
        atomic: bool { xed_operand_values_get_atomic }

        /// Returns true if the memory operation has a valid lock prefix.
        has_lock_prefix: bool { xed_operand_values_has_lock_prefix }

        /// Returns true if the instruction could be re-encoded to have a lock prefix but does not have one currently.
        lockable: bool { xed_operand_values_lockable }

        /// Returns The effective operand width in bits: 16/32/64.
        effective_operand_width: u32 { xed_operand_values_get_effective_operand_width }

        /// Returns The effective address width in bits: 16/32/64.
        effective_address_width: u32 { xed_operand_values_get_effective_address_width }

        /// Returns The stack address width in bits: 16/32/64.
        stack_address_width: u32 { xed_operand_values_get_stack_address_width }

        /// True if there is a memory displacement
        has_memory_displacement: bool { xed_operand_values_has_memory_displacement }

        /// True if there is a branch displacement
        has_branch_displacement: bool { xed_operand_values_has_branch_displacement }

        /// True if there is a memory or branch displacement
        has_displacement: bool { xed_operand_values_has_displacement }

        /// Return true if there is an immediate operand
        has_immediate: bool { xed_operand_values_has_immediate }

        /// This indicates the presence of a 67 prefix.
        has_address_size_prefix: bool { xed_operand_values_has_address_size_prefix }

        /// This does not include the cases when the 66 prefix is used an opcode-refining prefix for multibyte opcodes.
        has_operand_size_prefix: bool { xed_operand_values_has_operand_size_prefix }

        /// This includes any 66 prefix that shows up even if it is ignored.
        has_66_prefix: bool { xed_operand_values_has_66_prefix }

        /// This instruction has a REX prefix with the W bit set.
        has_rexw_prefix: bool { xed_operand_values_has_rexw_prefix }

        has_segment_prefix: bool { xed_operand_values_has_segment_prefix }

        /// Return the segment prefix, if any, as a #xed_reg_enum_t value.
        segment_prefix: Reg { xed_operand_values_segment_prefix }

        long_mode: bool { xed_operand_values_get_long_mode }

        real_mode: bool { xed_operand_values_get_real_mode }

        accesses_memory: bool { xed_operand_values_accesses_memory }

        memory_operands: u32 { xed_operand_values_number_of_memory_operands }

        scale: u32 { xed_operand_values_get_scale }

        /// Returns true if the instruction access memory but without using a MODRM byte limiting its addressing modes.
        memop_without_modrm: bool { xed_operand_values_memop_without_modrm }

        /// Returns true if the instruction has a MODRM byte.
        has_modrm_byte: bool { xed_operand_values_has_modrm_byte }

        /// Returns true if the instruction has a SIB byte.
        has_sib_byte: bool { xed_operand_values_has_sib_byte }

        /// Returns  true if 0x2E prefix on Jcc
        branch_not_taken_hint: bool { xed_operand_values_branch_not_taken_hint }

        /// Returns  true if 0x3E prefix on Jcc
        branch_taken_hint: bool { xed_operand_values_branch_taken_hint }

        /// Returns true for indirect call/jmp with 0x3E prefix (if the legacy prefix rules are obeyed)
        cet_no_track: bool { xed_operand_values_cet_no_track }

        is_nop: bool { xed_operand_values_is_nop }

        immediate_int64: i64 { xed_operand_values_get_immediate_int64 }

        immediate_uint64: u64 { xed_operand_values_get_immediate_uint64 }

        /// Return true if the first immediate (IMM0) is signed
        immediate_is_signed: bool { xed_operand_values_get_immediate_is_signed }

        second_immediate: u8 { xed_operand_values_get_second_immediate }

        /// Return the memory displacement width in BYTES
        memory_displacement_length: u32 { xed_operand_values_get_memory_displacement_length }

        /// Return the memory displacement width in BITS
        memory_displacement_length_bits: u32 { xed_operand_values_get_memory_displacement_length_bits }

        /// Return the raw memory displacement width in BITS(ignores scaling)
        memory_displacement_length_bits_raw: u32 { xed_operand_values_get_memory_displacement_length_bits_raw }

        /// Returns the potentially scaled value of the memory displacement.
        ///
        /// Certain AVX512 memory displacements are scaled before they are used.
        memory_displacement_int64: i64 { xed_operand_values_get_memory_displacement_int64 }

        /// Returns the unscaled (raw) memory displacement.
        ///
        /// Certain AVX512 memory displacements are scaled before they are used.
        memory_displacement_int64_raw: i64 { xed_operand_values_get_memory_displacement_int64_raw }

        /// Return the branch displacement width in bytes
        branch_displacement_length: u32 { xed_operand_values_get_branch_displacement_length }

        /// Return the branch displacement width in bits
        branch_displacement_length_bits: u32 { xed_operand_values_get_branch_displacement_length_bits }

        branch_displacement_int32: i32 { xed_operand_values_get_branch_displacement_int32 }

        iclass: Iclass { xed_operand_values_get_iclass }
    }
}

impl<I> Operands<I>
where
    Self: AsMutPtr<CType = ffi::xed_operand_values_t>,
{
    properties! {}
}
