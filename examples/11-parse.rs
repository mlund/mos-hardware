#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String};
use core::panic::PanicInfo;
use mos_hardware::mega65::libc::setlowercase;
use mos_hardware::mega65::libc::lpeek;
use mos_hardware::mega65::libc::lpoke;
// use mos_hardware::mega65::poke;
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

    unsafe { setlowercase(); }
    println!("{}eleven PREPROCESSOR V0.4.7{}", RVS_ON, RVS_OFF);
    println!();
    
    //for i in 0..tokens.len() {
    //    println!("{}", tokens[i]);
    //}

    let mystring = String::from("test");
    println!("{}", &mystring[..]);

    let filename = get_filename();

    println!("{}", &filename[..]);
    0
}

fn get_filename() -> String {
    let mut filename = String::new();
    let mut addr: i32 = 0x4ff00;
    unsafe {
        lpoke(0xffd3020i32, 0);
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
