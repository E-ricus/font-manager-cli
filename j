[package]
name = "font-manager"
description = "A simple utility to manage fonts on linux systems"
authors = [ "Eric Puentes <eric.david2333@gmail.com>" ]
version = "0.0.6"
edition = "2021"
license = "MIT"
readme = "README.md"
repository = "https://github.com/E-ricus/font-manager-cli"
homepage = "https://github.com/E-ricus/font-manager-cli"

exclude = [
    "test-data/*"
]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0"
home = "0.5.9"
log = "0.4.21"
pretty_env_logger = "0.4.0"
reqwest = "0.11.14"
clap = { version = "4.1.4", features = ["derive"] }
text_io = "0.1.12"
thiserror = "1.0.38"
tokio = { version = "1.24.2", features = ["macros", "rt-multi-thread", "net"] }
zip = "0.6.3"
