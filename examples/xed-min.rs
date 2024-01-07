//! a minimal toy example of using the decoder.

use anyhow::Result;
use array_concat::concat_arrays;
use clap::Parser;

use xed::{dec::Inst, tables, AddressWidth, Errno, MachineMode};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// Address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Qword)]
    pub width: AddressWidth,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    tables::init();

    // create the decoded instruction, and fill in the machine mode (dstate)
    // make up a simple 2Byte instruction to decode
    let itext: [u8; 15] = concat_arrays!([0x0f, 0x85, 0x99, 0x00, 0x00, 0x00], [0; 9]);

    // This is a test of error handling. I vary the instuction length from
    // 0 bytes to 15 bytes.  Normally, you should send in 15 bytes of itext
    // unless you are near the end of a page and don't want to take a page
    // fault or tlb miss. Note, you have to reinitialize the xedd each time
    // you try to decode in to it.

    // Try different instruction lengths to see when XED recognizes an
    // instruction as valid.
    for i in 0..=15 {
        let mut xedd = Inst::with_mode(opts.mode, opts.width);

        match xedd.decode(&itext[..i]) {
            Ok(_) => println!("{} {}", i, Errno::NONE),
            Err(err) => println!("{} {}", i, err),
        }
    }

    Ok(())
}
