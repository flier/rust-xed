use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{
        alpha1, alphanumeric0, alphanumeric1, char, hex_digit1, satisfy, space1, u32, u8,
    },
    combinator::{map, map_res, opt, recognize, verify},
    multi::many0,
    sequence::{pair, preceded, tuple},
    IResult,
};

use crate::{Iclass, Reg};

#[derive(Clone, Debug, PartialEq)]
pub struct Inst {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.opcode)?;

        for op in &self.operands {
            write!(f, " {}", op)?;
        }

        Ok(())
    }
}

pub fn inst(input: &str) -> IResult<&str, Inst> {
    map(
        pair(opcode, many0(preceded(space1, operand))),
        |(opcode, operands)| Inst { opcode, operands },
    )(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Opcode {
    pub name: Iclass,
    pub width_bits: Option<u32>,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(width_bits) = self.width_bits {
            write!(f, "/{}", width_bits)?;
        }
        Ok(())
    }
}

fn opcode(input: &str) -> IResult<&str, Opcode> {
    map(
        pair(
            map_res(
                recognize(pair(
                    alphanumeric1,
                    many0(satisfy(|c| c.is_alphanumeric() || c == '_')),
                )),
                |s: &str| s.to_uppercase().parse(),
            ),
            opt(preceded(
                char('/'),
                verify(u32, |n| OPCODE_WIDTHS.contains(n)),
            )),
        ),
        |(name, width_bits)| Opcode { name, width_bits },
    )(input)
}

const OPCODE_WIDTHS: [u32; 7] = [8, 16, 32, 64, 128, 256, 512];

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    /// registers
    Reg(Reg),
    /// memory operations
    Mem(Mem),
    /// address generation
    Agen(Agen),
    Seg(Seg),
    /// immediates
    Imm(Immed),
    Simm(Immed),
    /// immediates
    Imm2(Immed),
    /// branch displacements
    BrDisp(Immed),
    Ptr(Immed),
}

impl From<Reg> for Operand {
    fn from(reg: Reg) -> Self {
        Operand::Reg(reg)
    }
}

impl From<Mem> for Operand {
    fn from(mem: Mem) -> Self {
        Operand::Mem(mem)
    }
}

impl From<Agen> for Operand {
    fn from(agen: Agen) -> Self {
        Operand::Agen(agen)
    }
}

impl From<Seg> for Operand {
    fn from(seg: Seg) -> Self {
        Operand::Seg(seg)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mem(mem) => mem.fmt(f),
            Self::Agen(agen) => agen.fmt(f),
            Self::Seg(seg) => seg.fmt(f),
            Self::Imm(immed) => write!(f, "IMM:{}", immed),
            Self::Simm(immed) => write!(f, "SIMM:{}", immed),
            Self::Imm2(immed) => write!(f, "IMM2:{}", immed),
            Self::BrDisp(immed) => write!(f, "BRDISP:{}", immed),
            Self::Ptr(immed) => write!(f, "PTR:{}", immed),
            Self::Reg(reg) => write!(f, "{}", reg),
        }
    }
}

pub fn operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(mem, Operand::Mem),
        map(agen, Operand::Agen),
        map(seg, Operand::Seg),
        map(imm, Operand::Imm),
        map(simm, Operand::Simm),
        map(imm2, Operand::Imm2),
        map(brdisp, Operand::BrDisp),
        map(ptr, Operand::Ptr),
        map(reg, Operand::Reg),
    ))(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mem {
    pub length: u8,
    pub seg: Option<Reg>,
    pub base: Reg,
    pub index: Option<Reg>,
    pub scale: Option<u32>,
    pub displacement: Option<Immed>,
}

impl fmt::Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MEM{}", self.length)?;
        if let Some(seg) = self.seg {
            write!(f, ":{}", seg)?;
        }
        write!(f, ":{}", self.base)?;
        if let Some(index) = self.index {
            write!(f, ",{}", index)?;
        } else if self.scale.is_some() || self.displacement.is_some() {
            f.write_str(",-")?;
        }
        if let Some(scale) = self.scale {
            write!(f, ",{}", scale)?;
        } else if self.displacement.is_some() {
            f.write_str(",-")?;
        }
        if let Some(ref disp) = self.displacement {
            write!(f, ",{}", disp)?;
        }
        Ok(())
    }
}

