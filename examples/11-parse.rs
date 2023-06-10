#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use alloc::format;
use core::panic::PanicInfo;
use mos_hardware::mega65::lpeek;
use mos_hardware::mega65::set_lower_case;
//use mos_hardware::mega65::libc::cputs;
use mos_hardware::mega65::libc::mega65_fast;
use mos_hardware::mega65::lpoke;

use ufmt_stdio::*;

const MAX_CAP: usize = 20;

const RVS_ON: &str = "\x12";
const RVS_OFF: &str = "\u{0092}";
const QUOTE_CHAR: &str = "\"";
/// pf$ = type_suffix
const _TYPE_SUFFIX: [&str; 4] = ["", "%", "$", "&"];

enum VarType {
    Float = 0,
    Int = 1,
    String = 2,
    Ref = 3,
    Define = 4
}

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

// wh$ = whitespace_chars
const WHITESPACE_CHARS: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

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
    let mut next_line = String::new();
    let mut verbose = true;
    let mut pp_line: u16 = 0;
    let mut delete_line_flag: bool = false;
    let mut inside_ifdef: bool = false;
    let mut labels: Vec<Label> = Vec::with_capacity(MAX_CAP);
    let mut var_table: [Vec<String>; 5] = Default::default();
    let mut argument_list: Vec<String> = Vec::with_capacity(MAX_CAP);
    let mut define_values: Vec<String> = Vec::with_capacity(MAX_CAP);

    // after expanding memory via 'link.ld', I needed to print something
    // very early, otherwise it would freeze up for some reason...

    prepare_test_memory(&mut verbose);

    // ln%() = map_gen_line_to_orig_line[]
    let _map_gen_line_to_orig_line: [u16; 500] = [0; 500];

    set_lower_case();

    // li$() = processed_lines
    // NOTE: Seems like rust chokes if this is too large?
    let _processed_lines: Vec<String> = Vec::with_capacity(MAX_CAP);
    // dim ec(4) = element_count per type
    let mut element_count: [u16; 5] = [ 0; 5 ];
    // dim vt$(4,200) = var_table per type

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
            }
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
                parse_preprocessor_directive(
                    &mut current_line,
                    &mut next_line,
                    &mut delete_line_flag,
                    &mut inside_ifdef,
                    &mut element_count,
                    &mut var_table,
                    &mut define_values,
                    &mut argument_list,
                    verbose);
            }
        }

        // 750
        source_line_counter += 1;
    }

    0
}

fn index_of(line: &str, token: &str) -> i16 {
    if let Some(index) = (*line).find(token) {
        //println!("found {} at {}!", token, index);
        return index as i16;
    }
    //println!("cOULDN'T FIND '{}' in '{}'", token, line);
    return -1;
}

// 603 - 607
fn parse_preprocessor_directive(
    current_line: &mut String,
    next_line: &mut String,
    delete_line_flag: &mut bool,
    inside_ifdef: &mut bool,
    element_count: &mut [u16; 5],
    var_table: & mut [Vec<String>; 5],
    define_values: &mut Vec<String>,
    argument_list: & mut Vec<String>,
    verbose: bool
) {
    if index_of(current_line, "IFDEF") == 1 {
        // println!("** ifdef!");
        let def_str = &current_line[7..];
        check_if_define_exists(def_str, inside_ifdef, element_count, var_table);
        *delete_line_flag = true;
    }
    else if index_of(current_line, "ENDIF") == 1 {
        //println!("** endif!");
        *inside_ifdef = false;
        *delete_line_flag = true;
    }
    else if index_of(current_line, "DEFINE") == 1 {
        println!("** define!");
        let line_suffix = current_line[8..].to_string();
        declare_var(&line_suffix, var_table,
            element_count,
            current_line, next_line,
            false, define_values, 
            argument_list,
            delete_line_flag, verbose);
    }
    else if index_of(current_line, "DECLARE") == 1 {
        println!("** declare!");
        let line_suffix = current_line[9..].to_string();
        declare_var(&line_suffix, var_table,
            element_count,
            current_line, next_line,
            false, define_values, 
            argument_list,
            delete_line_flag, verbose);
    }
}

