#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use core::panic::PanicInfo;
use core::str::FromStr;
use mos_hardware::mega65::lpeek;
use mos_hardware::mega65::set_lower_case;
//use mos_hardware::mega65::libc::cputs;
use mos_hardware::mega65::libc::mega65_fast;
use mos_hardware::mega65::lpoke;

use ufmt_stdio::*;

const RVS_ON: &str = "\x12";
const RVS_OFF: &str = "\u{0092}";
/// pf$ = type_suffix
const _TYPE_SUFFIX: [&str; 4] = ["", "%", "$", "&"];

/// as at writing, rust doesn't allow for-loops in compile-time evaluation, hence the while-loop
const BIN_CONV: [u16; 16] = {
    let mut arr = [0; 16];
    arr[0] = 1;
    let mut i = 1;
    while i < 16 {
        arr[i] = arr[i - 1] * 2;
        i += 1;
    }
    arr
};

// rw$
const _TOKENS: [&str; 190] = [
    "print",
    "input",
    "if",
    "then",
    "else",
    "do",
    "loop",
    "while",
    "until",
    "gosub",
    "goto",
    "open",
    "close",
    "dopen",
    "dclose",
    "for",
    "next",
    "getkey",
    "hex$",
    "dim",
    "peek",
    "poke",
    "wait",
    "dec",
    "chr$",
    "asc",
    "sgn",
    "sqr",
    "graphic",
    "clr",
    "screen",
    "def",
    "begin",
    "bend",
    "len",
    "mid$",
    "right$",
    "left$",
    "instr",
    "for",
    "next",
    "step",
    "trap",
    "border",
    "and",
    "foreground",
    "background",
    "set",
    "abs",
    "sin",
    "cos",
    "tan",
    "log",
    "fre",
    "cursor",
    "pixel",
    "window",
    "rwindow",
    "line",
    "box",
    "circle",
    "ellipse",
    "palette",
    "restore",
    "data",
    "err$",
    "er",
    "el",
    "cursor",
    "on",
    "off",
    "val",
    "scratch",
    "return",
    "rnd",
    "stop",
    "bank",
    "ti",
    "do",
    "or",
    "st",
    "if",
    "el",
    "er",
    "on",
    "to",
    "pen",
    "get",
    "end",
    "int",
    "not",
    "ds",
    "run",
    "using",
    "append",
    "atn",
    "auto",
    "backup",
    "bload",
    "boot",
    "bsave",
    "bump",
    "bverify",
    "catalog",
    "change",
    "char",
    "cmd",
    "collision",
    "color",
    "concat",
    "cont",
    "copy",
    "wpoke",
    "wpeek",
    "setbit",
    "dclear",
    "deffn",
    "delete",
    "fn",
    "dir",
    "disk",
    "dload",
    "dma",
    "dmode",
    "dpat",
    "dsave",
    "dverify",
    "edma",
    "envelope",
    "erase",
    "exit",
    "exp",
    "fast",
    "filter",
    "find",
    "go64",
    "header",
    "help",
    "highlight",
    "joy",
    "list",
    "load",
    "locate",
    "lpen",
    "mod",
    "monitor",
    "mouse",
    "movspr",
    "new",
    "paint",
    "play",
    "pointer",
    "polygon",
    "pos",
    "pot",
    "pudef",
    "rclr",
    "rdot",
    "read",
    "record",
    "rem",
    "rename",
    "resume",
    "rgraphic",
    "rmouse",
    "rplay",
    "rreg",
    "rspcolor",
    "rsppos",
    "rsprite",
    "save",
    "scnclr",
    "sleep",
    "slow",
    "sound",
    "spc",
    "sprcolor",
    "sprite",
    "sprsav",
    "sys",
    "tab",
    "tempo",
    "troff",
    "tron",
    "type",
    "usr",
    "verify",
    "vol",
    "xor",
    "key",
];

struct Label {
    /// lb$ = label name
    name: String,
    /// ll$ = (post-processed line)
    pp_line: u16,
}

