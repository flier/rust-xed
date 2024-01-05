use std::{
    mem::MaybeUninit,
    ops::{Index, Range},
    slice,
};

use crate::{
    decoded::{Operand, Operands},
    ffi, properties,
    raw::{AsMutPtr, AsPtr, ToBool},
    AddressWidth, Attribute, Category, Chip, Errno, Extension, Iclass, Iform, Inst as InstTemplate,
    IsaSet, MachineMode, Op, Reg, Result, SimpleFlag,
};

use super::mem::MemOperand;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Inst(ffi::xed_decoded_inst_t);

impl Default for Inst {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Inst {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { self.as_bytes().get_unchecked(i) }
    }
}

impl AsPtr for Inst {
    type CType = ffi::xed_decoded_inst_t;

    fn as_ptr(&self) -> *const Self::CType {
        &self.0 as *const _
    }
}

impl AsMutPtr for Inst {
    fn as_mut_ptr(&mut self) -> *mut Self::CType {
        &mut self.0 as *mut _
    }
}

impl Inst {
    properties! {
        /// Return true if the instruction is valid
        valid: bool { xed_decoded_inst_valid }

        // Return the user-specified `Chip` name
        chip: Chip? { xed_decoded_inst_get_input_chip }

        category: Category { xed_decoded_inst_get_category }
        extension: Extension { xed_decoded_inst_get_extension }
        isa_set: IsaSet { xed_decoded_inst_get_isa_set }
        iclass: Iclass { xed_decoded_inst_get_iclass }

        /// Returns true if the instruction is xacquire.
        is_xacquire: bool { xed_decoded_inst_is_xacquire }

        /// Returns true if the instruction is xrelease.
        is_xrelease: bool { xed_decoded_inst_is_xrelease }

        /// Returns true if the instruction has mpx prefix.
        has_mpx_prefix: bool { xed_decoded_inst_has_mpx_prefix }

        /// Returns the modrm byte
        modrm: u8 { xed_decoded_inst_get_modrm }

        /// Returns 128, 256 or 512 for operations in the VEX, EVEX (or XOP)
        /// encoding space and returns 0 for (most) nonvector operations.
        ///
        /// This usually the content of the VEX.L or EVEX.LL field, reinterpreted.
        /// Some GPR instructions (like the BMI1/BMI2) are encoded in the VEX space
        /// and return non-zero values from this API.
        vector_length_bits: u32 { xed_decoded_inst_vector_length_bits }

        /// Returns the number of legacy prefixes.
        nprefixes: u32 { xed_decoded_inst_get_nprefixes }

        /// Return the number of operands
        noperands: u32 { xed_decoded_inst_noperands }

        /// The instruction uses write-masking
        masking: bool { xed_decoded_inst_masking }

        /// The instruction uses write-masking with merging
        merging: bool { xed_decoded_inst_merging }

        /// The instruction uses write-masking with zeroing
        zeroing: bool { xed_decoded_inst_zeroing }

        /// Returns the maximum number elements processed for an AVX512 vector instruction.
        ///
        /// Scalars report 1 element.
        avx512_dest_elements: u32 { xed_decoded_inst_avx512_dest_elements }

        /// Return the length of the decoded instruction in bytes.
        length: u32 { xed_decoded_inst_get_length }

        /// Returns 16/32/64 indicating the machine mode with in bits.
        ///
        /// This is derived from the input mode information.
        machine_mode_bits: u32 { xed_decoded_inst_get_machine_mode_bits }

        /// Returns 16/32/64 indicating the stack addressing mode with in bits.
        ///
        /// This is derived from the input mode information.
        stack_address_mode_bits: u32 { xed_decoded_inst_get_stack_address_mode_bits }

        /// Returns the operand width in bits: 8/16/32/64.
        operand_width: u32 { xed_decoded_inst_get_operand_width }

        /// Return the user-specified `chip` name
        input_chip: Chip? { xed_decoded_inst_get_input_chip }

        iform: Iform { xed_decoded_inst_get_iform_enum }

        /// Return the instruction zero-based iform number based on masking the corresponding `Iform`.
        iform_dispatch: u32 { xed_decoded_inst_get_iform_enum_dispatch }

        branch_displacement: i32 { xed_decoded_inst_get_branch_displacement }
        branch_displacement_width: u32 { xed_decoded_inst_get_branch_displacement_width }

        unsigned_immediate: u64 { xed_decoded_inst_get_unsigned_immediate }

        /// Return true if the first immediate (IMM0)  is signed
        immediate_is_signed: bool { xed_decoded_inst_get_immediate_is_signed }

        /// Return the immediate width in BYTES.
        immediate_width: u32 { xed_decoded_inst_get_immediate_width }

        /// Return the immediate width in BITS.
        immediate_width_bits: u32 { xed_decoded_inst_get_immediate_width_bits }

        signed_immediate: i32 { xed_decoded_inst_get_signed_immediate }
        second_immediate: u8 { xed_decoded_inst_get_second_immediate }

        /// See the comment on xed_decoded_inst_uses_rflags().
        rflags_info: &SimpleFlag? { xed_decoded_inst_get_rflags_info }

        /// This returns true if the flags are read or written.
        uses_rflags: bool { xed_decoded_inst_uses_rflags }

        number_of_memory_operands: u32 { xed_decoded_inst_number_of_memory_operands }

        conditionally_writes_registers: bool { xed_decoded_inst_conditionally_writes_registers }

        /// Returns true if the instruction is a prefetch
        is_prefetch: bool { xed_decoded_inst_is_prefetch }

        /// Return true for broadcast instructions or AVX512 load-op instructions using the broadcast feature
        is_broadcast: bool { xed_decoded_inst_is_broadcast }

        /// Return true for broadcast instruction. (NOT including AVX512 load-op instructions)
        is_broadcast_instruction: bool { xed_decoded_inst_is_broadcast_instruction }

        /// Return true for AVX512 load-op instructions using the broadcast feature,
        uses_embedded_broadcast: bool { xed_decoded_inst_uses_embedded_broadcast }

        /// True for AVX512 (EVEX-encoded) SIMD and (VEX encoded) K-mask instructions
        avx512: bool { xed_classify_avx512 }

        /// True for AVX512 (VEX-encoded) K-mask operations
        avx512_maskop: bool { xed_classify_avx512_maskop }

        /// True for AVX/AVX2 SIMD VEX-encoded operations. Does not include BMI/BMI2 instructions.
        avx: bool { xed_classify_avx }

        /// True for SSE/SSE2/etc. SIMD operations.  Includes AES and PCLMULQDQ
        sse: bool { xed_classify_sse }
    }

