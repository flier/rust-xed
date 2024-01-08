use xed::Iclass;

const REPE: [Iclass; 8] = [
    Iclass::REPE_CMPSB,
    Iclass::REPE_CMPSD,
    Iclass::REPE_CMPSQ,
    Iclass::REPE_CMPSW,
    Iclass::REPE_SCASB,
    Iclass::REPE_SCASD,
    Iclass::REPE_SCASQ,
    Iclass::REPE_SCASW,
];

const REPNE: [Iclass; 8] = [
    Iclass::REPNE_CMPSB,
    Iclass::REPNE_CMPSD,
    Iclass::REPNE_CMPSQ,
    Iclass::REPNE_CMPSW,
    Iclass::REPNE_SCASB,
    Iclass::REPNE_SCASD,
    Iclass::REPNE_SCASQ,
    Iclass::REPNE_SCASW,
];

const REP: [Iclass; 18] = [
    Iclass::REP_INSB,
    Iclass::REP_INSD,
    Iclass::REP_INSW,
    Iclass::REP_LODSB,
    Iclass::REP_LODSD,
    Iclass::REP_LODSQ,
    Iclass::REP_LODSW,
    Iclass::REP_MOVSB,
    Iclass::REP_MOVSD,
    Iclass::REP_MOVSQ,
    Iclass::REP_MOVSW,
    Iclass::REP_OUTSB,
    Iclass::REP_OUTSD,
    Iclass::REP_OUTSW,
    Iclass::REP_STOSB,
    Iclass::REP_STOSD,
    Iclass::REP_STOSQ,
    Iclass::REP_STOSW,
];

fn xtest(a: Iclass, b: Iclass) -> bool {
    if a != b {
        println!("MISMATCH: {a} {b}");
    } else {
        println!("MATCH: {a} {b}");
    }

    a != b
}

fn main() {
    xed::tables::init();

    let mut r = false;

    for i in REPE {
        r |= xtest(i, i.norep().unwrap().repe().unwrap());
    }

    for i in REPNE {
        r |= xtest(i, i.norep().unwrap().repne().unwrap());
    }

    for i in REP {
        r |= xtest(i, i.norep().unwrap().rep().unwrap());
    }

    if r {
        eprintln!("SOME MISMATCH");
    } else {
        println!("ALL MATCHES");
    }
}
