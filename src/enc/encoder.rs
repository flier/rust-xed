use std::{ffi::CStr, fmt, mem::MaybeUninit};

use derive_more::From;
use nom::Finish;

use crate::{
    decoded::Operands,
    ffi,
    raw::{AsMutPtr, AsPtr},
    DecodedInst, Errno, Error, Iclass, Op, Reg, RegClass, Result, State,
};

use super::lang::{inst, Immed, Inst, Operand};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, From)]
pub struct Request<I>(I);

impl AsPtr for Request<&mut DecodedInst> {
    type CType = ffi::xed_encoder_request_t;

    fn as_ptr(&self) -> *const Self::CType {
        self.0.as_ptr().cast()
    }
}

impl AsMutPtr for Request<&mut DecodedInst> {
    fn as_mut_ptr(&mut self) -> *mut Self::CType {
        self.0.as_mut_ptr().cast()
    }
}

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

    pub fn reset_mode<S: Into<State>>(&mut self, state: S) {
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

    pub fn set_branch_displacement(&mut self, brdisp: i32, nbytes: u32) -> &mut Self {
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

    fn apply_memory_fields(
        &mut self,
        length: Option<u8>,
        seg: Option<Reg>,
        base: Reg,
        index: Option<Reg>,
        scale: Option<u32>,
        displacement: Option<Immed>,
    ) {
        if base == Reg::EIP
            || base.class() == Some(RegClass::GPR32)
            || index.and_then(|reg| reg.class()) == Some(RegClass::GPR32)
        {
            self.set_effective_address_size(32);
        } else if base.class() == Some(RegClass::GPR16)
            || index.and_then(|reg| reg.class()) == Some(RegClass::GPR16)
        {
            self.set_effective_address_size(16);
        }

        self.set_base0(base);

        if let Some(reg) = index {
            self.set_index(reg);
        }
        if let Some(scale) = scale {
            self.set_scale(scale);
        }
        if let Some(reg) = seg {
            self.set_seg0(reg);
        }
        if let Some(len) = length {
            self.set_memory_operand_length(len as u32);
        }
        if let Some(disp) = displacement {
            self.set_memory_displacement(disp.value as i64, disp.width_bits / 8);
        }
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

pub fn compile(expr: &str, state: State) -> Result<Request<DecodedInst>> {
    let expr = match inst(expr).finish() {
        Ok((rest, e)) => {
            if rest.is_empty() {
                e
            } else {
                return Err(Error::UnexpectedExpr(rest.to_string()));
            }
        }
        Err(nom::error::Error { input: _, code }) => return Err(Error::Parse(code)),
    };

    let inst = DecodedInst::new();
    let mut req = Request(inst);

    req.reset_mode(state);
    req.apply(expr);

    Ok(req)
}

impl<I> Request<I>
where
    Self: AsMutPtr<CType = ffi::xed_encoder_request_t>,
{
    fn apply(&mut self, expr: Inst) {
        let mut vl = None;
        let mut uvl = None;

        self.set_iclass(expr.opcode.name);

        // we can attempt to override the mode
        if let Some(width) = expr.opcode.width_bits {
            match width {
                8 | 16 | 32 | 64 => {
                    self.set_effective_operand_width(width);
                }
                128 => {
                    uvl = Some(VectorLength::Xmm);
                }
                256 => {
                    uvl = Some(VectorLength::Ymm);
                }
                512 => {
                    uvl = Some(VectorLength::Zmm);
                }
                _ => unreachable!(),
            }
        }

        let mut memop = 0;

        // The registers operands are numbered starting from the first one as Op::REG0.
        // We increment regnum (below) every time we add a register operands.
        let mut regs = (i32::from(Op::REG0)..=i32::from(Op::REG9)).map(Op::from);

        // put the operands in the request.
        for (i, op) in expr.operands.into_iter().enumerate() {
            let operand_index = i as u32;

            match op {
                Operand::Mem(mem) => {
                    if memop == 0 {
                        // Tell XED that we have a memory operand
                        self.set_mem0();
                        // Tell XED that the mem0 operand is the next operand:
                        self.set_operand_order(operand_index, Op::MEM0);
                    } else {
                        self.set_mem1();
                        // Tell XED that the mem1 operand is the next operand:
                        self.set_operand_order(operand_index, Op::MEM1);
                    }
                    memop += 1;

                    self.apply_memory_fields(
                        Some(mem.length),
                        mem.seg,
                        mem.base,
                        mem.index,
                        mem.scale,
                        mem.displacement,
                    );

                    if let Some(reg) = mem.index {
                        // for scatter/gather
                        vl = find_vl(reg, vl);
                    }
                }
                Operand::Agen(agen) => {
                    // Tell XED we have an AGEN
                    self.set_agen();
                    // The AGEN is the next operand
                    self.set_operand_order(operand_index, Op::AGEN);

                    self.apply_memory_fields(
                        None,
                        None,
                        agen.base,
                        agen.index,
                        agen.scale,
                        agen.displacement,
                    );

                    if let Some(reg) = agen.index {
                        // for scatter/gather
                        vl = find_vl(reg, vl);
                    }
                }
                Operand::Seg(seg) => {
                    if matches!(seg.id, Some(0) | None) {
                        self.set_seg0(seg.reg);
                    } else {
                        // need SEG1 for MOVS[BWDQ]
                        self.set_seg1(seg.reg);
                    }

                    /* SEG/SEG0/SEG1 is NOT a normal operand. It is a setting, like
                     * the lock prefix. Normally the segment will be specified with
                     * normal memory operations. With memops without MODRM, or
                     * impliclit memops, we need a way of specifying the segment
                     * when it is not the default. This is the way. it does not
                     * change encoding forms. (When segments are "moved", they are
                     * REG operands, not SEG0/1, and are specified by name like EAX
                     * is.) */
                }
                Operand::Imm(imm) => {
                    self.set_uimm0_bits(imm.value as u64, imm.width_bits)
                        .set_operand_order(operand_index, Op::IMM0);
                }
                Operand::Simm(imm) => {
                    self.set_simm(imm.value as i32, imm.width_bits / 8)
                        .set_operand_order(operand_index, Op::IMM0);
                }
                Operand::Imm2(imm) => {
                    self.set_uimm1(imm.value as u8)
                        .set_operand_order(operand_index, Op::IMM1);
                }
                Operand::BrDisp(disp) => {
                    self.set_branch_displacement(disp.value as i32, disp.width_bits / 8)
                        .set_operand_order(operand_index, Op::RELBR)
                        .set_relbr();
                }
                Operand::Ptr(disp) => {
                    self.set_branch_displacement(disp.value as i32, disp.width_bits / 8)
                        .set_operand_order(operand_index, Op::PTR)
                        .set_ptr();
                }
                Operand::Reg(reg) => {
                    if let Some(op) = regs.next() {
                        // store the register identifier in the operand storage field
                        self.set_reg(op, reg);
                        // store the operand storage field name in the encode-order array
                        self.set_operand_order(operand_index, op);
                    }

                    vl = find_vl(reg, vl);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum VectorLength {
    Xmm,
    Ymm,
    Zmm,
}

fn find_vl(reg: Reg, nvl: Option<VectorLength>) -> Option<VectorLength> {
    // This will "grow" bad user settings. So if you try to specify /128 on
    // the instruction and it sees a YMM or ZMM register operand, then
    // it'll grow the VL to the right observed size. The right observed
    // size might still be wrong, that is too small (as it can be for
    // "shrinking" converts (PD2PS, PD2DQ, etc.).

    match reg.class()? {
        RegClass::XMM if nvl.is_none() => {
            // not set and see xmm
            Some(VectorLength::Xmm)
        }
        RegClass::YMM if nvl.map_or(false, |v| v < VectorLength::Ymm) => {
            // not set, set to xmm and then see ymm
            Some(VectorLength::Ymm)
        }
        RegClass::ZMM if nvl.map_or(false, |v| v < VectorLength::Zmm) => {
            // not set, set to xmm or ymm and then see zmm
            Some(VectorLength::Zmm)
        }
        _ => nvl,
    }
}
