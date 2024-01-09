use derive_more::{Deref, DerefMut, From, Into};

use crate::{ffi, Op, Reg};

#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, From, Into)]
pub struct Displacement(ffi::xed_enc_displacement_t);

pub fn disp(disp: i64, disp_bits: u32) -> Displacement {
    Displacement(unsafe { ffi::xed_disp(disp, disp_bits) })
}

impl Displacement {
    pub const MAX_BYTES: usize = ffi::XED_MAX_DISPLACEMENT_BYTES as usize;
}

#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, From, Into)]
pub struct Type(ffi::xed_encoder_operand_type_t);

impl Type {
    pub const INVALID: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_INVALID);
    pub const REL_BRDISP: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_REL_BRDISP);
    pub const ABS_BRDISP: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_ABS_BRDISP);
    pub const REG: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_REG);
    pub const IMM0: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_IMM0);
    pub const SIMM0: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_SIMM0);
    pub const IMM1: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_IMM1);
    pub const MEM: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_MEM);
    pub const PTR: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_PTR);

    /// special for things with suppressed implicit memops
    pub const SEG0: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_SEG0);

    /// special for things with suppressed implicit memops
    pub const SEG1: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_SEG1);

    /// specific operand storage fields -- must supply a name
    pub const OTHER: Type = Type(ffi::XED_ENCODER_OPERAND_TYPE_OTHER);
}

#[repr(transparent)]
#[derive(Clone, Debug, From, Into)]
pub struct Operand(ffi::xed_encoder_operand_t);

impl From<Reg> for Operand {
    fn from(r: Reg) -> Self {
        reg(r)
    }
}

/// Branch Displacement
pub fn relbr(brdisp: i32, width_bits: u32) -> Operand {
    unsafe { ffi::xed_relbr(brdisp, width_bits) }.into()
}

/// Pointer Displacement
pub fn ptr(brdisp: i32, width_bits: u32) -> Operand {
    unsafe { ffi::xed_ptr(brdisp, width_bits) }.into()
}

/// Register and Immediate Operands
pub fn reg(reg: Reg) -> Operand {
    unsafe { ffi::xed_reg(reg.into()) }.into()
}

/// The first immediate operand (known as IMM0)
pub fn imm0(v: u64, width_bits: u32) -> Operand {
    unsafe { ffi::xed_imm0(v, width_bits) }.into()
}

/// An 32b signed immediate operand
pub fn simm0(v: i32, width_bits: u32) -> Operand {
    unsafe { ffi::xed_simm0(v, width_bits) }.into()
}

/// The 2nd immediate operand (known as IMM1) for rare instructions that require it.
pub fn imm1(v: u8) -> Operand {
    unsafe { ffi::xed_imm1(v) }.into()
}

/// an operand storage field name and value
pub fn other(op: Op, value: i32) -> Operand {
    unsafe { ffi::xed_other(op.into(), value) }.into()
}

/// seg reg override for implicit suppressed memory ops
pub fn seg0(seg0: Reg) -> Operand {
    unsafe { ffi::xed_seg0(seg0.into()) }.into()
}

/// seg reg override for implicit suppressed memory ops
pub fn seg1(seg1: Reg) -> Operand {
    unsafe { ffi::xed_seg1(seg1.into()) }.into()
}

/// memory operand - base only
pub fn mem_b(base: Reg, width_bits: u32) -> Operand {
    unsafe { ffi::xed_mem_b(base.into(), width_bits) }.into()
}

/// memory operand - base and displacement only
pub fn mem_bd<B>(base: B, disp: Displacement, width_bits: u32) -> Operand
where
    B: Into<Option<Reg>>,
{
    unsafe {
        ffi::xed_mem_bd(
            base.into().unwrap_or(Reg::INVALID).into(),
            disp.into(),
            width_bits,
        )
    }
    .into()
}

/// memory operand - base, index, scale, displacement
pub fn mem_bisd(base: Reg, index: Reg, scale: u32, disp: Displacement, width_bits: u32) -> Operand {
    unsafe { ffi::xed_mem_bisd(base.into(), index.into(), scale, disp.into(), width_bits) }.into()
}

/// memory operand - segment and base only
pub fn mem_gb(seg: Reg, base: Reg, width_bits: u32) -> Operand {
    unsafe { ffi::xed_mem_gb(seg.into(), base.into(), width_bits) }.into()
}

/// memory operand - segment, base and displacement only
pub fn mem_gbd(seg: Reg, base: Reg, disp: Displacement, width_bits: u32) -> Operand {
    unsafe { ffi::xed_mem_gbd(seg.into(), base.into(), disp.into(), width_bits) }.into()
}

/// memory operand - segment and displacement only
pub fn mem_gd(seg: Reg, disp: Displacement, width_bits: u32) -> Operand {
    unsafe { ffi::xed_mem_gd(seg.into(), disp.into(), width_bits) }.into()
}

/// memory operand - base, index, scale, displacement
pub fn mem_gbisd<B, I, S>(
    seg: Reg,
    base: B,
    index: I,
    scale: S,
    disp: Displacement,
    width_bits: u32,
) -> Operand
where
    B: Into<Option<Reg>>,
    I: Into<Option<Reg>>,
    S: Into<Option<u32>>,
{
    unsafe {
        ffi::xed_mem_gbisd(
            seg.into(),
            base.into().unwrap_or(Reg::INVALID).into(),
            index.into().unwrap_or(Reg::INVALID).into(),
            scale.into().unwrap_or_default(),
            disp.into(),
            width_bits,
        )
    }
    .into()
}
