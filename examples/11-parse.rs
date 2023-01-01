#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String, vec::Vec};
use core::panic::PanicInfo;
use mos_hardware::mega65::set_lower_case;
use mos_hardware::mega65::lpeek;
//use mos_hardware::mega65::libc::cputs;
use mos_hardware::mega65::lpoke;
use mos_hardware::mega65::libc::mega65_fast;

use ufmt_stdio::*;

const RVS_ON: &str = "\x12";
const RVS_OFF: &str = "\u{0092}";

struct GlobalVars {
    verbose: bool,
    current_line: String,
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
    let mut var = GlobalVars {
        verbose: true,
        current_line: String::new(),
        pp_line: 0,
    };

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

    prepare_test_memory(&mut var);

    // pf$ = type_suffix
    let TYPE_SUFFIX: [&str; 4] = ["", "%", "$", "&"];

    // TODO: Convert to `bin_conv` constant evaluation: https://doc.rust-lang.org/reference/const_eval.html
    let mut bin_conv: [u16; 16] = [0; 16];
    bin_conv[0] = 1;
    for x in 1..16 {
        bin_conv[x] = bin_conv[x-1] * 2;
    }

    // ln%() = map_gen_line_to_orig_line[]
    let map_gen_line_to_orig_line: [u16; 500] = [0; 500];

    set_lower_case();
    println!("testing TESTING 1, 2, 3...");

    // li$() = processed_lines
    // NOTE: Seems like rust chokes if this is too large?
    let processed_lines: Vec<String> = Vec::with_capacity(500);

    set_lower_case();
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

    //let mystring = String::from("test");
    //println!("{}", &mystring[..]);

    let filename = get_filename(&mut var);

    unsafe { mega65_fast(); }

    println!("{}", &filename[..]);

    // ------------------- pass 1 ---------------
    // nl = next_line_flag
    let mut next_line_flag = false;

    // wh$ = whitespace_chars
    let whitespace_chars: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

    // clean up temporary files
    // NOTE: sl/source_line_counter and rl/current_line_index serve the same purpose
    // so I will remove 'current_line_index'
    let mut source_line_counter = 0;    // sl
    let mut post_proc_line_counter = 0; // ln

    // TODO: 195 clr ti: rem keep start time for timing
    let mut cb_addr: u32 = 0x8010000;
    let mut ca_addr: u32 = cb_addr;

    println!("PASS 1 ");

    // rl = current_line_index (zero-indexed, increments by one)
    // removing this one, as it's equivalent to sl/soure_line_counter
    //let mut current_line_index = 0;
    // tl = total_lines
    let mut total_lines: u16 = 0;
    total_lines = lpeek(ca_addr) as u16 + 256 * lpeek(ca_addr + 1) as u16;

    ca_addr += 2;

    var.pp_line = 0; // ln = index into li$ (current post-processed line)

    //200
    while source_line_counter != total_lines
    {
        // copy line's data into 'curent_line'
        // -----------------------------------
        let line_length: u8 = lpeek(ca_addr) as u8;
        ca_addr += 1;
        var.current_line = String::new();
        let mut idx: u8 = 0;
        while idx < line_length {
            var.current_line.push(lpeek(ca_addr) as char);
            ca_addr += 1;
            idx += 1;
        }

        println!("l{}: {}", source_line_counter, &var.current_line[..]);

        var.current_line = String::from(trim_left(&var.current_line[..], &whitespace_chars[..]));
        println!("{}", &var.current_line[..]);

        let mut quote_flag = false;
        let mut cut_tail_idx = None;

        // single-quote comment trimming logic
        // -----------------------------------
        //422
        cut_tail_idx = var.current_line.find('\'');
        if cut_tail_idx != None {
            //423
            if var.current_line.contains('"') {
                //424
                cut_tail_idx = None;
                //440
                for (in_line_idx, c) in var.current_line.chars().enumerate() {
                    //let c = current_line.chars().nth(in_line_idx).unwrap();
                    match c {
                        ':' => quote_flag = !quote_flag,
                        '\'' => if !quote_flag {
                            cut_tail_idx = Some(in_line_idx);
                            break;    
                        },
                        _ => (),
                    }
                }
            }
            //540
            if cut_tail_idx != None {
                var.current_line = String::from(&var.current_line[..cut_tail_idx.unwrap()]);
            }
        }
        //println!("'{}'", &current_line[..]);

        //560-580
        if var.current_line.len() > 0 {
            var.current_line = String::from(trim_right(&var.current_line[..], &whitespace_chars[..]));
            //println!("'{}'", &current_line[..]);
        }

        //585
        if var.current_line.len() > 0 {
            let mut delete_line_flag = false;
            if var.verbose {
                println!(">> {} {} {}", post_proc_line_counter, source_line_counter, &var.current_line[..]);
                // 600
                if (&var.current_line[..1]).eq(".") {
                    println!("dot!");
                    next_line_flag = true;
                    parse_label(&mut var);
                }
            }
        }

        // 750
        source_line_counter += 1;
    }

