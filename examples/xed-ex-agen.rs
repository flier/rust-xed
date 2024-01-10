//! decoder example with agen callbacks.

/* THIS IS JUST AN EXAMPLE FOR TESTING AND DOES NOT RETURN REAL DATA */
/* THIS IS JUST AN EXAMPLE FOR TESTING AND DOES NOT RETURN REAL DATA */
/* THIS IS JUST AN EXAMPLE FOR TESTING AND DOES NOT RETURN REAL DATA */
/* THIS IS JUST AN EXAMPLE FOR TESTING AND DOES NOT RETURN REAL DATA */

use std::ptr::NonNull;

use anyhow::{bail, Result};

use clap::Parser;
use xed::{
    dec::{agen, Inst},
    fmt, tables, AddressWidth, Errno, Error, MachineMode, Reg, State, Syntax,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// Stack address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Qword)]
    pub width: AddressWidth,

    /// Long 64bit mode
    #[arg(long = "64")]
    pub long64: bool,

    /// Real 16bit mode
    #[arg(short, long)]
    pub real16: bool,

    /// Legacy 32bit mode
    #[arg(long = "32")]
    pub legacy32: bool,

    /// Protect 16bit mode
    #[arg(long = "16")]
    pub protect16: bool,

    pub bytes: Vec<String>,
}

impl Opts {
    pub fn state(&self) -> State {
        if self.long64 {
            State::LONG64
        } else if self.real16 {
            State::REAL16
        } else if self.legacy32 {
            State::LEGACY32
        } else if self.protect16 {
            State::LEGACY16
        } else {
            State::new(self.mode, self.width)
        }
    }

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

fn register_callback(reg: Reg, _: Option<NonNull<()>>) -> Result<u64, Error> {
    /* After we register this function (see xed_agen_register_callback in
     * main()), this function is called as needed by xed_agen().  This
     * function provides register values for xed_agen(). In a real usage,
     * you would probably pass in a context (in your call to xed_agen())
     * that contains the actual values.  */

    /* Note that AL is required for the XLAT instruction. That is the only
    byte reg needed. Also note, that for real mode, raw segement
    selectors are returned by this function.  */

    /* in reality, you'd have to return valid values for each case */

    /* THIS IS JUST AN EXAMPLE FOR TESTING AND DOES NOT RETURN REAL DATA */

    match reg {
        Reg::RAX | Reg::EAX | Reg::AX => Ok(0xAABB_CC00),
        Reg::AL => Ok(0), // FOR XLAT

        Reg::RCX | Reg::ECX | Reg::CX => Ok(0xAABB_CCDD),
        Reg::RDX | Reg::EDX | Reg::DX => Ok(0),
        Reg::RBX | Reg::EBX | Reg::BX => Ok(0x1122_3344),
        Reg::RSP | Reg::ESP | Reg::SP => Ok(0),
        Reg::RBP | Reg::EBP | Reg::BP => Ok(0),
        Reg::RSI | Reg::ESI | Reg::SI => Ok(0x11_2233_4455),
        Reg::RDI | Reg::EDI | Reg::DI => Ok(0x66_5544_3322),
        Reg::R8 | Reg::R8D | Reg::R8W => Ok(0),
        Reg::R9 | Reg::R9D | Reg::R9W => Ok(0),
        Reg::R10 | Reg::R10D | Reg::R10W => Ok(0),
        Reg::R11 | Reg::R11D | Reg::R11W => Ok(0),
        Reg::R12 | Reg::R12D | Reg::R12W => Ok(0),
        Reg::R13 | Reg::R13D | Reg::R13W => Ok(0),
        Reg::R14 | Reg::R14D | Reg::R14W => Ok(0),
        Reg::R15 | Reg::R15D | Reg::R15W => Ok(0),
        Reg::RIP => Ok(0x7990_1000_2000_3000),
        Reg::EIP | Reg::IP => Ok(0),
        Reg::CS | Reg::DS | Reg::ES | Reg::SS | Reg::FS | Reg::GS => Ok(0),
        _ => Ok(0),
    }
}

fn segment_callback(reg: Reg, _: Option<NonNull<()>>) -> Result<u64, Error> {
    /* for protected mode, this function returns the valid segment base values */

    /* in reality, you'd have to return valid valies for each case */
    /* THIS IS JUST AN EXAMPLE FOR TESTING AND DOES NOT RETURN REAL DATA */

    match reg {
        Reg::CS | Reg::DS | Reg::ES | Reg::SS => Ok(0),
        Reg::FS => Ok(0),
        Reg::GS => Ok(0),
        _ => Ok(0),
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    tables::init();

    // register callbacks that provide actual values. These functions will
    // be called by xed_agen() later on when values are needed.
    agen::register_callback(Some(register_callback), Some(segment_callback));

    xed::set_verbosity(99);

    let bytes = opts.bytes()?;

    if bytes.is_empty() {
        bail!("Must supply some hex bytes, e.g., 48 89 C0");
    }

    println!(
        "PARSING BYTES: {}",
        bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let mut xedd = Inst::with_state(opts.state());

    match xedd.decode(&bytes) {
        Ok(_) => {
            println!("{xedd}");
        }
        Err(Error::Errno(Errno::BUFFER_TOO_SHORT)) => {
            bail!("Not enough bytes provided");
        }
        Err(Error::Errno(Errno::GENERAL_ERROR)) => {
            bail!("Could not decode given input.");
        }
        Err(err) => {
            bail!("Unhandled error {err}");
        }
    }

    // print the output a few different ways
    println!("iclass: {}", xedd.iclass());
    println!("iform: {}", xedd.iform());

    for syntax in [Syntax::XED, Syntax::ATT, Syntax::INTEL] {
        if let Some(code) = fmt::context::<()>(syntax, &xedd, None, None, None) {
            println!("{syntax} syntax: {code}");
        } else {
            bail!("Error disassembling {syntax} syntax");
        }
    }

    println!(
        "\nNumber of memory operands: {}",
        xedd.number_of_memory_operands()
    );

    // call the address generation (agen) calculator for each memop.
    // It will call the callbacks you registered at top of main().
    for (i, op) in xedd.mem_operands().enumerate() {
        // The "context" passed to your registered callbacks when they are asked to produce values.
        // In this example we don't require a real context.
        let addr = op.agen(None)?;

        println!("\tMemory agen{i}: 0x{addr:x}");
    }

    Ok(())
}
