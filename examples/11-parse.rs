#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String, vec::Vec};
use core::panic::PanicInfo;
use mos_hardware::mega65::libc::setlowercase;
use mos_hardware::mega65::libc::lpeek;
use mos_hardware::mega65::libc::cputs;
use mos_hardware::mega65::libc::lpoke;
use mos_hardware::mega65::libc::mega65_fast;

use ufmt_stdio::*;

const RVS_ON: &str = "\x12";
const RVS_OFF: &str = "\u{0092}";

static mut verbose: bool = false;

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
    // rw$
    let tokens = ["print", "input", "if", "then", "else", "do", "loop", "while",
        "until", "gosub", "goto", "open", "close", "dopen", "dclose", "for", "next",
        "getkey", "hex$", "dim", "peek", "poke", "wait", "dec", "chr$", "asc", "sgn", "sqr",
        "graphic", "clr", "screen", "def", "begin", "bend", "len", "mid$", "right$", "left$",
        "instr", "for", "next", "step", "trap", "border", "and", "foreground",
        "background", "set", "abs", "sin", "cos", "tan", "log", "fre", "cursor", "pixel",
        "window", "rwindow", "line", "box", "circle", "ellipse", "palette", "restore", "data",
        "err$", "er", "el", "cursor", "on", "off", "val", "scratch", "return", "rnd", "stop",
        "bank", "ti", "do", "or", "st", "if", "el", "er", "on", "to", "pen", "get", "end",
        "int", "not", "ds", "run", "using", "append", "atn", "auto", "backup", "bload", "boot",
        "bsave", "bump", "bverify", "catalog", "change", "char", "cmd", "collision", "color",
        "concat", "cont", "copy", "wpoke", "wpeek", "setbit", "dclear", "deffn", "delete", "fn",
        "dir", "disk", "dload", "dma", "dmode", "dpat", "dsave", "dverify", "edma", "envelope",
        "erase", "exit", "exp", "fast", "filter", "find", "go64", "header", "help", "highlight",
        "joy", "list", "load", "locate", "lpen", "mod", "monitor", "mouse", "movspr", "new",
        "paint", "play", "pointer", "polygon", "pos", "pot", "pudef", "rclr", "rdot", "read",
        "record", "rem", "rename", "resume", "rgraphic", "rmouse", "rplay", "rreg", "rspcolor",
        "rsppos", "rsprite", "save", "scnclr", "sleep", "slow", "sound", "spc", "sprcolor",
        "sprite", "sprsav", "sys", "tab", "tempo", "troff", "tron", "type", "usr", "verify",
        "vol", "xor", "key"];

    prepare_test_memory();

    // pf$ = type_suffix
    let type_suffix = ["", "%", "$", "&"];

    // b() = bin_conv[]
    let mut bin_conv: [u16; 16] = [0; 16];
    bin_conv[0] = 1;
    for x in 1..16 {
        bin_conv[x] = bin_conv[x-1] * 2;
    }

    // ln%() = map_gen_line_to_orig_line[]
    let map_gen_line_to_orig_line: [u16; 1000] = [0; 1000];

    unsafe { setlowercase(); }
    println!("testing TESTING 1, 2, 3...");

    // li$() = processed_lines
    // NOTE: Seems like rust chokes if this is too large?
    //let processed_lines: Vec<String> = Vec::with_capacity(1000);

    unsafe { setlowercase(); }
    println!("{}eleven PREPROCESSOR V0.4.7{}", RVS_ON, RVS_OFF);
    
    //unsafe { cputs("hello".as_ptr()); }
    println!();
    
    // tl$ = tl_string
    let mut tl_string = String::from("                                                                                ");
    // bl$ = bl_string
    //let mut bl_string: String = String::new();
    //bl_string.push_str(&tl_string[..]);
    //bl_string.push_str(&tl_string[..]);
    //bl_string.push_str(&tl_string[..]);

    tl_string = String::new();

    //for i in 0..tokens.len() {
    //    println!("{}", tokens[i]);
    //}

    let mystring = String::from("test");
    println!("{}", &mystring[..]);

    let filename = get_filename();

    unsafe { mega65_fast(); }

    println!("{}", &filename[..]);

    // ------------------- pass 1 ---------------
    // nl = next_line_flag
    let mut next_line_flag = false;

    // wh$ = whitespace_chars
    let whitespace_chars: &[u8] = &[32, 160, 29, 9]; // space, shift+space, right, tab

    // clean up temporary files
    let source_line_counter = 0;

    // TODO: 195 clr ti: rem keep start time for timing
    let mut cb_addr: i32 = 0x8010000;
    let mut ca_addr: i32 = cb_addr;

    println!("PASS 1 ");

    // rl = current_line_index (zero-indexed, increments by one)
    let mut current_line_index = 0;
    // tl = total_lines
    let mut total_lines: u16 = 0;
    unsafe { total_lines = lpeek(ca_addr) as u16 + 256 * lpeek(ca_addr + 1) as u16; }

    ca_addr += 2;

    unsafe
    {
        //200
        while current_line_index != total_lines
        {
            let line_length: u8 = lpeek(ca_addr) as u8;
            ca_addr += 1;
            let mut current_line = String::new();
            let mut idx: u8 = 0;
            while idx < line_length {
                current_line.push(lpeek(ca_addr) as char);
                ca_addr += 1;
                idx += 1;
            }

            println!("l{}: {}", current_line_index, &current_line[..]);
            current_line_index += 1;

            current_line = String::from(trim_left(&current_line[..], &whitespace_chars[..]));
            println!("{}", &current_line[..]);

            let mut quote_flag = false;
            let mut cut_tail_idx = None;

            // single-quote comment trimming logic
            // -----------------------------------
            //422
            cut_tail_idx = current_line.find('\'');
            if cut_tail_idx != None {
                //423
                if current_line.contains('"') {
                    //424
                    cut_tail_idx = None;
                    //440
                    for in_line_idx in 0..current_line.len() {
                        let c = current_line.chars().nth(in_line_idx).unwrap();
                        if c == '"' { // quote-quote?
                            quote_flag = !quote_flag;
                        } else if c == '\'' && !quote_flag {
                            cut_tail_idx = Some(in_line_idx);
                            break;
                        }
                    }
                }
                //540
                if cut_tail_idx != None {
                    current_line = String::from(&current_line[..cut_tail_idx.unwrap()]);
                }
            }
            println!("'{}'", &current_line[..]);

            //break;
        }
    }

    0
}

