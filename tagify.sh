$! /bin/sh

export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/library"

/home/adam/.cargo/bin/rusty-tags -f vi 
