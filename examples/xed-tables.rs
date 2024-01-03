//! a minimal example of accessing the XED internal tables

use std::fmt;

use anyhow::Result;

use xed::{tables, Inst, Operand, OperandType};

#[derive(Clone, Copy, Debug)]
struct InstFmt<'a>(&'a Inst);

impl<'a> fmt::Display for InstFmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.0.iclass(),
            self.0.iform(),
            self.0.category(),
            self.0.extension(),
            self.0.isa_set()
        )?;

        // Walk the attributes.
        // Generally, you'll know the one you want to query and just access that one directly. */
        write!(f, " ATTRIBUTES: ")?;

        for attr in self.0.attrs() {
            write!(f, "{} ", attr)?;
        }

        writeln!(f)?;

        for (i, op) in self.0.operands().into_iter().enumerate() {
            writeln!(f, "\t{} {}", i, OperandFmt(op))?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct OperandFmt<'a>(&'a Operand);

impl<'a> fmt::Display for OperandFmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.0.name(),
            self.0.visibility(),
            self.0.action(),
            self.0.ty(),
            self.0.xtype(),
        )?;

        match self.0.ty() {
            OperandType::NT_LOOKUP_FN => {
                if let Some(nt) = self.0.nonterminal() {
                    write!(f, " {}", nt)?;
                }
            }
            OperandType::REG => {
                if let Some(reg) = self.0.reg() {
                    write!(f, " {}", reg)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

fn dump_insts() -> Result<()> {
    for (i, inst) in Inst::tables().iter().enumerate() {
        println!("{} {}", i, InstFmt(inst));
    }

    Ok(())
}

fn main() -> Result<()> {
    // initialize the XED tables -- one time.
    tables::init();

    dump_insts()
}
