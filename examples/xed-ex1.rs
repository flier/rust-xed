use std::fmt::Write;

use anyhow::Result;
use clap::Parser;

use xed::{dec::Inst, tables, AddressWidth, Attribute, Chip, MachineMode, Op, SignExtend};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// Stack address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Qword)]
    pub width: AddressWidth,

    /// Chip model
    #[arg(short, long)]
    pub chip: Option<Chip>,

    pub bytes: Vec<String>,
}

impl Opts {
    pub fn bytes(&self) -> Result<Vec<u8>> {
        Ok(hex::decode(
            self.bytes
                .iter()
                .map(|s| s.bytes())
                .flatten()
                .collect::<Vec<_>>(),
        )?)
    }
}

fn print_operands(xedd: &Inst) -> Result<()> {
    let xi = xedd.inst();

    println!("Operands");
    println!("#   TYPE               DETAILS        VIS  RW       OC2 BITS BYTES NELEM ELEMSZ   ELEMTYPE   REGCLASS");
    println!("#   ====               =======        ===  ==       === ==== ===== ===== ======   ========   ========");

    for (i, op) in xi.operands().enumerate() {
        let name = op.name();

        print!("{} {:^8} ", i, name.to_string());

        let mut buf = String::new();

        match name {
            Op::AGEN | Op::MEM0 | Op::MEM1 => {
                // we print memops in a different function
                write!(&mut buf, "(see below)")?;
            }
            Op::PTR | // pointer (always in conjunction with a IMM0)
            Op::RELBR // branch displacements
            => {
                let disp_bits = xedd.branch_displacement_width();
                if disp_bits > 0 {
                    let disp = xedd.branch_displacement();

                    write!(
                        &mut buf,
                        "BRANCH_DISPLACEMENT_BYTES= {} {:x}",
                        disp_bits, disp
                    )?;
                }
            }
            Op::IMM0 => {
                let ibits = xedd.immediate_width_bits();
                let (x, n) = if xedd.immediate_is_signed() {
                    let x  = (xedd.signed_immediate() as u64).arbitrary(ibits) as u64;
                    let n = if ibits > 0 { ibits as usize/8} else {1};

                    (x, n)
                }else {
                    let x = xedd.unsigned_immediate();
                    let n = if ibits > 0 { ibits as usize/8} else {2};

                    (x, n)
                };
                let b = hex::encode(&x.to_ne_bytes()[..n]);

                write!(&mut buf, "0x{}({}b)", b, ibits)?;
            }
            Op::IMM1 => {
                write!(&mut buf, "0x{:x}", xedd.second_immediate())?;
            }
            Op::REG0
            | Op::REG1
            | Op::REG2
            | Op::REG3
            | Op::REG4
            | Op::REG5
            | Op::REG6
            | Op::REG7
            | Op::REG8
            | Op::REG9
            | Op::BASE0
            | Op::BASE1 => {
                let reg = xedd.reg(name);

                write!(&mut buf, "{}={}", name, reg)?;
            }
            _ => {
                println!("need to add support for printing operand: {}", name);
            }
        }

        print!("{:<21}", buf);

        if let Some(oi) = xedd.operand(i as u32) {
            print!(
                "{:<10} {:3} {:>7} ",
                op.visibility().to_string(),
                oi.action().to_string(),
                op.width().to_string()
            );

            let bits = oi.length_bits();

            print!("  {:<3}", bits);
            /* rounding, bits might not be a multiple of 8 */
            print!(" {:^4}", (bits + 7) / 8);
            print!("   {:2}", oi.elements());
            print!("    {:3}", oi.element_size_bits());
            print!("  {:>10}", oi.element_type().to_string());
            if let Some(clz) = xedd.reg(name).class() {
                print!(" {:>10}", clz.to_string());
            }
        }

        println!()
    }

    Ok(())
}

