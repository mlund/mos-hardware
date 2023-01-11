/* automatically generated by rust-bindgen 0.63.0 */

pub const CH_HLINE: u32 = 192;
pub const CH_VLINE: u32 = 221;
pub const CH_ULCORNER: u32 = 176;
pub const CH_URCORNER: u32 = 174;
pub const CH_LLCORNER: u32 = 173;
pub const CH_LRCORNER: u32 = 189;
pub const CH_TTEE: u32 = 178;
pub const CH_BTEE: u32 = 177;
pub const CH_LTEE: u32 = 171;
pub const CH_RTEE: u32 = 179;
pub const CH_CROSS: u32 = 219;
pub const CH_CURS_UP: u32 = 145;
pub const CH_CURS_DOWN: u32 = 17;
pub const CH_CURS_LEFT: u32 = 157;
pub const CH_CURS_RIGHT: u32 = 29;
pub const CH_PI: u32 = 222;
pub const CH_HOME: u32 = 19;
pub const CH_DEL: u32 = 20;
pub const CH_INS: u32 = 148;
pub const CH_ENTER: u32 = 13;
pub const CH_STOP: u32 = 3;
pub const CH_LIRA: u32 = 92;
pub const CH_ESC: u32 = 27;
pub const CH_FONT_LOWER: u32 = 14;
pub const CH_FONT_UPPER: u32 = 142;
pub const CBM_A_RO: u32 = 1;
pub const CBM_A_WO: u32 = 2;
pub const CBM_A_RW: u32 = 3;
pub const CBM_READ: u32 = 0;
pub const CBM_WRITE: u32 = 1;
pub const CBM_SEQ: u32 = 2;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __clang_max_align_nonce2: f64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cbm_dirent {
    pub name: [::core::ffi::c_char; 17usize],
    pub size: ::core::ffi::c_uint,
    pub type_: ::core::ffi::c_uchar,
    pub access: ::core::ffi::c_uchar,
}

extern "C" {
    pub fn cbm_k_acptr() -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_basin() -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_bsout(C: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_chkin(FN: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_chrin() -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_chrout(C: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_ciout(C: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_ckout(FN: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_clall();
}
extern "C" {
    pub fn cbm_k_close(FN: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_clrch();
}
extern "C" {
    pub fn cbm_k_getin() -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_iobase() -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn cbm_k_listen(dev: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_load(flag: ::core::ffi::c_uchar, addr: ::core::ffi::c_uint)
        -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn cbm_k_open() -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_readst() -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_save(start: ::core::ffi::c_uint, end: ::core::ffi::c_uint)
        -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn cbm_k_scnkey();
}
extern "C" {
    pub fn cbm_k_second(addr: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_setlfs(
        LFN: ::core::ffi::c_uchar,
        DEV: ::core::ffi::c_uchar,
        SA: ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn cbm_k_setnam(Name: *const ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_settim(timer: ::core::ffi::c_ulong);
}
extern "C" {
    pub fn cbm_k_talk(dev: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_tksa(addr: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn cbm_k_udtim();
}
extern "C" {
    pub fn cbm_k_unlsn();
}
extern "C" {
    pub fn cbm_k_untlk();
}