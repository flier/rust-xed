//! Encoder example

use anyhow::Result;
use clap::Parser;

use xed::{enc::compile, tables, AddressWidth, MachineMode, State};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Machine mode
    #[arg(short, long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// Stack address width
    #[arg(short, long, value_enum, default_value_t = AddressWidth::Qword)]
    pub width: AddressWidth,

    pub expr: Vec<String>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    tables::init();

    let expr = opts.expr.join(" ");
    let state = State::new(opts.mode, opts.width);
    let mut req = compile(&expr, state)?;

    println!("Encode request: {}", req);

    match req.encode() {
        Ok(bytes) => {
            println!("Encodable! {}", hex::encode(bytes));
        }
        Err(err) => {
            println!("Could not encode, {}", err);
        }
    }

    Ok(())
}
