// build.rs

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return
    }
    cc::Build::new()
        .file("src/irq.c")
        .compile("irq");
}
