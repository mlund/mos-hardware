// build.rs

#[cfg(feature = "docs-rs")]
fn main() {} // Skip the script when the doc is building

#[cfg(not(feature = "docs-rs"))]
fn main() {
    cc::Build::new()
        .compiler("mos-c64-clang")
        .file("src/irq.c")
        .compile("irq");
}