/*fn print(s: String) {
    let cstr: Vec<u8> = Vec::with_capacity(s.len() + 1);
    let x = 0;
    let ptr = &s[..].as_ptr();

    for x in 0..s.len() {
        cstr[x] = *ptr;
        ptr += 1;
    }
}*/

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut current_line = String::new();
    let mut verbose = true;
    let mut pp_line: u16 = 0;
    let mut delete_line_flag: bool = false;
    let mut labels: Vec<Label> = Vec::with_capacity(200);

    prepare_test_memory(&mut verbose);

    // ln%() = map_gen_line_to_orig_line[]
    let _map_gen_line_to_orig_line: [u16; 500] = [0; 500];

    set_lower_case();
    println!("testing TESTING 1, 2, 3...");

    // li$() = processed_lines
    // NOTE: Seems like rust chokes if this is too large?
    let _processed_lines: Vec<String> = Vec::with_capacity(200);

    set_lower_case();
    println!("{}eleven PREPROCESSOR V0.4.7{}", RVS_ON, RVS_OFF);

    //unsafe { cputs("hello".as_ptr()); }
    println!();

    // tl$ = tl_string
    let mut _tl_string = String::new(); //String::from("                                                                                ");
                                        // bl$ = bl_string
                                        //let mut bl_string: String = String::new();
                                        //bl_string.push_str(&tl_string[..]);
                                        //bl_string.push_str(&tl_string[..]);
                                        //bl_string.push_str(&tl_string[..]);

    // tl_string = String::new();

    //for i in 0..tokens.len() {
    //    println!("{}", tokens[i]);
    //}

    //let mystring = String::from("test");
    //println!("{}", &mystring[..]);

    let filename = get_filename(&mut verbose);

    unsafe {
        mega65_fast();
    }

    println!("{}", &filename[..]);

    // ------------------- pass 1 ---------------
    // nl = next_line_flag
    let mut _next_line_flag = false;

    // wh$ = whitespace_chars
    const WHITESPACE_CHARS: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

    // clean up temporary files
    // NOTE: sl/source_line_counter and rl/current_line_index serve the same purpose
    // so I will remove 'current_line_index'
    let mut source_line_counter = 0; // sl
    let mut _post_proc_line_counter = 0; // ln

    // TODO: 195 clr ti: rem keep start time for timing
    const CB_ADDR: u32 = 0x8010000;
    let mut ca_addr: u32 = CB_ADDR;

    println!("PASS 1 ");

    // rl = current_line_index (zero-indexed, increments by one)
    // removing this one, as it's equivalent to sl/soure_line_counter
    //let mut current_line_index = 0;
    // tl = total_lines
    let mut _total_lines: u16 = lpeek(ca_addr) as u16 + 256 * lpeek(ca_addr + 1) as u16;

    ca_addr += 2;

    pp_line = 0; // ln = index into li$ (current post-processed line)

    //200
    while source_line_counter != _total_lines {
        copy_data_to_current_line(&mut ca_addr, &mut current_line);

        println!("l{}: {}", source_line_counter, &current_line[..]);

        // 340
        current_line = String::from(trim_left(&current_line[..], &WHITESPACE_CHARS[..]));
        println!("{}", &current_line[..]);

        single_quote_comment_trim(&mut current_line);

        //560-580
        if current_line.len() > 0 {
            current_line = String::from(trim_right(&current_line[..], &WHITESPACE_CHARS[..]));
            //println!("'{}'", &current_line[..]);
        }

        //585
        if current_line.len() > 0 {
            // dl = delete_line_flag
            delete_line_flag = false;
            if verbose {
                println!(
                    ">> {} {} {}",
                    _post_proc_line_counter,
                    source_line_counter,
                    &current_line[..]
                );
                // 600
                if (&current_line[..1]).eq(".") {
                    println!("dot!");
                    _next_line_flag = true;
                    parse_label(
                        verbose,
                        &current_line,
                        pp_line,
                        &mut delete_line_flag,
                        &mut labels,
                    );
                }
                // 601
                if (&current_line[..1]).eq("#") {
                    if (*current_line).find("ifdef") = Some(2) {
                        let def_str = &current_line[7..];
                        check_if_define_exists(def_str);
                        delete_line_flag = true;
                    }
                }
            }
        }

        // 750
        source_line_counter += 1;
    }

    0
}

fn single_quote_comment_trim(current_line: &mut String) {
    //422
    if current_line.find('\'').is_none() || current_line.find('"').is_none() {
        return;
    }
    //423
    //424
    let mut quote_flag = false;
    let mut cut_tail_idx = None;
    //440
    for (in_line_idx, c) in current_line.chars().enumerate() {
        //let c = (*current_line).chars().nth(in_line_idx).unwrap();
        match c {
            '"' => quote_flag = !quote_flag,
            '\'' => {
                if !quote_flag {
                    cut_tail_idx = Some(in_line_idx);
                    break;
                }
            }
            _ => (),
        }
    }
    //540
    if cut_tail_idx.is_some() {
        *current_line = current_line[..cut_tail_idx.unwrap()].to_string();
    }
    //println!("'{}'", &(*current_line)[..]);
}

/// @todo: skip `current_line` as argument as it is zeroed
fn copy_data_to_current_line(ca_addr: &mut u32, current_line: &mut String) {
    *current_line = String::new();
    let mut idx: u8 = 0;
    let line_length = lpeek(*ca_addr) as u32;
    *ca_addr += 1;

    (*ca_addr..*ca_addr + line_length)
        .for_each(|address| (*current_line).push(lpeek(address) as char));

    *ca_addr += line_length;

    // while idx < line_length {
    //     (*current_line).push(lpeek(*ca_addr) as char);
    //     *ca_addr += 1;
    //     idx += 1;
    // }
}