fn trim_left<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str
{
    let mut i = 0;

    while i < line.len() && trim_chars.contains(&line.as_bytes()[i]) {
        i = i + 1;
    }
    
    &line[i..]
}

fn prepare_test_memory() {
    let data: [u8;97] = [
        0x08, 0x00, 0x0f, 0x23, 0x4f, 0x55, 0x54, 0x50, 0x55, 0x54, 0x20, 0x22, 0x48, 0x45, 0x4c, 0x4c,
        0x4f, 0x22, 0x00, 0x0a, 0x23, 0x44, 0x45, 0x43, 0x4c, 0x41, 0x52, 0x45, 0x20, 0x58, 0x00, 0x05,
        0x2e, 0x4d, 0x41, 0x49, 0x4e, 0x11, 0x20, 0x20, 0x46, 0x4f, 0x52, 0x20, 0x58, 0x20, 0x3d, 0x20,
        0x30, 0x20, 0x54, 0x4f, 0x20, 0x31, 0x35, 0x0b, 0x20, 0x20, 0x20, 0x20, 0x50, 0x52, 0x49, 0x4e,
        0x54, 0x20, 0x58, 0x1d, 0x20, 0x20, 0x4e, 0x45, 0x58, 0x54, 0x20, 0x58, 0x20, 0x20, 0x20, 0x27,
        0x20, 0x54, 0x52, 0x41, 0x49, 0x4c, 0x49, 0x4e, 0x47, 0x20, 0x43, 0x4f, 0x4d, 0x4d, 0x45, 0x4e,
        0x54
    ];

    for idx in 0..data.len() {
        unsafe { lpoke(0x8010000i32 + idx as i32, data[idx]); }
    }
}

fn get_filename() -> String {
    let mut filename = String::new();
    let mut addr: i32 = 0x4ff00;
    unsafe {
        // 7020 bank 4:ba=dec("ff00")
        // 7030 if peek(ba+0)=asc("s") and peek(ba+1)=asc("k") thenbegin
        if lpeek(addr) == 83   /* 's' */ &&
        lpeek(addr+1) == 75 /* 'k' */
        {
            // 7040   vb=peek(dec("ff07"))and8
            verbose = lpeek(0x4ff07i32) & 8 == 8;
            if verbose {
                println!("verbose");
            }
            // 7050   f$="":a=ba+16:dowhilepeek(a)<>0:f$=f$+chr$(peek(a)):a=a+1:loop:
            addr += 16;
            while lpeek(addr) != 0 {
                filename.push(lpeek(addr) as char);
                addr += 1;
            }

            // 7060   if peek(dec("ff07"))and1 thenreturn
            if lpeek(0x4ff07i32) & 1 == 1 {
                // this bit got referred to as an autoload bit?
                // it gets set by '11.edit' in the gosub 7720 (save filename in mailbox ram)
                return filename;
            }

            // 7070   print "filename? "+f$:print"{up}";
            println!("FILENAME? {}", &filename[..]);        
            // 7080 bend    
        }
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
