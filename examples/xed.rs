use std::{
    borrow::Cow,
    cmp::min,
    collections::HashMap,
    ffi::CStr,
    fs::{self, File},
    io::{BufRead, Cursor},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use derive_more::{Deref, Display};
use lazy_static::lazy_static;
use log::{debug, error, info, trace};
use memmap2::Mmap;
use object::{read::Object, ObjectSection, ObjectSymbol, SectionKind};
use prometheus::{
    register_counter, register_histogram, register_int_counter, Counter, Histogram, IntCounter,
};
use xed::{
    dec::Inst, enc::compile, fmt, AddressWidth, Attribute, Category, Chip, Errno, Error, Extension,
    Iclass, MachineMode, State, Syntax,
};

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// decode file
    #[arg(short)]
    pub input_file: Option<PathBuf>,

    /// just decode one instruction
    #[arg(short, long)]
    pub just_decode_one: bool,

    /// decode ascii hex bytes after prefix
    #[arg(short, long)]
    pub prefix: Option<String>,

    /// decode a sequence of bytes, must be last
    #[arg(short = 'd')]
    pub decode_bytes: bool,

    /// encode expression
    #[arg(short = 'e', long)]
    pub encode: bool,

    /// assemble the contents of the file
    #[arg(short, long = "ie")]
    pub assemble_file: Option<PathBuf>,

    /// decode-then-encode
    #[arg(long = "de")]
    pub decode_encode: bool,

    /// XED engine verbosity
    #[arg(short, long)]
    pub verbosity: Option<i32>,

    /// count instructions that are not valid for CHIP
    #[arg(long = "chip-check")]
    pub chip: Option<Chip>,

    /// list the valid chips
    #[arg(long)]
    pub chip_check_list: bool,

    /// target section for file disassembly
    #[arg(short = 's', long = "section")]
    pub target_section: Option<String>,

    /// number of instructions to decode
    #[arg(short = 'n')]
    pub ninsts: Option<usize>,

    /// Base address offset, for DLLs/shared libraries. (Use 0x for hex addresses)
    #[arg(short, long)]
    pub base: Option<u64>,

    /// Address to start disassembling. (Use 0x for hex addresses)
    #[arg(long)]
    pub start: Option<u64>,

    /// Address to end disassembling. (Use 0x for hex addresses)
    #[arg(long)]
    pub end: Option<u64>,

    /// Disable symbol-based resynchronization algorithm for disassembly
    #[arg(long)]
    pub no_resync: bool,

    /// Show the AVX/SSE transition classfication
    #[arg(long)]
    pub ast: bool,

    /// Histogram decode times
    #[arg(long)]
    pub histogram: bool,

    /// Intel syntax for disassembly
    #[arg(short = 'I', long)]
    pub intel: bool,

    /// ATT SYSV syntax for disassembly
    #[arg(short = 'A', long)]
    pub att: bool,

    /// XED syntax for disassembly
    #[arg(short = 'X', long)]
    pub xed: bool,

    /// syntax for disassembly
    #[arg(long, default_value_t = Syntax::INTEL)]
    pub syntax: Syntax,

    /// Emit the XED "ISA set" in dissasembly
    #[arg(long = "isa-set")]
    pub emit_isa_set: bool,

    /// XML formatting
    #[arg(long)]
    pub xml: bool,

    /// upper case hex formatting
    #[arg(short = 'U', long)]
    pub uppercase_hex: bool,

    /// positive memory displacement formatting
    #[arg(long = "pmd")]
    pub positive_memory_displacements: bool,

    /// Format AVX512 without curly braces for writemasks, include k0
    #[arg(long = "nwm")]
    pub no_write_mask_curly_k0: bool,

    /// Output __emit statements for the Intel compiler
    #[arg(long)]
    pub emit: bool,

    /// Read symbol table in "nm" format from file
    #[arg(short = 'S')]
    pub symbol_table_file: Option<PathBuf>,

    /// Emit line number information, if present
    #[arg(long)]
    pub line: bool,

    /// Emit a register dependence graph file in dot format.
    #[arg(long)]
    pub dot: bool,

    /// Machine mode
    #[arg(long, value_enum, default_value_t = MachineMode::Long64)]
    pub machine_mode: MachineMode,

    /// Stack address width
    #[arg(long, value_enum, default_value_t = AddressWidth::Qword)]
    pub stack_address_width: AddressWidth,

    /// for REAL_16 mode, 16b addressing (20b addresses), 16b default data size
    #[arg(long)]
    pub real: bool,

    /// for REAL_32 mode, 16b addressing (20b addresses), 32b default data size
    #[arg(long)]
    pub real32: bool,

    /// for LEGACY_16 mode, 16b addressing, 16b default data size
    #[arg(long)]
    pub legacy16: bool,

    /// for LEGACY_32 mode, 32b addressing, 32b default data size -- default
    #[arg(long)]
    pub legacy32: bool,

    /// for LONG_64 mode w/64b addressing
    #[arg(long)]
    pub long64: bool,

    /// 32b stack addressing, default, not in LONG_64 mode
    #[arg(long = "s32")]
    pub stack_address_width32: bool,

    /// 16b stack addressing, not in LONG_64 mode
    #[arg(long = "s16")]
    pub stack_address_width16: bool,

    /// Search path for windows symbols
    #[arg(long)]
    pub search_path: Vec<PathBuf>,

    pub args: Vec<String>,
}

