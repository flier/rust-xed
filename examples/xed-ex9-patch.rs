//! encoder patching example. (uses decoder too)

use anyhow::{bail, Result};

use xed::{
    dec::Inst as DecodedInst,
    enc::{disp, imm0, mem_bd, relbr, Inst, Request},
    fmt, tables, Iclass, Reg, State, Syntax,
};

fn ex_make_patchable_instr(inst: &Inst) -> Result<(DecodedInst, Vec<u8>)> {
    let mut xede = Request::with_mode(inst.mode());

    if !inst.convert_to(&mut xede) {
        bail!("conversion to encode request failed");
    }

    let bytes = xede.encode()?;

    let mut xedd = DecodedInst::with_state(inst.mode());

    xedd.decode(&bytes)?;

    Ok((xedd, bytes))
}

fn main() -> Result<()> {
    // initialize the XED tables -- one time.
    tables::init();

    let mut insts = vec![Inst::default(); 50];

    let dstate32 = State::LEGACY32;
    let dstate64 = State::LONG64;

    let mut x = insts.iter_mut().enumerate();
    let mut end: usize = 0;

    let mut next = || {
        let (n, inst) = x.next().unwrap();
        end = n;
        inst
    };

    next().inst1(dstate64, Iclass::JMP, 64, relbr(0x11223344, 32));

    // using 0 for instructions that have the default effective operand width for their mode.
    // The default effective operand width for 16b mode is 16b.
    // The default effective operand width for 32b and 64b modes is 32b.
    next().inst2(
        dstate32,
        Iclass::XOR_LOCK,
        None,
        mem_bd(Reg::EDX, disp(0x11, 8), 32),
        Reg::ECX,
    );

    next().inst2(
        dstate32,
        Iclass::ADD,
        None,
        Reg::EAX,
        mem_bd(Reg::EDX, disp(0x11223344, 32), 32),
    );

    next().inst2(dstate64, Iclass::ADD, 64, Reg::RAX, imm0(0x77, 8));

    // make patchable instructions (encode -> decode)
    let mut decoded = insts
        .iter()
        .take(end + 1)
        .map(|inst| ex_make_patchable_instr(inst))
        .collect::<Result<Vec<_>>>()?;

    // patch the displacements or immediates
    if let Some((xedd, ref mut itext)) = decoded.get_mut(0) {
        if !xedd.patch_relbr(itext, relbr(0x55667788, 32)) {
            eprintln!("Patching failed for 1st instr");
        }
    }
    if let Some((xedd, ref mut itext)) = decoded.get_mut(1) {
        if !xedd.patch_disp(itext, disp(0x55, 8)) {
            eprintln!("Patching failed for 2nd instr");
        }
    }
    if let Some((xedd, ref mut itext)) = decoded.get_mut(2) {
        if !xedd.patch_disp(itext, disp(0x55446677, 32)) {
            eprintln!("Patching failed for 3rd instr");
        }
    }
    if let Some((xedd, ref mut itext)) = decoded.get_mut(3) {
        if !xedd.patch_imm0(itext, imm0(0x22, 8)) {
            eprintln!("Patching failed for 4th instr");
        }
    }

    // print out the patched instructions
    for (enc, (mut xedd, bytes)) in insts.into_iter().zip(decoded) {
        xedd.reset(enc.mode());
        xedd.decode(bytes)?;

        if let Some(code) = fmt::context::<()>(Syntax::INTEL, &xedd, None, None, None) {
            println!("{code}");
        } else {
            eprintln!("DISASSEMBLY ERROR");
        }
    }

    Ok(())
}