/// Parse Mem operand
///
/// MEMlength:[segment:]base,index,scale[,displacement]
pub fn mem(input: &str) -> IResult<&str, Mem> {
    map(
        tuple((
            tuple((
                preceded(tag_no_case("MEM"), u8),
                alt((
                    map(
                        pair(preceded(char(':'), reg_opt), preceded(char(':'), reg)),
                        |(seg, base)| (seg, base),
                    ),
                    map(preceded(char(':'), reg), |base| (None, base)),
                )),
            )),
            opt(preceded(char(','), reg_opt)),
            opt(preceded(char(','), scale)),
            opt(preceded(char(','), immed)),
        )),
        |((length, (seg, base)), index, scale, displacement)| Mem {
            length,
            seg,
            base,
            index: index.flatten(),
            scale: scale.flatten(),
            displacement,
        },
    )(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Agen {
    pub base: Reg,
    pub index: Option<Reg>,
    pub scale: Option<u32>,
    pub displacement: Option<Immed>,
}

impl fmt::Display for Agen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AGEN:{}", self.base)?;
        if let Some(index) = self.index {
            write!(f, ",{}", index)?;
        } else if self.scale.is_some() || self.displacement.is_some() {
            f.write_str(",-")?;
        }
        if let Some(scale) = self.scale {
            write!(f, ",{}", scale)?;
        } else if self.displacement.is_some() {
            f.write_str(",-")?;
        }
        if let Some(ref disp) = self.displacement {
            write!(f, ",{}", disp)?;
        }
        Ok(())
    }
}

/// parse Seg operand
///
/// AGEN:base,index,scale[,displacement]
pub fn agen(input: &str) -> IResult<&str, Agen> {
    map(
        tuple((
            tag_no_case("AGEN"),
            preceded(char(':'), reg),
            opt(preceded(char(','), reg_opt)),
            opt(preceded(char(','), scale)),
            opt(preceded(char(','), immed)),
        )),
        |(_, base, index, scale, displacement)| Agen {
            base,
            index: index.flatten(),
            scale: scale.flatten(),
            displacement,
        },
    )(input)
}

fn reg(input: &str) -> IResult<&str, Reg> {
    map_res(recognize(pair(alpha1, alphanumeric0)), |s: &str| {
        s.to_uppercase().parse()
    })(input)
}

fn reg_opt(input: &str) -> IResult<&str, Option<Reg>> {
    alt((map(char('-'), |_| None), map(reg, Some)))(input)
}

fn scale(input: &str) -> IResult<&str, Option<u32>> {
    alt((map(char('-'), |_| None), map(u32, Some)))(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Seg {
    pub id: Option<u8>,
    pub reg: Reg,
}

impl fmt::Display for Seg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SEG")?;
        if let Some(id) = self.id {
            write!(f, "{}", id)?;
        }
        write!(f, ":{}", self.reg)
    }
}

/// parse Seg operand
///
/// ```
/// # use xed::{enc::lang::{seg, Seg}, Reg};
/// assert_eq!(seg("SEG:GS").unwrap(), ("", Seg { id: 0, reg: Reg::GS }));
/// assert_eq!(seg("SEG1:DS").unwrap(), ("", Seg { id: 1, reg: Reg::DS }));
/// ```
pub fn seg(input: &str) -> IResult<&str, Seg> {
    map(
        tuple((tag_no_case("SEG"), opt(u8), char(':'), reg)),
        |(_, id, _, reg)| Seg { id, reg },
    )(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Immed {
    pub value: u32,
    pub width_bits: u32,
}

impl fmt::Display for Immed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:X}", self.value);

        write!(
            f,
            "{}{}",
            "0".repeat(self.width_bits as usize / 4 - s.len()),
            s
        )
    }
}

fn immed(input: &str) -> IResult<&str, Immed> {
    map(
        map_res(hex_digit1, |s: &str| {
            u32::from_str_radix(s, 16).map(|n| (n, s.len() as u32 * 4))
        }),
        |(value, width_bits)| Immed { value, width_bits },
    )(input)
}

/// parse IMM operand
///
/// ```
/// # use xed::enc::lang::{imm, Immed};
/// assert_eq!(imm("IMM:11223344").unwrap(), ("", Immed { value: 0x11223344, width_bits: 32 }));
/// assert_eq!(imm("IMM:ff").unwrap(), ("", Immed { value: 0xff, width_bits: 8 }));
/// ```
pub fn imm(input: &str) -> IResult<&str, Immed> {
    preceded(pair(tag_no_case("IMM"), char(':')), immed)(input)
}