impl Opts {
    pub fn state(&self) -> State {
        let width = if self.stack_address_width16 {
            AddressWidth::Word
        } else if self.stack_address_width32 {
            AddressWidth::Dword
        } else {
            self.stack_address_width
        };

        if self.real {
            State::REAL16
        } else if self.real32 {
            State::REAL32
        } else if self.legacy16 {
            State::LEGACY16
        } else if self.legacy32 {
            State::LEGACY32
        } else if self.long64 {
            State::LONG64
        } else {
            State::new(self.machine_mode, width)
        }
    }

    pub fn syntax(&self) -> Syntax {
        if self.intel {
            Syntax::INTEL
        } else if self.att {
            Syntax::ATT
        } else if self.xed {
            Syntax::XED
        } else {
            self.syntax
        }
    }

    pub fn fmt_opts(&self) -> fmt::Options {
        let mut opts = fmt::Options::default();

        if self.xml {
            opts.xml_output().xml_flags();
        }
        if self.positive_memory_displacements {
            opts.positive_memory_displacements();
        }
        if !self.uppercase_hex {
            opts.lowercase_hex();
        }
        if !self.no_write_mask_curly_k0 {
            opts.write_mask_curly_k0();
        }

        opts
    }

    pub fn formatter(&self) -> fmt::Formatter {
        let mut fmt = fmt::Formatter::new();

        if let Some(base) = self.base {
            fmt.runtime_address(base);
        }

        fmt.syntax(self.syntax()).options(self.fmt_opts());

        fmt
    }
}

fn assemble(state: State, file: &Path, emit_intel_asm: bool) -> Result<()> {
    let mut buf = fs::read(file)?;
    let cur = Cursor::new(&mut buf);

    for line in cur.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        encode(state, &line, emit_intel_asm)?;
    }

    Ok(())
}

fn encode(state: State, expr: &str, emit_intel_asm: bool) -> Result<()> {
    let mut req = compile(expr, state)?;

    debug!("Request: {}", req);

    let bytes = req.encode()?;

    if emit_intel_asm {
        for b in bytes {
            println!("__emit 0x{:02x}", b);
        }
    } else {
        println!(
            ".byte {}",
            bytes
                .into_iter()
                .map(|b| format!("0x{b:02x}"))
                .collect::<Vec<_>>()
                .join(",")
        );
    }

    Ok(())
}

lazy_static! {
    static ref TOTAL_SECONDS: Counter =
        register_counter!("total_seconds", "total seconds to decode instruction").unwrap();
    static ref TOTAL_INSTS: IntCounter =
        register_int_counter!("total_insts", "total decoded instructions").unwrap();
    static ref CHIP_CHECK_ERRORS: IntCounter = register_int_counter!(
        "chip_check_errors",
        "total decoded errors for the input chip"
    )
    .unwrap();
    static ref ENCODE_TIME: Histogram = register_histogram!("encode_time", "encode time").unwrap();
    static ref DECODE_TIME: Histogram = register_histogram!("decode_time", "decode time").unwrap();
}

