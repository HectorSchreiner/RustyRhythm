# WASM Extension Template

Use this template to quickly generate an empty *Hello World* web extension in Rust. This will get your code executed the right way.

Supports both manifest v2 and v3.

Will install wasm-specific build tools for you.

## Install tools

```sh
cargo install cargo-generate
cargo install wasm-pack
```

## Compile your extension

You cannot use `cargo build` to compile your crate. Use the `build.sh` script instead:

```sh
sh build.sh
```

Once compiled, the target files are ready to be used in the `pkg` folder.

## Test your program

Web browsers allow developpers to test web extensions before publishment.
See your browser's specific instructions to do that.
After build, the `manifest.json` file is located in the `pkg` folder.
By default, your extension will run on example.com and have no other permissions.
You should want to modify the manifest (see [the doc](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json)).