/// parse SIMM operand
///
/// ```
/// # use xed::enc::lang::{simm, Immed};
/// assert_eq!(simm("SIMM:11223344").unwrap(), ("", Immed { value: 0x11223344, width_bits: 32 }));
/// assert_eq!(simm("SIMM:f0").unwrap(), ("", Immed { value: 0xf0, width_bits: 8 }));
/// ```
pub fn simm(input: &str) -> IResult<&str, Immed> {
    preceded(pair(tag_no_case("SIMM"), char(':')), immed)(input)
}

/// parse IMM2 operand
///
/// ```
/// # use xed::enc::lang::{imm2, Immed};
/// assert_eq!(imm2("IMM2:11223344").unwrap(), ("", Immed { value: 0x11223344, width_bits: 32 }));
/// assert_eq!(imm2("IMM2:ff").unwrap(), ("", Immed { value: 0xff, width_bits: 8 }));
/// ```
pub fn imm2(input: &str) -> IResult<&str, Immed> {
    preceded(pair(tag_no_case("IMM2"), char(':')), immed)(input)
}

/// parse BRDISP operand
///
/// ```
/// # use xed::enc::lang::{brdisp, Immed};
/// assert_eq!(brdisp("BRDISP:11223344").unwrap(), ("", Immed { value: 0x11223344, width_bits: 32 }));
/// assert_eq!(brdisp("BRDISP:ff").unwrap(), ("", Immed { value: 0xff, width_bits: 8 }));
/// ```
pub fn brdisp(input: &str) -> IResult<&str, Immed> {
    preceded(pair(tag_no_case("BRDISP"), char(':')), immed)(input)
}