// line 1000 - rem declare var(s) in s$
fn declare_var(
    varline: &str,
    var_table:  &mut [Vec<String>; 5],
    element_count: &mut [u16; 5],
    current_line: &mut String,
    next_line: &mut String,
    is_define: bool,
    define_values: &mut Vec<String>,
    argument_list: &mut Vec<String>,
    delete_line_flag: &mut bool,
    verbose: bool
) {
    println!("new var! {}", varline);
    parse_args(varline, ",;", true, argument_list);

    if argument_list.len() == 0 {
        println!("?DECLARE PARAMETER MISSING IN LINE ..."); // {}", source_line_counter);
        // TODO: need to do goto 1800
        return;
    }

    // lines 1030 - 1120
    for arg in argument_list {
        let mut dimension: String = String::new();

        let open_bkt_pos = arg.find('('); // b1
        let close_bkt_pos = arg.find(')'); // b2
        let equals_pos = arg.find('='); // eq
        let mut rhs = String::new();    // vl$
        let mut lhs = String::new();    // p$

        if let Some(eq_idx) = equals_pos {
            // --- assignment ---
            rhs = arg.split_off(eq_idx + 1);
            lhs = arg[..eq_idx - 1].to_string();
            trim_left(&mut lhs, &WHITESPACE_CHARS);
    
            trim_right(&mut rhs, &WHITESPACE_CHARS);
    
            if rhs.starts_with("$") {
                let mut hx = rhs[1..].to_string();
                convert_hex(&mut hx);
                rhs = hx;
            }
    
            if rhs.starts_with("%") {
                let mut bi = rhs[1..].to_string();
                convert_binary(&mut bi);
                rhs = bi;
            }
        }
    
        // 1050 - 1060
        if let (Some(opn_bkt_idx), Some(close_bkt_idx)) = (open_bkt_pos, close_bkt_pos) {
            // --- dimension ---
            dimension = arg[opn_bkt_idx + 1..close_bkt_idx].to_string();
            *arg = arg[..opn_bkt_idx - 1].to_string();
    
            replace_vars_and_labels_in_source_string(&mut dimension);
    
            *delete_line_flag = false;
        }

        let mut var_type = VarType::Float; // var type
        let t = lhs.chars().rev().next();
        if verbose {
            print!("adding {{rvon}}");
        }
    
        let mut t_str = String::new();
        if let Some(t_char) = t {
            t_str.push(t_char);
        }
    
        if !t_str.contains(&"%&$") {
            t_str = String::new();
            var_type = VarType::Float;
        }

        if *delete_line_flag == true {
            var_type = VarType::Define;
        }
    
        if t_str == "%" {
            var_type = VarType::Int;
        }
    
        if t_str == "$" {
            var_type = VarType::String;
        }
    
        if t_str == "&" {
            var_type = VarType::Ref;
        }
            
        // 1074
        var_table[var_type as usize][element_count[var_type as usize] as usize] = arg.to_string();

        let mut var_name: String = String::new();

        if !dimension.is_empty() {
            let id = element_count[var_type as usize];
            var_name = generate_varname_from_index(id as usize);
            if *delete_line_flag == false {
                // nl$ = next_line
                next_line.push_str(&format!("dim {}{}({}):", var_name, t_str, dimension));
            }
        }
    
        if !rhs.is_empty() {
            let id = element_count[var_type as usize];
            var_name = generate_varname_from_index(id as usize);
            if *delete_line_flag == false {
                next_line.push_str(&format!("{}{}={}:", var_name, t_str, rhs));
            }
        }
    
        if *delete_line_flag == true {
            define_values[element_count[var_type as usize] as usize] = rhs.clone();
        }
    
        if verbose {
            print!("{}{{rvof}}: {}", arg.as_str(), element_count[var_type as usize]);
        }
    
        element_count[var_type as usize] += 1;
    }

    // 1120
    if !next_line.is_empty() {
        *delete_line_flag = false;
        *current_line = format!("^^{}", next_line);
    } else {
        *delete_line_flag = true;
    }    
}

// lines 5000 - 5030
fn generate_varname_from_index(id: usize) -> String {
    let vn: String;
    
    if id < 26 {
        vn = ((65 + id as u8) as char).to_string();
    } else {
        let n2 = id % 26;
        let n1 = id / 26 - 1;
        vn = format!("{}{}", ((65 + n1 as u8) as char).to_string(),
            ((65 + n2 as u8) as char).to_string());
    }
    
    vn
}

fn convert_binary(bi: &str) -> u16 {
    let mut br: u16 = 0; // result

    for (b, bc) in bi.chars().rev().enumerate() {
        let bc_str = bc.to_string();

        if bc_str != "1" && bc_str != "0" {
            // Bail out
            bail_out();
        }

        if bc_str == "1" {
            br += 1 << b;
        }
    }

    let c = br.to_string()[1..].to_string();
    let c_num = c.parse::<u16>().unwrap();

    c_num
}

fn convert_hex(hx: &str) -> u16 {
    match u16::from_str_radix(hx, 16) {
        Ok(vl) => vl,
        Err(_) => {
            bail_out(); // Call bail_out function in case of error
            0 // Return a default value or handle the error case accordingly
        }
    }
}

// lines 3000-3220
fn replace_vars_and_labels_in_source_string(s: &str) -> String {
    if s.starts_with("^^") {
        return s[2..].to_string();
    }
    
    let mut quote_flag = false;
    let mut a = String::new();
    let mut c = String::new();
    let d = "<>=+-#*/^,.:;() ";

    for b in s.chars() {
        if b.to_string() == QUOTE_CHAR {
            quote_flag = !quote_flag;
            if quote_flag {
                a.push_str(&assess_token(&c));
                c = String::new();
            } else {
                a.push_str(&c);
                c = String::new();
            }
        }

        if quote_flag {
            a.push(b);
            continue;
        }

        if d.contains(b) {
            a.push_str(&assess_token(&c));
            c = String::new();
            if b == ' ' {
                continue;
            }
        }

        a.push(b);
        c.push(b);
    }

    a.push_str(&assess_token(&c));
    let result = a + &c;
    result
}

