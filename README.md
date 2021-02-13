# mockapi
A public service api to help test http calls.


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