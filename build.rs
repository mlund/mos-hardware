// build.rs

fn main() {
    cc::Build::new()
        .file("src/irq.c")
        .compile("irq");
}
