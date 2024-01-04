use std::fmt::Write;

use anyhow::Result;
use clap::Parser;

use xed::{tables, AddressWidth, Chip, DecodedInst, MachineMode, Op, Operand, SignExtend};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Legacy32)]
    pub mode: MachineMode,

    /// Address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Dword)]
    pub width: AddressWidth,

    /// Chip model
    #[arg(short, long)]
    pub chip: Option<Chip>,

    pub bytes: Vec<String>,
}

fn print_operands(xedd: &DecodedInst) -> Result<()> {
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

        let oi = xedd.operand(i as u32);

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

        println!()
    }

    Ok(())
}

fn print_memops(xedd: &DecodedInst) -> Result<()> {
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

fn print_flags(xedd: &DecodedInst) -> Result<()> {
    Ok(())
}

fn print_reads_zf_flag(xedd: &DecodedInst) -> Result<()> {
    Ok(())
}

fn print_attributes(xedd: &DecodedInst) -> Result<()> {
    let xi = xedd.inst();

    print!("ATTRIBUTES: ");
    for attr in xi.attrs() {
        print!("{} ", attr);
    }
    println!();

    Ok(())
}

fn print_misc(xedd: &DecodedInst) -> Result<()> {
    Ok(())
}

fn print_branch_hints(xedd: &DecodedInst) -> Result<()> {
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

    let mut xedd = DecodedInst::new();

    if let Some(chip) = opts.chip {
        xedd.set_chip(chip)
    }

    // convert ascii hex to hex bytes
    let bytes = hex::decode(
        opts.bytes
            .into_iter()
            .map(|s| s.into_bytes())
            .flatten()
            .collect::<Vec<_>>(),
    )?;

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