#[derive(Clone, Debug, Deref)]
struct Decoder<'a> {
    #[deref]
    opts: &'a Opts,
    line_numbers: Option<LineNumbers>,
}

impl<'a> Decoder<'a> {
    pub fn new(opts: &'a Opts) -> Self {
        Self {
            opts,
            line_numbers: None,
        }
    }

    pub fn stats_update(&self, elapsed: f64) {
        TOTAL_SECONDS.inc_by(elapsed);
        TOTAL_INSTS.inc();
    }

    pub fn decode_encode(&self, mut xedd: Inst, bytes: &[u8]) -> Result<()> {
        self.decode(&mut xedd, bytes)?;

        // convert decode structure to proper encode structure
        let mut req = xedd.request();

        trace!("encode request: {}", &req);

        // encode it again...
        let t = ENCODE_TIME.start_timer();
        let encoded = req.encode()?;
        let elapsed = Duration::from_secs_f64(t.stop_and_record());

        trace!("encode time: {:?}", elapsed);
        trace!("encodable: {:?}", &encoded);

        if encoded == bytes {
            println!("Identical re-encoding");
        } else {
            eprintln!(
                "Discrepenacy after re-encoding. dec_len={} {:?} enc_olen={} {:?}",
                xedd.length(),
                &bytes[..xedd.length() as usize],
                encoded.len(),
                encoded,
            );
        }

        Ok(())
    }

