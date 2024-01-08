use std::{
    ffi::CStr,
    fmt,
    mem::MaybeUninit,
    ops::{Index, Range},
    slice,
};

use derive_more::{From, Into};

use crate::{
    chip::Features,
    dec::{Operand, Operands},
    ffi, properties,
    raw::{AsMutPtr, AsPtr, ToBool},
    AddressWidth, Attribute, Category, Chip, Errno, Extension, Iclass, Iform, Inst as InstTemplate,
    IsaSet, MachineMode, Op, Reg, Result, SimpleFlag, State,
};

use super::mem::MemOperand;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, From, Into)]
pub struct Inst(ffi::xed_decoded_inst_t);

impl_as_ptr!(Inst(ffi::xed_decoded_inst_t));

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

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = MaybeUninit::<[u8; 4096]>::zeroed();

        unsafe {
            ffi::xed_decoded_inst_dump(self.as_ptr(), buf.as_mut_ptr().cast(), 4096);

            let buf = buf.assume_init();

            f.write_str(
                &CStr::from_bytes_until_nul(&buf[..])
                    .unwrap()
                    .to_string_lossy(),
            )
        }
    }
}

impl Inst {
    properties! { prefix = xed_decoded_inst;
        /// Return true if the instruction is valid
        valid: bool

        input_chip: Chip? {
            // Return the user-specified `Chip` name
            get;
            // Set a user-specified `Chip` name for restricting decode
            set;
        }

        category: Category { get; }
        extension: Extension { get; }
        isa_set: IsaSet { get; }
        iclass: Iclass { get; }

        /// Returns true if the instruction is xacquire.
        is_xacquire: bool

        /// Returns true if the instruction is xrelease.
        is_xrelease: bool

        /// Returns true if the instruction has mpx prefix.
        has_mpx_prefix: bool

        /// Returns the modrm byte
        modrm: u8 { get; }

        /// Returns 128, 256 or 512 for operations in the VEX, EVEX (or XOP)
        /// encoding space and returns 0 for (most) nonvector operations.
        ///
        /// This usually the content of the VEX.L or EVEX.LL field, reinterpreted.
        /// Some GPR instructions (like the BMI1/BMI2) are encoded in the VEX space
        /// and return non-zero values from this API.
        vector_length_bits: u32

        /// Returns the number of legacy prefixes.
        nprefixes: u32 { get; }

        /// Return the number of operands
        noperands: u32

        /// The instruction uses write-masking
        masking: bool

        /// The instruction uses write-masking with merging
        merging: bool

        /// The instruction uses write-masking with zeroing
        zeroing: bool

        /// Returns the maximum number elements processed for an AVX512 vector instruction.
        ///
        /// Scalars report 1 element.
        avx512_dest_elements: u32

        /// Return the length of the decoded instruction in bytes.
        length: u32 { get; }

        /// Returns 16/32/64 indicating the machine mode with in bits.
        ///
        /// This is derived from the input mode information.
        machine_mode_bits: u32 { get; }

        /// Returns 16/32/64 indicating the stack addressing mode with in bits.
        ///
        /// This is derived from the input mode information.
        stack_address_mode_bits: u32 { get; }

        /// Returns the operand width in bits: 8/16/32/64.
        operand_width: u32 { get; }

        iform: Iform { xed_decoded_inst_get_iform_enum }

        /// Return the instruction zero-based iform number based on masking the corresponding `Iform`.
        iform_dispatch: u32 { xed_decoded_inst_get_iform_enum_dispatch }

        branch_displacement: i32 { get; }
        branch_displacement_width: u32 { get; }

        unsigned_immediate: u64 { get; }

        /// Return true if the first immediate (IMM0)  is signed
        immediate_is_signed: bool { get; }

        /// Return the immediate width in BYTES.
        immediate_width: u32 { get; }

        /// Return the immediate width in BITS.
        immediate_width_bits: u32 { get; }

        signed_immediate: i32 { get; }
        second_immediate: u8 { get; }

        /// See the comment on xed_decoded_inst_uses_rflags().
        rflags_info: &SimpleFlag? { get; }

        /// This returns true if the flags are read or written.
        uses_rflags: bool

        number_of_memory_operands: u32

        conditionally_writes_registers: bool

        /// Returns true if the instruction is a prefetch
        is_prefetch: bool

        /// Return true for broadcast instructions or AVX512 load-op instructions using the broadcast feature
        is_broadcast: bool

        /// Return true for broadcast instruction. (NOT including AVX512 load-op instructions)
        is_broadcast_instruction: bool

        /// Return true for AVX512 load-op instructions using the broadcast feature,
        uses_embedded_broadcast: bool
    }

    properties! { prefix = xed_classify;
        /// True for AVX512 (EVEX-encoded) SIMD and (VEX encoded) K-mask instructions
        avx512: bool

        /// True for AVX512 (VEX-encoded) K-mask operations
        avx512_maskop: bool

        /// True for AVX/AVX2 SIMD VEX-encoded operations. Does not include BMI/BMI2 instructions.
        avx: bool

        /// True for SSE/SSE2/etc. SIMD operations.  Includes AES and PCLMULQDQ
        sse: bool
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

    /// Zero the decode structure, but set the machine state/mode information.
    ///
    /// Re-initializes all operands.
    pub fn reset<S: Into<State>>(&mut self, state: S) {
        unsafe { ffi::xed_decoded_inst_zero_set_mode(self.as_mut_ptr(), state.into().as_ptr()) }
    }

    /// Set the machine mode and stack addressing width directly.
    pub fn set_mode(&mut self, mode: MachineMode, width: AddressWidth) {
        unsafe { ffi::xed_decoded_inst_set_mode(self.as_mut_ptr(), mode.into(), width.into()) }
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
        let n = unsafe { ffi::xed_decoded_inst_get_user_data(self.as_ptr().cast_mut()) };

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
        unsafe { ffi::xed_decoded_inst_masked_vector_operation(self.as_ptr().cast_mut()) }.bool()
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

        Errno::from(unsafe { ffi::xed_decode(self.as_mut_ptr(), b.as_ptr(), b.len() as u32) })
            .into()
    }

    /// Decode bytes to instruction
    ///
    /// This version of the decode API adds a CPUID feature vector
    /// to support restricting decode based on both a specified chip via set_input_chip and a modify-able cpuid feature
    /// vector obtained from #xed_get_chip_features().
    pub fn decode_with_features<T: AsRef<[u8]>>(
        &mut self,
        bytes: T,
        features: Features,
    ) -> Result<()> {
        let b = bytes.as_ref();

        Errno::from(unsafe {
            ffi::xed_decode_with_features(
                self.as_mut_ptr(),
                b.as_ptr(),
                b.len() as u32,
                features.as_ptr().cast_mut(),
            )
        })
        .into()
    }
}
