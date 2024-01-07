use anyhow::{anyhow, bail, Result};

use xed::{
    enc::{
        disp, imm0, mem_b, mem_bd, mem_bisd, mem_gbisd, other, relbr, simm0, Inst, Operand, Request,
    },
    format, tables, DecodedInst, Iclass, Op, Reg, State, Syntax,
};

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

    // add an lock and xacquire
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

    next().inst2(
        dstate32,
        Iclass::ADD,
        None,
        Reg::EAX,
        mem_gbisd(Reg::FS, Reg::EAX, Reg::ESI, 4, disp(0x11223344, 32), 32),
    );

    // displacment-only LEA
    next().inst2(
        dstate32,
        Iclass::LEA,
        32,
        Reg::EAX,
        mem_bd(None, disp(0x11223344, 32), 32),
    );

    next().inst0(dstate32, Iclass::REPE_CMPSB, None);

    // nondefault effective operand width for 32b mode so we must specify it.
    // XED could figure it out from the opcode, but currently does not.
    next().inst0(dstate32, Iclass::REPE_CMPSW, 16);

    next().inst0(dstate32, Iclass::REPE_CMPSD, None);

    next().inst1(dstate32, Iclass::PUSH, None, Reg::ECX);

    // this one has a nondefault effective operand width for 64b mode so we must specify it.
    // XED could figure this output from the operands, but currently it does not. */
    next().inst2(dstate64, Iclass::XOR, 64, Reg::RCX, Reg::RDX);

    // nondefault effective operand width for 64b mode so we must specify it.
    // XED could figure it out from the opcode, but currently does not.
    next().inst0(dstate64, Iclass::REPE_CMPSQ, 64);

    // here it is ambiguous from the opcode what the effective operand width is.
    // I could use the operand, but do not do that yet.
    next().inst1(dstate64, Iclass::PUSH, 64, Reg::RCX);

    // again, here's one where I could infer that the operation is 64b from the memory operand, but not yet.
    next().inst1(
        dstate64,
        Iclass::PUSH,
        64,
        mem_bd(Reg::RDX, disp(0x11223344, 32), 64),
    );

    // move a 64b quantity in to RAX using only a 64b displacment
    next().inst2(
        dstate64,
        Iclass::MOV,
        64,
        Reg::RAX,
        mem_bd(None, disp(0x1122334455667788, 64), 64),
    );

    next().inst1(
        dstate64,
        Iclass::JMP_FAR,
        64,
        mem_bd(Reg::RAX, disp(0x20, 8), 80),
    );

    next().inst2(dstate64, Iclass::ADD, 64, Reg::RAX, imm0(0x77, 8));

    next().inst2(dstate64, Iclass::ADD, 64, Reg::RAX, imm0(0x44332211, 32));

    next().inst2(dstate64, Iclass::MOV_CR, 64, Reg::CR3, Reg::RDI);

    next().inst2(
        dstate64,
        Iclass::MOV,
        32,
        mem_bisd(Reg::R8, Reg::RBP, 1, disp(0xf8, 8), 32),
        simm0(0x0, 32),
    );

    // example showing how to set the effective address size to 32b in 64b mode.  Normally XED deduces that from the memory operand base register,
    // but in this case the memops are implicit.
    next().inst0(dstate64, Iclass::STOSQ, 64).addr(32);

    next().inst1(dstate32, Iclass::JECXZ, 4, relbr(5, 8));

    next()
        .inst1(dstate64, Iclass::JECXZ, 4, relbr(5, 8))
        .addr(32);

    next().inst1(dstate64, Iclass::JRCXZ, 4, relbr(5, 8));

    next().inst2(
        dstate64,
        Iclass::VBROADCASTSD,
        32,
        Reg::YMM4,
        mem_gbisd(Reg::GS, None, None, None, disp(0x808, 32), 64),
    );

    // example showing how to set EVEX features.
    // increase the number of operands and use xed_other(...)
    next().inst5(
        dstate64,
        Iclass::VADDPS,
        32,
        Reg::XMM1,
        Reg::K1,
        Reg::XMM2,
        mem_b(Reg::RCX, 16),
        other(Op::ZEROING, 1),
    );

    let mut enc_req = Request::from(DecodedInst::new());
    let mut xedd = DecodedInst::new();

    for inst in insts.iter().take(end + 1) {
        let mode = State::from(inst.mode);

        enc_req.reset_mode(mode);

        if !inst.convert_to(&mut enc_req) {
            bail!("conversion to encode request failed")
        }

        let bytes = enc_req.encode()?;

        println!("Result! {}", hex::encode(&bytes));

        xedd.reset_mode(mode);
        xedd.decode(bytes)?;

        let code = format::context::<()>(Syntax::INTEL, &xedd, None, None, None)
            .ok_or_else(|| anyhow!("DISASSEMBLY ERROR"))?;

        println!("\t{code}");
    }

    Ok(())
}
