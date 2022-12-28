source export SSL_CERT_FILE=/workspaces/mos-hardware/nscacert.pem
cargo build --release --target mos-mega65-none --example 11-parse