    pub fn decode(&self, xedd: &mut Inst, bytes: &[u8]) -> Result<()> {
        trace!("decode bytes: {:?}", bytes);

        let t = DECODE_TIME.start_timer();
        xedd.decode(bytes)?;
        let elapsed = Duration::from_secs_f64(t.stop_and_record());

        trace!("decode time: {:?}", elapsed);
        debug!("decoded inst: {}", xedd);

        if xedd.valid() {
            info!(
                "iclass: {}, category: {}, extension: {}, iform: {}, isa_set: {}, attributes: {}",
                xedd.iclass(),
                xedd.category(),
                xedd.extension(),
                xedd.iform(),
                xedd.isa_set(),
                xedd.attrs()
                    .map(|attr| attr.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );

            self.disassemble(xedd)?;
            println!();
        }

        Ok(())
    }

    pub fn disassemble(&self, xedd: &Inst) -> Result<()> {
        let mut buf = vec![0; 4096];

        if let Some(s) = self
            .formatter()
            .format(xedd, buf.as_mut_slice())
            .then(|| CStr::from_bytes_until_nul(&buf))
            .transpose()?
            .map(|s| s.to_str())
            .transpose()?
        {
            print!("{}", s);
        } else {
            eprintln!("Error disassembling {} syntax.", self.syntax());
        }

        Ok(())
    }

    pub fn xed_disas_file(&'a self, obj: object::File) -> Result<()> {
        let symbols = obj
            .symbols()
            .chain(obj.dynamic_symbols())
            .flat_map(|sym| sym.name().map(|name| ((sym.address(), name.to_string()))))
            .collect::<HashMap<_, _>>();

        debug!("found {} symbols", symbols.len());

        let emit_sym = |addr| {
            if let Some(name) = symbols.get(&addr) {
                if self.xml {
                    println!("\n<SYM>{name}</SYM>");
                } else {
                    println!("\nSYM {name}:");
                }
            }
        };

        for sec in obj.sections() {
            if sec.kind() != SectionKind::Text {
                continue;
            }

            if let Some(ref name) = self.target_section {
                if sec.name()? != name {
                    continue;
                }
            }

            let (offset, size) = sec.file_range().unwrap();

            if !self.xml {
                println!(
                    "# SECTION {} {} addr 0x{:08x} offset 0x{:08x} size {}",
                    sec.index().0,
                    sec.name()?,
                    sec.address(),
                    offset,
                    size
                );
            }

            let mut data = sec.data()?;
            let mut off = 0;

            // for skipping long strings of zeros
            let mut skipping = false;
            let mut last_all_zeros = false;

            let mut xedd = Inst::with_state(self.state());

            if let Some(chip) = self.chip {
                xedd.set_input_chip(chip);
            }

            for i in 0..self.ninsts.unwrap_or(usize::MAX) {
                // if we get near the end of the section, clip the itext length
                let ilim = min(Inst::MAX_BYTES, data.len());

                // if we get two full things of 0's in a row, start skipping.
                if data.iter().take(ilim).all(|&b| b == 0) {
                    if skipping {
                        (_, data) = data.split_at(ilim);
                        continue;
                    }

                    if last_all_zeros {
                        println!("...");
                        (_, data) = data.split_at(ilim);
                        skipping = true;
                        continue;
                    }

                    last_all_zeros = true;
                } else {
                    skipping = false;
                    last_all_zeros = false;
                }

                let mut buf = &data[..ilim];

                trace!("decoding instruction {i} {:?}", buf);

                let t = DECODE_TIME.start_timer();
                let res = xedd.decode(buf);
                let elapsed = t.stop_and_record();
                self.stats_update(elapsed);

                let mut len = xedd.length();

                let addr = sec.address() + off as u64;

                if res.is_ok() && len == 0 {
                    error!("Zero length on decoded instruction!");

                    self.decode_error(addr, buf);
                }

                if data.len() == len as usize {
                    break;
                }

                if len == 0 {
                    len = 1;
                }

                (buf, data) = data.split_at(len as usize);
                off += len;

                match res {
                    Ok(_) => {}
                    Err(Error::Errno(Errno::INVALID_FOR_CHIP)) => {
                        CHIP_CHECK_ERRORS.inc();
                    }
                    _ => {
                        self.decode_error(addr, buf);
                    }
                }

                emit_sym(addr);

                self.emit_disasm(&xedd, addr, buf)?;

                xedd.reset(self.state());

                if let Some(chip) = self.chip {
                    xedd.set_input_chip(chip);
                }
            }

            if !self.xml {
                if self.end.is_some() {
                    println!("# end of range.");
                } else {
                    println!("# end of text section.");
                }
            }
        }

        Ok(())
    }

    fn emit_disasm(&self, xedd: &Inst, addr: u64, bytes: &[u8]) -> Result<()> {
        debug!("{}", xedd);

        if self.xml {
            self.emit_xml(&xedd, addr, bytes)?;
        } else {
            print!("{addr:08x}: ");
            self.emit_cat_ext_ast(&xedd);
            self.emit_hex(bytes);
            self.disassemble(&xedd)?;
            self.emit_line_num(addr);
            println!();
        }

        Ok(())
    }

    fn emit_xml(&self, xedd: &Inst, addr: u64, bytes: &[u8]) -> Result<()> {
        println!("<ASMLINE>");
        println!("\t<ADDR>{addr:08x}</ADDR>");
        println!("\t<CATEGORY>{}</CATEGORY>", xedd.category());
        println!("\t<EXTENSION>{}</EXTENSION>", xedd.extension());
        println!("\t<ITEXT>{}</ITEXT>", hex::encode(bytes));
        self.disassemble(&xedd)?;
        println!("</ASMLINE>");

        Ok(())
    }

    fn emit_cat_ext_ast(&self, xedd: &Inst) {
        if self.ast {
            print!("{:<11}", classify_avx_sse(xedd));
        } else {
            self.emit_cat_ext(xedd);
        }
    }

    fn emit_cat_ext(&self, xedd: &Inst) {
        print!("{:<9} ", xedd.category().to_string());
        print!("{:<10} ", xedd.extension().to_string());

        if self.emit_isa_set {
            print!("{:<10} ", xedd.isa_set().to_string());
        }
    }

    fn emit_hex(&self, buf: &[u8]) {
        print!(
            "\t {:<30} ",
            buf.iter()
                .map(|b| format!("{b:02x}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    fn emit_line_num(&self, addr: u64) {
        if let Some(ref line_numbers) = self.line_numbers {
            if let Some(loc) = line_numbers.locations.get(&addr) {
                if let Some(path) = line_numbers.files.get(&loc.file) {
                    print!("\t# {}:{}", path.display(), loc.line);
                }
            }
        }
    }

    fn decode_error(&self, addr: u64, buf: &[u8]) {
        error!("could not decode at 0x{:08x}: {:?}", addr, buf);
    }
}

fn classify_avx_sse(xedd: &Inst) -> AstInput {
    let ext = xedd.extension();
    let iclass = xedd.iclass();

    match iclass {
        Iclass::VZEROALL => AstInput::VZeroAll,
        Iclass::VZEROUPPER => AstInput::VZeroUpper,
        Iclass::XRSTOR => AstInput::XRstor,
        _ => {
            if is_interesting_avx(ext) {
                avx_type(xedd)
            } else if is_sse(xedd) {
                AstInput::SSE
            } else {
                AstInput::Nothing
            }
        }
    }
}

fn is_interesting_avx(ext: Extension) -> bool {
    matches!(
        ext,
        Extension::AVX
            | Extension::FMA
            | Extension::F16C
            | Extension::AVX2
            | Extension::AVX2GATHER
            | Extension::AVX512EVEX
    )
}

fn is_sse(xedd: &Inst) -> bool {
    match xedd.extension() {
        Extension::SSE => !matches!(xedd.category(), Category::MMX | Category::PREFETCH),
        Extension::SSE2 | Extension::SSSE3 | Extension::SSE4 => xedd.category() != Category::MMX,
        Extension::AES | Extension::PCLMULQDQ | Extension::SHA => true,
        _ => false,
    }
}

fn avx_type(xedd: &Inst) -> AstInput {
    let avx512 = xedd.extension() == Extension::AVX512EVEX;

    // scalar ops are implicitly 128b
    if xedd.has_attr(Attribute::SIMD_SCALAR) {
        return if avx512 {
            AstInput::EvexScalar
        } else {
            AstInput::AvxScalar
        };
    }

    // look at the VEX.VL field
    match xedd.vl() {
        0 => {
            if avx512 {
                AstInput::Evex128
            } else {
                AstInput::Avx128
            }
        }
        1 => {
            if avx512 {
                AstInput::Evex256
            } else {
                AstInput::Avx256
            }
        }
        2 => AstInput::Evex512,
        _ => AstInput::Nothing,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
enum AstInput {
    #[display("n/a")]
    Nothing,
    #[display("sse")]
    SSE,
    #[display("avx.scalar")]
    AvxScalar,
    #[display("avx.128")]
    Avx128,
    #[display("avx.256")]
    Avx256,
    #[display("vzeroall")]
    VZeroAll,
    #[display("vzeroupper")]
    VZeroUpper,
    #[display("xrstor")]
    XRstor,
    #[display("evex.scalar")]
    EvexScalar,
    #[display("evex.128")]
    Evex128,
    #[display("evex.256")]
    Evex256,
    #[display("evex.512")]
    Evex512,
}

#[derive(Clone, Debug)]
struct LineNumbers {
    files: HashMap<u64, PathBuf>,
    locations: HashMap<u64, Location>,
}

#[derive(Clone, Debug, PartialEq)]
struct Location {
    file: u64,
    line: u64,
    column: u64,
}

fn read_dwarf_line_numbers(object: &object::File) -> Result<LineNumbers> {
    let endian = if object.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };

    // Load a section and return as `Cow<[u8]>`.
    let load_section = |id: gimli::SectionId| -> Result<Cow<[u8]>, gimli::Error> {
        match object.section_by_name(id.name()) {
            Some(ref section) => Ok(section
                .uncompressed_data()
                .unwrap_or(Cow::Borrowed(&[][..]))),
            None => Ok(Cow::Borrowed(&[][..])),
        }
    };

    // Load all of the sections.
    let dwarf_cow = gimli::Dwarf::load(&load_section)?;

    // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
    let borrow_section: &dyn for<'s> Fn(
        &'s Cow<[u8]>,
    ) -> gimli::EndianSlice<'s, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(&*section, endian);

    // Create `EndianSlice`s for all of the sections.
    let dwarf = dwarf_cow.borrow(&borrow_section);

    let mut files = HashMap::new();
    let mut locations = HashMap::new();

    // Iterate over the compilation units.
    let mut iter = dwarf.units();
    while let Some(header) = iter.next()? {
        let unit = dwarf.unit(header)?;

        // Get the line program for the compilation unit.
        if let Some(program) = unit.line_program.clone() {
            let comp_dir = if let Some(ref dir) = unit.comp_dir {
                PathBuf::from(dir.to_string_lossy().into_owned())
            } else {
                PathBuf::new()
            };

            // Iterate over the line program rows.
            let mut rows = program.rows();
            while let Some((header, row)) = rows.next_row()? {
                if !row.end_sequence() {
                    // Determine the path. Real applications should cache this for performance.
                    let mut path = PathBuf::new();
                    if let Some(file) = row.file(header) {
                        path = comp_dir.clone();

                        // The directory index 0 is defined to correspond to the compilation unit directory.
                        if file.directory_index() != 0 {
                            if let Some(dir) = file.directory(header) {
                                path.push(
                                    dwarf.attr_string(&unit, dir)?.to_string_lossy().as_ref(),
                                );
                            }
                        }

                        path.push(
                            dwarf
                                .attr_string(&unit, file.path_name())?
                                .to_string_lossy()
                                .as_ref(),
                        );
                    }

                    let file = row.file_index();

                    // Determine line/column. DWARF line/column is never 0, so we use that
                    // but other applications may want to display this differently.
                    let line = match row.line() {
                        Some(line) => line.get(),
                        None => 0,
                    };
                    let column = match row.column() {
                        gimli::ColumnType::LeftEdge => 0,
                        gimli::ColumnType::Column(column) => column.get(),
                    };

                    files.insert(file, path);
                    locations.insert(row.address(), Location { file, line, column });
                }
            }
        }
    }

    debug!(
        "found {} line numbers in {} files",
        locations.len(),
        files.len()
    );

    Ok(LineNumbers { files, locations })
}

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let opts = Opts::parse();

    // initialize the XED tables -- one time.
    debug!("Initializing XED tables...");
    xed::tables::init();
    trace!("Done initialing XED tables.");

    xed::set_verbosity(opts.verbosity.unwrap_or(3));

    opts.fmt_opts().apply();

    debug!("XED version: [{}]", xed::version());

    let mut xedd = Inst::with_state(opts.state());

    if let Some(chip) = opts.chip {
        xedd.set_input_chip(chip);
    }

    let mut d = Decoder::new(&opts);

    if let Some(ref file) = opts.assemble_file {
        assemble(opts.state(), &file, opts.emit)?;
    } else if opts.encode {
        let expr = opts.args.join(" ");

        encode(opts.state(), &expr, opts.emit)?;
    } else if opts.decode_encode {
        let bytes = hex::decode(opts.args.join(""))?;

        d.decode_encode(xedd, &bytes)?;
    } else if opts.decode_bytes {
        let bytes = hex::decode(opts.args.join(""))?;
        let mut buf = bytes.as_slice();

        while !buf.is_empty() {
            d.decode(&mut xedd, buf)?;

            if opts.just_decode_one {
                break;
            }

            (_, buf) = bytes.split_at(xedd.length() as usize);

            xedd.reset(opts.state());

            if let Some(chip) = opts.chip {
                xedd.set_input_chip(chip);
            }
        }
    } else {
        if opts.xml {
            println!("<?xml version=\"1.0\"?>");
            println!("<XEDDISASM>");
            println!("<XEDFORMAT>1</XEDFORMAT>");
        }

        if let Some(ref file) = opts.input_file {
            let f = File::open(file)?;
            let mm = unsafe { Mmap::map(&f)? };
            let obj = object::File::parse(&*mm)?;

            if opts.line {
                d.line_numbers = read_dwarf_line_numbers(&obj).map(Some)?;
            }

            d.xed_disas_file(obj)?;
        }

        if opts.xml {
            println!("</XEDDISASM>");
        }
    }

    Ok(())
}
