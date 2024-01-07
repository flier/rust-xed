//! decoder example.

use anyhow::{bail, Result};
use clap::Parser;

use xed::{dec::Inst, format, tables, AddressWidth, Errno, Error, MachineMode, Syntax};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// Address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Qword)]
    pub width: AddressWidth,

    /// Simple XML output format for the Intel syntax disassembly.
    #[arg(long)]
    pub xml: bool,

    /// omit unit scale "*1"
    #[arg(long)]
    pub no_unit_scale: bool,

    /// do not sign extend signed immediates
    #[arg(long)]
    pub no_sign_extend: bool,

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

    pub fn format_options(&self) -> format::Options {
        let mut opts = format::Options::default();

        if self.xml {
            opts.xml_output();
        }
        if self.no_unit_scale {
            opts.omit_unit_scale();
        }
        if self.no_sign_extend {
            opts.no_sign_extend_signed_immediates();
        }

        opts
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    tables::init();

    xed::set_verbosity(99);

    opts.format_options().apply();

    // begin processing of instructions...

    let mut xedd = Inst::new();

    xedd.set_mode(opts.mode, opts.width);

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

    for syntax in [Syntax::XED, Syntax::ATT, Syntax::INTEL] {
        if let Some(code) = format::context::<()>(syntax, &xedd, None, None, None) {
            println!("{syntax} syntax: {code}");
        } else {
            bail!("Error disassembling {syntax} syntax");
        }
    }

    Ok(())
}