/// parse PTR operand
///
/// ```
/// # use xed::enc::lang::{ptr, Immed};
/// assert_eq!(ptr("PTR:11223344").unwrap(), ("", Immed { value: 0x11223344, width_bits: 32 }));
/// assert_eq!(ptr("PTR:ff").unwrap(), ("", Immed { value: 0xff, width_bits: 8 }));
/// ```
pub fn ptr(input: &str) -> IResult<&str, Immed> {
    preceded(pair(tag_no_case("PTR"), char(':')), immed)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inst() {
        for (s, r) in [
            (
                "ADD EAX EAX",
                Inst {
                    opcode: Opcode {
                        name: Iclass::ADD,
                        width_bits: None,
                    },
                    operands: vec![Reg::EAX.into(), Reg::EAX.into()],
                },
            ),
            (
                "VGATHERDPS xmm0 MEM4:rax,xmm1,1 xmm2",
                Inst {
                    opcode: Opcode {
                        name: Iclass::VGATHERDPS,
                        width_bits: None,
                    },
                    operands: vec![
                        Reg::XMM0.into(),
                        Mem {
                            length: 4,
                            seg: None,
                            base: Reg::RAX,
                            index: Some(Reg::XMM1),
                            scale: Some(1),
                            displacement: None,
                        }
                        .into(),
                        Reg::XMM2.into(),
                    ],
                },
            ),
            (
                "LEA/64 RAX AGEN:R13,R13,2,00",
                Inst {
                    opcode: Opcode {
                        name: Iclass::LEA,
                        width_bits: Some(64),
                    },
                    operands: vec![
                        Reg::RAX.into(),
                        Agen {
                            base: Reg::R13,
                            index: Some(Reg::R13),
                            scale: Some(2),
                            displacement: Some(Immed {
                                value: 0,
                                width_bits: 8,
                            }),
                        }
                        .into(),
                    ],
                },
            ),
        ] {
            assert_eq!(r.to_string(), s.to_uppercase());
            assert_eq!(inst(s).unwrap(), ("", r), "parse {s}");
        }
    }

    #[test]
    fn test_opcode() {
        for (s, r) in [
            (
                "MOV",
                Opcode {
                    name: Iclass::MOV,
                    width_bits: None,
                },
            ),
            (
                "LEA/64",
                Opcode {
                    name: Iclass::LEA,
                    width_bits: Some(64),
                },
            ),
            (
                "mov_cr",
                Opcode {
                    name: Iclass::MOV_CR,
                    width_bits: None,
                },
            ),
        ] {
            assert_eq!(r.to_string(), s.to_uppercase());
            assert_eq!(opcode(s).unwrap(), ("", r), "parse {s}");
        }
    }

    #[test]
    fn test_mem_operand() {
        for (s, r) in [
            (
                "MEM4:FS:EAX,EBX,4,223344",
                Mem {
                    length: 4,
                    seg: Some(Reg::FS),
                    base: Reg::EAX,
                    index: Some(Reg::EBX),
                    scale: Some(4),
                    displacement: Some(Immed {
                        value: 0x223344,
                        width_bits: 24,
                    }),
                },
            ),
            (
                "MEM4:FS:EAX,EBX,4",
                Mem {
                    length: 4,
                    seg: Some(Reg::FS),
                    base: Reg::EAX,
                    index: Some(Reg::EBX),
                    scale: Some(4),
                    displacement: None,
                },
            ),
            (
                "MEM4:EAX,EBX,4,223344",
                Mem {
                    length: 4,
                    seg: None,
                    base: Reg::EAX,
                    index: Some(Reg::EBX),
                    scale: Some(4),
                    displacement: Some(Immed {
                        value: 0x223344,
                        width_bits: 24,
                    }),
                },
            ),
            (
                "MEM8:RIP,-,-,11223344",
                Mem {
                    length: 8,
                    seg: None,
                    base: Reg::RIP,
                    index: None,
                    scale: None,
                    displacement: Some(Immed {
                        value: 0x11223344,
                        width_bits: 32,
                    }),
                },
            ),
            (
                "MEM8:EBX,EAX,4,00",
                Mem {
                    length: 8,
                    seg: None,
                    base: Reg::EBX,
                    index: Some(Reg::EAX),
                    scale: Some(4),
                    displacement: Some(Immed {
                        value: 0x00,
                        width_bits: 8,
                    }),
                },
            ),
            (
                "MEM4:R13D,-,-,FF",
                Mem {
                    length: 4,
                    seg: None,
                    base: Reg::R13D,
                    index: None,
                    scale: None,
                    displacement: Some(Immed {
                        value: 0xFF,
                        width_bits: 8,
                    }),
                },
            ),
            (
                "MEM4:rax,ymm1,1",
                Mem {
                    length: 4,
                    seg: None,
                    base: Reg::RAX,
                    index: Some(Reg::YMM1),
                    scale: Some(1),
                    displacement: None,
                },
            ),
            (
                "MEM4:rax",
                Mem {
                    length: 4,
                    seg: None,
                    base: Reg::RAX,
                    index: None,
                    scale: None,
                    displacement: None,
                },
            ),
        ] {
            assert_eq!(r.to_string(), s.to_uppercase());
            assert_eq!(mem(s).unwrap(), ("", r), "parse {s}");
        }
    }

    #[test]
    fn test_agen_operand() {
        for (s, r) in [
            (
                "AGEN:ECX",
                Agen {
                    base: Reg::ECX,
                    index: None,
                    scale: None,
                    displacement: None,
                },
            ),
            (
                "AGEN:EAX,-,-,00",
                Agen {
                    base: Reg::EAX,
                    index: None,
                    scale: None,
                    displacement: Some(Immed {
                        value: 0x00,
                        width_bits: 8,
                    }),
                },
            ),
            (
                "AGEN:R13,R13,2,00",
                Agen {
                    base: Reg::R13,
                    index: Some(Reg::R13),
                    scale: Some(2),
                    displacement: Some(Immed {
                        value: 0x00,
                        width_bits: 8,
                    }),
                },
            ),
            (
                "AGEN:r8d,ebx,8",
                Agen {
                    base: Reg::R8D,
                    index: Some(Reg::EBX),
                    scale: Some(8),
                    displacement: None,
                },
            ),
        ] {
            assert_eq!(r.to_string(), s.to_uppercase());
            assert_eq!(agen(s).unwrap(), ("", r), "parse {s}");
        }
    }

    #[test]
    fn test_seg() {
        for (s, r) in [
            (
                "SEG:GS",
                Seg {
                    id: None,
                    reg: Reg::GS,
                },
            ),
            (
                "SEG0:GS",
                Seg {
                    id: Some(0),
                    reg: Reg::GS,
                },
            ),
            (
                "SEG1:DS",
                Seg {
                    id: Some(1),
                    reg: Reg::DS,
                },
            ),
        ] {
            assert_eq!(r.to_string(), s.to_uppercase());
            assert_eq!(seg(s).unwrap(), ("", r), "parse {s}");
        }
    }

    #[test]
    fn test_imm() {
        for (s, r) in [
            (
                "IMM:11223344",
                Immed {
                    value: 0x11223344,
                    width_bits: 32,
                },
            ),
            (
                "IMM:ff",
                Immed {
                    value: 0xff,
                    width_bits: 8,
                },
            ),
        ] {
            assert_eq!(imm(s).unwrap(), ("", r), "parse {s}");
        }
    }
}