fn assess_token(token: &str) -> String {
    // Logic to assess and convert the token if needed
    // Replace this with your implementation
    // You can return the converted token or modify it as required

    // Placeholder implementation: return the original token without any modification
    token.to_string()
}

fn bail_out() {
    // Perform necessary actions to bail out of the program
    // ...
    // Your implementation here
}

// line 2100
fn parse_args(
    s: &str,
    delimiter: &str,
    parse_brackets: bool,
    argument_list: &mut Vec<String> ) {

    let mut remaining_string = s.to_string();
    let mut argument_count = 0;
    let string_length = remaining_string.len();
    let mut inside_group = false;
    const SPACE_CHAR_ONLY: [u8; 1] = [32];

    (*argument_list) = Vec::with_capacity(MAX_CAP);

    if string_length == 0 {
        return; // no string
    }

    let mut i = 0;
    while i < string_length {
        let b = remaining_string.chars().nth(i).unwrap().to_string();
        println!("chr={}", b[..]);

        if b == "(" && parse_brackets == true {
            println!("inside!");
            inside_group = true;
        }

        if b == ")" && parse_brackets == true {
            println!("outside!");
            inside_group = false;
        }

        if delimiter.contains(&b) && inside_group == false {
            //(*argument_list).push(String::from(trim_all(&b[..], &SPACE_CHAR_ONLY)));
            let mut current_arg = argument_list[argument_count as usize].clone();
            trim_all(&mut current_arg, &SPACE_CHAR_ONLY);
            println!("arg={}", current_arg[..]);
            argument_list[argument_count as usize] = current_arg;
            argument_count += 1;
        } else if argument_count >= argument_list.len() {
            let current_arg = String::from(b);
            argument_list.push(current_arg);
        } else {
            let current_arg = &mut argument_list[argument_count as usize];
            current_arg.push_str(&b);
        }

        i += 1;
    }

    let mut last_arg: String = argument_list[argument_count as usize].clone();
    trim_all(&mut last_arg, &SPACE_CHAR_ONLY);
    println!("lastarg={}", last_arg[..]);
    argument_list[argument_count as usize] = last_arg;
}

// 9210
fn check_if_define_exists(
    def_str: &str,
    inside_ifdef: &mut bool,
    element_count: &mut [u16; 5],
    var_table: &[Vec<String>; 5]
) {
    *inside_ifdef = true;
    for k in 0..element_count[VarType::Define as usize] {
        if var_table[VarType::Define as usize][k as usize] == def_str {
            *inside_ifdef = false;
            return;
        }
    }
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
    let line_length = lpeek(*ca_addr) as u32;
    *ca_addr += 1;

    (*ca_addr..*ca_addr + line_length)
        .for_each(|address| (*current_line).push(lpeek(address) as char));

    *ca_addr += line_length;
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
        println!("label {} at pp_line {}", current_line[..], pp_line);
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

fn trim_all(line: &mut String, trim_chars: &[u8]) {
    *line = String::from(trim_left(&line[..], trim_chars));
    *line = String::from(trim_right(&line[..], trim_chars));
}

fn prepare_test_memory(verbose: &mut bool) {
    // turn on verbose flag
    // (in memory doesn't work yet, as I'd have to put dummy info into 0x4ff00 to be parsed by get_filename()
    // unsafe { lpoke(0x4ff07u32, 0x08u8); }

    // so for now, just hardcode the flag
    *verbose = true;

    const STRDATA: [&str; 9] = [
        "#output \"hello\"",
        "",
        "#declare x",
        "",
        "#define z=1",
        "#ifdef z",
        "  print z",
        "#endif",
        "#declare v1$(15)"
/*        ".main",
        "  for x = 0 to 15",
        "    print x",
        "  next x   ' trailing comment" */
    ];

    // write number of lines
    unsafe { lpoke(0x8010000u32, (STRDATA.len() & 0xff) as u8) }
    unsafe { lpoke(0x8010001u32, ((STRDATA.len() >> 8) & 0xff) as u8) }

    let mut offset = 2 as u32;

    // functional style, yeah!
    STRDATA.iter()
        .copied()
        .enumerate()
        .for_each(|(lineno, line)|{
            unsafe { lpoke(0x8010000u32 + offset, line.len() as u8)};
            offset += 1;
            for c in line.chars() {
                let mut cc: u8 = c as u8;
                if (cc >= 0x41 && cc <= 0x5a) || (cc >= 0x61u8 && cc <= 0x7au8) {
                    cc ^= 0x20; // toggle bit 5 to swap upper/lower case between ASCII and PETSCII
                }
                unsafe { lpoke(0x8010000u32 + offset, cc) };
                offset += 1;
            }});
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