fn print_memops(xedd: &Inst) -> Result<()> {
    println!("Memory Operands");

    for (i, op) in xedd.mem_operands().enumerate() {
        if op.read() {
            print!("   read ");
        }
        if op.written() {
            print!("written ");
        }
        if !op.read() && !op.written() {
            print!("   agen "); // LEA instructions
        }

        if let Some(seg) = op.seg() {
            print!("SEG= {} ", seg);
        }
        if let Some(base) = op.base() {
            print!("BASE= {}/{} ", base, base.class().unwrap());
        }
        if let Some(index) = op.index() {
            if i == 0 {
                print!("INDEX= {}/{} ", index, index.class().unwrap());

                if let Some(scale) = op.scale() {
                    // only have a scale if the index exists.
                    print!("SCALE= {} ", scale);
                }
            }
        }

        if xedd.operands().has_memory_displacement() {
            if let Some((disp, disp_bits)) =
                op.memory_displacement().zip(op.memory_displacement_width())
            {
                print!("DISPLACEMENT_BYTES= {} ", disp_bits);
                print!("0x{:x} base10={}", disp, disp);
            }
        }

        println!(" ASZ{}={}", i, op.address_width());
    }

    println!(
        "  MemopBytes = {}",
        xedd.mem_operand(0)
            .map(|op| op.length())
            .unwrap_or_default()
    );

    Ok(())
}

fn print_flags(xedd: &Inst) -> Result<()> {
    if xedd.uses_rflags() {
        if let Some(rfi) = xedd.rflags_info() {
            println!("FLAGS:");

            if rfi.reads_flags() {
                print!("   reads-rflags ");
            } else if rfi.writes_flags() {
                if rfi.may_write() {
                    print!("  may-write-rflags ");
                }
            }

            for action in rfi.actions() {
                print!("{} ", action);
            }
            println!();

            // or as as bit-union
            let read_set = rfi.read_flag_set();
            let written_set = rfi.written_flag_set();
            let undefined_set = rfi.undefined_flag_set();

            println!("       read: {:30} mask=0x{:x}", read_set, read_set.mask());
            println!(
                "    written: {:30} mask=0x{:x}",
                written_set,
                written_set.mask()
            );
            println!(
                "  undefined: {:30} mask=0x{:x}",
                undefined_set,
                undefined_set.mask()
            );
        }
    }

    Ok(())
}

fn print_reads_zf_flag(xedd: &Inst) -> Result<()> {
    if xedd.uses_rflags() {
        if let Some(rfi) = xedd.rflags_info() {
            if rfi.read_flag_set().zf() != 0 {
                print!("READS ZF");
            }
        }
    }

    Ok(())
}

fn print_attributes(xedd: &Inst) -> Result<()> {
    let xi = xedd.inst();

    print!("ATTRIBUTES: ");
    for attr in xi.attrs() {
        print!("{} ", attr);
    }
    println!();

    Ok(())
}