    0
}

fn parse_label(var: &mut GlobalVars)
{
    if var.verbose {
        println!("label {} at pp_line {}", &var.current_line[..], var.pp_line);
    }
}

fn trim_left<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str
{
    let mut i = 0;

    while i < line.len() && trim_chars.contains(&line.as_bytes()[i]) {
        i = i + 1;
    }
    
    &line[i..]
}

fn trim_right<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str
{
    let mut i: i16 = (line.len()-1) as i16;

    while i >= 0 && trim_chars.contains(&line.as_bytes()[i as usize]) {
        i = i - 1;
    }
    
    &line[..((i+1) as usize)]
}

fn prepare_test_memory(var: &mut GlobalVars) {
    // turn on verbose flag
    // (in memory doesn't work yet, as I'd have to put dummy info into 0x4ff00 to be parsed by get_filename()
    // unsafe { lpoke(0x4ff07u32, 0x08u8); }

    // so for now, just hardcode the flag
    var.verbose = true;

    let data: [u8;97] = [
        0x08, 0x00, 0x0f, 0x23, 0x4f, 0x55, 0x54, 0x50, 0x55, 0x54, 0x20, 0x22, 0x48, 0x45, 0x4c, 0x4c,
        0x4f, 0x22, 0x00, 0x0a, 0x23, 0x44, 0x45, 0x43, 0x4c, 0x41, 0x52, 0x45, 0x20, 0x58, 0x00, 0x05,
        0x2e, 0x4d, 0x41, 0x49, 0x4e, 0x11, 0x20, 0x20, 0x46, 0x4f, 0x52, 0x20, 0x58, 0x20, 0x3d, 0x20,
        0x30, 0x20, 0x54, 0x4f, 0x20, 0x31, 0x35, 0x0b, 0x20, 0x20, 0x20, 0x20, 0x50, 0x52, 0x49, 0x4e,
        0x54, 0x20, 0x58, 0x1d, 0x20, 0x20, 0x4e, 0x45, 0x58, 0x54, 0x20, 0x58, 0x20, 0x20, 0x20, 0x27,
        0x20, 0x54, 0x52, 0x41, 0x49, 0x4c, 0x49, 0x4e, 0x47, 0x20, 0x43, 0x4f, 0x4d, 0x4d, 0x45, 0x4e,
        0x54
    ];

    for (idx, byte) in data.iter().enumerate() {
        unsafe { lpoke(0x8010000u32 + idx as u32, *byte); }
    }
}

fn get_filename(var: &mut GlobalVars) -> String {
    let mut filename = String::new();
    let mut addr: u32 = 0x4ff00;
    // 7020 bank 4:ba=dec("ff00")
    // 7030 if peek(ba+0)=asc("s") and peek(ba+1)=asc("k") thenbegin
    if lpeek(addr) == 83   /* 's' */ &&
    lpeek(addr+1) == 75 /* 'k' */
    {
        // 7040   vb=peek(dec("ff07"))and8
        var.verbose = lpeek(0x4ff07u32) & 8 == 8;
        if var.verbose {
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
