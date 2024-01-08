//! decoder example - printing register operands

use anyhow::{bail, Result};

use clap::Parser;
use xed::{dec::Inst, tables, AddressWidth, Errno, Error, MachineMode, State};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// Address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Qword)]
    pub width: AddressWidth,

    /// Long 64bit mode
    #[arg(long = "64")]
    pub long64: bool,

    /// Legacy 32bit mode
    #[arg(long = "32")]
    pub legacy32: bool,

    /// Legacy 16bit mode
    #[arg(long = "16")]
    pub legacy16: bool,

    /// Change to Long 64bit mode
    #[arg(long)]
    pub change_to_long_mode: bool,

    pub bytes: Vec<String>,
}

impl Opts {
    pub fn state(&self) -> State {
        if self.long64 {
            State::LONG64
        } else if self.legacy32 {
            State::LEGACY32
        } else if self.legacy16 {
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

fn print_operands(xedd: &Inst) {
    let xi = xedd.inst();

    for (i, op) in xi.operands().enumerate() {
        let name = op.name();

        if name.is_register() {
            println!("{:2}: {:5} {:5}", i, xedd.reg(name), op.action());
        }
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    tables::init();

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

    print_operands(&xedd);

    Ok(())
}
