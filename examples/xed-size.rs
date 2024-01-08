use std::mem;

use xed::{dec, IformInfo, Inst, Operand};

fn main() {
    println!("DecodedInst       {:12}", mem::size_of::<dec::Inst>());
    println!("Inst              {:12}", mem::size_of::<Inst>());
    println!("Operand           {:12}", mem::size_of::<Operand>());
    println!("IformInfo         {:12}", mem::size_of::<IformInfo>());
}
