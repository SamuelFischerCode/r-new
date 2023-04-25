deploy : 
    cargo fmt
    cargo clippy
    cargo build --release
    cp target/release/r_new /usr/local/bin/r-new