    /// Create and zero the decode structure completely.
    pub fn new() -> Self {
        let mut inst = MaybeUninit::zeroed();

        Self(unsafe {
            ffi::xed_decoded_inst_zero(inst.as_mut_ptr());

            inst.assume_init()
        })
    }

    /// Create with the machine mode and stack addressing width directly.
    pub fn with_mode(mode: MachineMode, width: AddressWidth) -> Self {
        let mut inst = Self::new();

        inst.set_mode(mode, width);

        inst
    }

    /// Set the machine mode and stack addressing width directly.
    pub fn set_mode(&mut self, mode: MachineMode, width: AddressWidth) {
        unsafe { ffi::xed_decoded_inst_set_mode(self.as_mut_ptr(), mode.into(), width.into()) }
    }

    // Set a user-specified `Chip` name for restricting decode
    pub fn set_chip(&mut self, chip: Chip) {
        unsafe { ffi::xed_decoded_inst_set_input_chip(self.as_mut_ptr(), chip.into()) }
    }

    /// Indicate if this decoded instruction is valid for the specified `Chip`
    pub fn valid_for_chip(&self, chip: Chip) -> bool {
        unsafe { ffi::xed_decoded_inst_valid_for_chip(self.as_ptr(), chip.into()) }.bool()
    }

    /// Return the `Inst` for this instruction.
    ///
    /// This is the route to the basic operands form information.
    pub fn inst(&self) -> &InstTemplate {
        unsafe {
            ffi::xed_decoded_inst_inst(self.as_ptr())
                .cast::<InstTemplate>()
                .as_ref()
                .unwrap()
        }
    }

    /// Returns true if the attribute is defined for this instruction.
    pub fn has_attr(&self, attr: Attribute) -> bool {
        unsafe { ffi::xed_decoded_inst_get_attribute(self.as_ptr(), attr.into()) != 0 }
    }

    /// Returns the attribute bitvector
    pub fn attrs(&self) -> Range<u64> {
        let attrs = unsafe { ffi::xed_decoded_inst_get_attributes(self.as_ptr()) };

        attrs.a1..attrs.a2
    }

    /// Obtain a constant reference to the operands
    pub fn operands(&self) -> Operands<&Inst> {
        self.into()
    }

    /// Obtain a non-constant reference to the operands
    pub fn operands_mut(&mut self) -> Operands<&mut Inst> {
        self.into()
    }

    pub fn operand(&self, i: u32) -> Option<Operand<&Inst>> {
        if i < self.noperands() {
            Some((self, i).into())
        } else {
            None
        }
    }

    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0._byte_array._dec, self.0._decoded_length as usize) }
    }

    /// Read itext byte.
    pub fn get(&self, i: u32) -> Option<u8> {
        if i < self.length() {
            Some(unsafe { ffi::xed_decoded_inst_get_byte(self.as_ptr(), i) })
        } else {
            None
        }
    }

    /// Return a user data field for arbitrary use by the user after decoding.
    pub fn user_data(&self) -> Option<u64> {
        let n = unsafe { ffi::xed_decoded_inst_get_user_data(self.as_ptr() as *mut _) };

        (n != 0).then_some(n)
    }

    /// Modify the user data field.
    pub fn set_user_data(&mut self, n: u64) {
        unsafe { ffi::xed_decoded_inst_set_user_data(self.as_mut_ptr(), n) }
    }

    /// Returns true if the instruction uses destination-masking.
    ///
    /// This is false for blend operations that use their mask field as a control.
    pub fn masked_vector_operation(&self) -> bool {
        unsafe { ffi::xed_decoded_inst_masked_vector_operation(self.as_ptr() as *mut _) }.bool()
    }

    pub fn mem_operands(&self) -> impl Iterator<Item = MemOperand<&Inst>> {
        (0..self.number_of_memory_operands()).map(move |idx| (self, idx).into())
    }

    pub fn mem_operand(&self, i: u32) -> Option<MemOperand<&Inst>> {
        (i < self.number_of_memory_operands()).then(|| (self, i).into())
    }

    pub fn reg(&self, op: Op) -> Reg {
        unsafe { ffi::xed_decoded_inst_get_reg(self.as_ptr(), op.into()) }.into()
    }

    pub fn decode<T: AsRef<[u8]>>(&mut self, bytes: T) -> Result<()> {
        let b = bytes.as_ref();

        let errno =
            Errno::from(unsafe { ffi::xed_decode(self.as_mut_ptr(), b.as_ptr(), b.len() as u32) });

        if errno.is_none() {
            Ok(())
        } else {
            Err(errno.into())
        }
    }
}
