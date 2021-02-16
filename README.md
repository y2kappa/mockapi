# mockapi
A public service api to help test http calls.


## Run locally
```
$ cargo build
$ ./target/debug/server
```

## Build on Mac for linux
```
$ rustup target add x86_64-unknown-linux-musl
$ brew install FiloSottile/musl-cross/musl-cross
$ cat ~/.cargo/config
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```


References:
- https://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html
- https://john-millikin.com/notes-on-cross-compiling-rust

## To build on netlify
```
cat rust-toolchain
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-musl"]
```

References:
- https://docs.netlify.com/configure-builds/manage-dependencies/#rust