/// Parse a single label and add it to the `labels` vector
///
/// ## Note
///
///   Original source code: 1500
///
/// ## Todo
///
/// return the label and let the caller add to `labels`.
fn parse_label(
    verbose: bool,
    current_line: &str,
    pp_line: u16,
    delete_line_flag: &mut bool,
    labels: &mut Vec<Label>,
) {
    if verbose {
        println!("label {} at pp_line {}", *current_line[..], pp_line);
    }
    *delete_line_flag = true;
    (*labels).push(Label {
        name: String::from(&((*current_line)[1..])),
        pp_line: pp_line + 1,
    });
}

fn trim_left<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = 0;
    while i < line.len() && trim_chars.contains(&line.as_bytes()[i]) {
        i += 1;
    }
    &line[i..]
}

fn trim_right<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = (line.len() - 1) as i16;

    while i >= 0 && trim_chars.contains(&line.as_bytes()[i as usize]) {
        i = i - 1;
    }

    &line[..((i + 1) as usize)]
}

fn prepare_test_memory(verbose: &mut bool) {
    // turn on verbose flag
    // (in memory doesn't work yet, as I'd have to put dummy info into 0x4ff00 to be parsed by get_filename()
    // unsafe { lpoke(0x4ff07u32, 0x08u8); }

    // so for now, just hardcode the flag
    *verbose = true;

    const DATA: [u8; 97] = [
        0x08, 0x00, 0x0f, 0x23, 0x4f, 0x55, 0x54, 0x50, 0x55, 0x54, 0x20, 0x22, 0x48, 0x45, 0x4c,
        0x4c, 0x4f, 0x22, 0x00, 0x0a, 0x23, 0x44, 0x45, 0x43, 0x4c, 0x41, 0x52, 0x45, 0x20, 0x58,
        0x00, 0x05, 0x2e, 0x4d, 0x41, 0x49, 0x4e, 0x11, 0x20, 0x20, 0x46, 0x4f, 0x52, 0x20, 0x58,
        0x20, 0x3d, 0x20, 0x30, 0x20, 0x54, 0x4f, 0x20, 0x31, 0x35, 0x0b, 0x20, 0x20, 0x20, 0x20,
        0x50, 0x52, 0x49, 0x4e, 0x54, 0x20, 0x58, 0x1d, 0x20, 0x20, 0x4e, 0x45, 0x58, 0x54, 0x20,
        0x58, 0x20, 0x20, 0x20, 0x27, 0x20, 0x54, 0x52, 0x41, 0x49, 0x4c, 0x49, 0x4e, 0x47, 0x20,
        0x43, 0x4f, 0x4d, 0x4d, 0x45, 0x4e, 0x54,
    ];

    // functional style, yeah!
    DATA.iter()
        .copied()
        .enumerate()
        .for_each(|(offset, byte)| unsafe { lpoke(0x8010000u32 + offset as u32, byte) });
}

fn get_filename(verbose: &mut bool) -> String {
    println!("get-filename");
    let mut filename = String::new();
    let mut addr: u32 = 0x4ff00;
    // 7020 bank 4:ba=dec("ff00")
    // 7030 if peek(ba+0)=asc("s") and peek(ba+1)=asc("k") thenbegin
    const LETTER_S: u8 = 83;
    const LETTER_K: u8 = 75;
    if lpeek(addr) == LETTER_S && lpeek(addr + 1) == LETTER_K {
        // 7040   vb=peek(dec("ff07"))and8
        *verbose = lpeek(0x4ff07u32) & 8 == 8;
        if *verbose {
            println!("verbose");
        }
        // 7050   f$="":a=ba+16:dowhilepeek(a)<>0:f$=f$+chr$(peek(a)):a=a+1:loop:
        addr += 16;
        while lpeek(addr) != 0 {
            filename.push(lpeek(addr) as char);
            addr += 1;
        }

        // 7060   if peek(dec("ff07"))and1 thenreturn
        if lpeek(0x4ff07u32) & 1 == 1 {
            // this bit got referred to as an autoload bit?
            // it gets set by '11.edit' in the gosub 7720 (save filename in mailbox ram)
            return filename;
        }

        // 7070   print "filename? "+f$:print"{up}";
        println!("FILENAME? {}", &filename[..]);
        // 7080 bend
    }

    return filename;
    // NOTE: not sure how to do 'input' in rust yet, so skipping this part...
    // (maybe something in mega65's libc could do it?)

    // 7090 input "filename";a$
    // 7100 if a$="" thenprint "no filename set":end
    // 7110 poke ba,asc("s"):poke ba+1,asc("k")
    // 7120 forr=1to16:poke ba+8+r-1,asc(mid$(a$,r,1)):nextr
    // 7130 f$=a$
    // 7140 return
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}
