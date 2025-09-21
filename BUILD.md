# How to Build

> [!NOTE]
> It's assumed you already have Rust installed, if not, please get it from [here](https://www.rust-lang.org/).

To build for binary, it's as easy as

```
cargo build --release
```

For a web build, you will need the `wasm32-unknown-unknown` triplet.

```
rustup target add wasm32-unknown-unknown
```

And target it when building...

```
cargo build --target wasm32-unknown-unknown --release
```

Once built, be sure to copy `index.html` from project root into the same space as the WASM file! From there you can serve it as-is or through some service and play to your hearts content.