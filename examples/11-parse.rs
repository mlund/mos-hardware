#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String, vec::Vec};
use core::panic::PanicInfo;
use mos_hardware::mega65::libc::setlowercase;
use mos_hardware::mega65::libc::lpeek;
//use mos_hardware::mega65::libc::lpoke;
use mos_hardware::mega65::libc::mega65_fast;

use ufmt_stdio::*;

const RVS_ON: &str = "\x12";
const RVS_OFF: &str = "\u{0092}";

static mut verbose: bool = false;

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

    // li$() = processed_lines
    //let mut processed_lines: Vec<String> =  vec![String::new(); 1000];
    let processed_lines: Vec<String> = Vec::with_capacity(1000);
    //let mut processed_lines: [String; 1000] = [String::from(""); 1000];

    unsafe { setlowercase(); }
    println!("{}eleven PREPROCESSOR V0.4.7{}", RVS_ON, RVS_OFF);
    println!();
    
    // tl$ = tl_string
    let mut tl_string = String::from("                                                                               ");
    // bl$ = bl_string
    let mut bl_string: String = String::new();
    bl_string.push_str(&tl_string[..]);
    bl_string.push_str(&tl_string[..]);
    bl_string.push_str(&tl_string[..]);
    bl_string.push_str(&tl_string[..]);

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
    let whitespace_chars: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

    // clean up temporary files
    let source_line_counter = 0;

    // TODO: 195 clr ti: rem keep start time for timing
    let mut cb_addr: i32 = 0x8010000;
    let mut ca_addr: i32 = cb_addr;
    
    print!("PASS 1 ");

    // rl = current_line_index (zero-indexed, increments by one)
    let current_line_index = 0;
    // tl = total_lines
    let mut total_lines: u16 = 0;
    unsafe { total_lines = lpeek(ca_addr) as u16 + 256 * lpeek(ca_addr + 1) as u16; }

    ca_addr += 2;

    unsafe
    {
        while current_line_index != total_lines
        {
            let line_length = lpeek(ca_addr) as usize;
            let current_line = String::from(&bl_string[..line_length]);
            let line_ptr = &current_line;
        }
    }

    0
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