fn print_misc(xedd: &Inst) -> Result<()> {
    let ov = xedd.operands();
    let xi = xedd.inst();

    if ov.has_real_rep() {
        let norep = xedd.iclass().rep_remove();

        println!("REAL REP \tcorresponding no-rep iclass: {}", norep)
    }
    if ov.has_rep_prefix() {
        println!("F3 PREFIX");
    }
    if ov.has_repne_prefix() {
        println!("F2 PREFIX");
    }
    if ov.has_address_size_prefix() {
        println!("67 PREFIX");
    }
    if ov.has_operand_size_prefix() {
        // this 66 prefix is not part of the opcode
        println!("66-OSZ PREFIX");
    }
    if ov.has_66_prefix() {
        // this is any 66 prefix including the above
        println!("ANY 66 PREFIX");
    }
    if xedd.has_attr(Attribute::RING0) {
        println!("RING0 only");
    }
    if let Some(e) = xi.exception() {
        println!("EXCEPTION TYPE: {}", e);
    }
    if xedd.is_broadcast() {
        println!("BROADCAST");
    }
    if xedd.sse() || xedd.avx() || xedd.avx512() {
        if xedd.avx512_maskop() {
            println!("AVX512 KMASK-OP");
        } else {
            if xedd.sse() {
                println!("SSE");
            } else if xedd.avx() {
                println!("AVX");
            } else if xedd.avx512() {
                println!("AVX512");
            }

            if xedd.has_attr(Attribute::SIMD_SCALAR) {
                println!("SCALAR");
            } else {
                // vector_length_bits is only for VEX/EVEX instr.
                // This will print 128 vl for FXSAVE and LD/ST MXCSR which is unfortunate.
                let vl_bits = if xedd.sse() {
                    128
                } else {
                    xedd.vector_length_bits()
                };
                println!("Vector length: {}", vl_bits);
            }

            if xedd.avx512() {
                let vec_elements = xedd.avx512_dest_elements();
                println!("AVX512 vector elements: {}", vec_elements);
            }
        }
    }

    // does not include instructions that have XED_ATTRIBUTE_MASK_AS_CONTROL.
    // does not include vetor instructions that have k0 as a mask register.
    if xedd.masked_vector_operation() {
        println!("WRITE-MASKING");
    }

    let np = xedd.nprefixes();
    if np > 0 {
        println!("Number of legacy prefixes: {}", np);
    }

    let isa_set = xedd.isa_set();

    println!("ISA SET: [{}]", isa_set);

    for (i, rec) in isa_set.cpuid_recs().enumerate() {
        println!("{i}\tCPUID RECORD NAME: [{rec}]");

        if let Some(crec) = rec.rec() {
            println!(
                "\t\t{{Leaf 0x{:08x}, subleaf 0x{:08x}, {}[{}:{}] = {}",
                crec.leaf,
                crec.subleaf,
                crec.reg(),
                crec.bit_start,
                crec.bit_end,
                crec.value
            );
        } else {
            println!("Could not find cpuid leaf information");
        }
    }

    Ok(())
}

fn print_branch_hints(xedd: &Inst) -> Result<()> {
    let op = xedd.operands();

    if op.branch_not_taken_hint() {
        println!("HINT: NOT TAKEN");
    } else if op.branch_taken_hint() {
        println!("HINT: TAKEN");
    } else if op.cet_no_track() {
        println!("CET NO-TRACK");
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    tables::init();

    let mut xedd = Inst::new();

    xedd.set_mode(opts.mode, opts.width);

    if let Some(chip) = opts.chip {
        xedd.set_input_chip(chip);
    }

    // convert ascii hex to hex bytes
    let bytes = opts.bytes()?;

    println!("Attempting to decode: {}", hex::encode(&bytes));

    xedd.decode(bytes)?;

    println!(
        "iclass {}\tcategory {}\tISA-extension {}\tISA-set {}",
        xedd.iclass(),
        xedd.category(),
        xedd.extension(),
        xedd.isa_set()
    );
    println!(
        "instruction-length {}\toperand-width {}",
        xedd.length(),
        xedd.operand_width()
    );

    let op = xedd.operands();
    println!("effective-operand-width {}", op.effective_operand_width());
    println!("effective-address-width {}", op.effective_address_width());
    println!("stack-address-width {}", op.stack_address_width());

    println!("iform-enum-name {}", xedd.iform());
    println!(
        "iform-enum-name-dispatch (zero based) {}",
        xedd.iform_dispatch()
    );
    println!("iclass-max-iform-dispatch {}", xedd.iclass().iform_max());

    // operands
    print_operands(&xedd)?;

    // memops
    print_memops(&xedd)?;

    // flags
    print_flags(&xedd)?;
    print_reads_zf_flag(&xedd)?;

    // attributes
    print_attributes(&xedd)?;

    // misc
    print_misc(&xedd)?;
    print_branch_hints(&xedd)?;

    Ok(())
}
