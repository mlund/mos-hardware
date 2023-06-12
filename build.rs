/// Call to update `mega65/libc/bindings.rs` with
/// ~~~
/// cargo build --lib --release
/// rustfmt src/mega65/libc/bindings.rs
/// ~~~
/// Call to update mega65/libc/bindings.rs
#[cfg(feature = "mega65")]
fn _make_mega65_libc_bindings() {
    bindgen::Builder::default()
        .header("src/mega65/libc/conio.h")
        .header("src/mega65/libc/debug.h")
        .header("src/mega65/libc/dirent.h")
        .header("src/mega65/libc/fileio.h")
        .header("src/mega65/libc/hal.h")
        .header("src/mega65/libc/memory.h")
        .header("src/mega65/libc/random.h")
        .header("src/mega65/libc/sdcard.h")
        .header("src/mega65/libc/targets.h")
        .header("src/mega65/libc/tests.h")
        .header("src/mega65/libc/time.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("::core::ffi")
        .use_core()
        .derive_default(true)
        .generate()
        .unwrap()
        .write_to_file("src/mega65/libc/bindings.rs")
        .expect("Couldn't write bindings!");
}

/// update cbm kernal bindings
fn _make_cbm_kernal_bindings() {
    bindgen::Builder::default()
        .header("cbm.h") // from llvm-mos-sdk/mos-targets/commodore/cbm.h
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("::core::ffi")
        .use_core()
        .derive_default(true)
        .generate()
        .unwrap()
        .write_to_file("src/cbm_kernal.rs")
        .expect("Couldn't write bindings!");
}
#[cfg(feature = "docs-rs")]
fn main() {} // Skip the script when the doc is building

#[cfg(not(feature = "docs-rs"))]
fn main() {
    //_make_mega65_libc_bindings();
    //_make_cbm_kernal_bindings();
    #[cfg(feature = "c64")]
    cc::Build::new()
        .compiler("clang")
        .target("mos-c64")
        .file("src/irq.c")
        .compile("irq");

    #[cfg(feature = "mega65")]
    cc::Build::new()
        .compiler("clang")
        .target("mos-mega65")
        .include("src/mega65/libc")
        .include("/usr/local/mos-platform/common/include")
        .files([
            "src/mega65/libc/conio.c",
            "src/mega65/libc/debug.c",
            "src/mega65/libc/dirent.s",
            "src/mega65/libc/unmap-basic.S",
            "src/mega65/libc/example.c",
            "src/mega65/libc/fat32.c",
            "src/mega65/libc/fileio.s",
            "src/mega65/libc/hal.c",
            "src/mega65/libc/memory.c",
            "src/mega65/libc/mouse.c",
            "src/mega65/libc/random.c",
            "src/mega65/libc/sdcard.c",
            "src/mega65/libc/targets.c",
            "src/mega65/libc/tests.c",
            "src/mega65/libc/time.c",
        ])
        .flag("-mcpu=mos65c02")
        .flag("-w")
        .flag("-Os")
        .flag("-T link.ld")
        .compile("mega65libc");
}
