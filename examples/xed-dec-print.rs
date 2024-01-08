// decode and print

use xed::{dec, fmt, State, Syntax};

fn main() {
    xed::tables::init();

    let mut xedd = dec::Inst::with_state(State::LEGACY32);

    match xedd.decode(&[00, 00]) {
        Ok(code_) => {
            if let Some(code) = fmt::context::<()>(Syntax::ATT, &xedd, None, None, None) {
                println!("{code}")
            } else {
                eprintln!("Error disassembling");
            }
        }
        Err(err) => eprintln!("Decoding error: {err}"),
    }
}
