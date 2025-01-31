# LogRythm Message Formatter
**Ruxamat**™ (*Name is Work In Progress*), is a formatter for LogRythm's WebConsole Platform. It runs as a chrome-extension, and reformats the LogMessage box, into something a bit prettier and easier for the eyes. It is written in Rust and WebAssembly.
# Installation
In order to run the chrome extension, you need to add it to your extensions folder. You can do that by going to **Extensions -> Manage Extensions** -> **Enable Developer Mode** (*Click the Button in the Upper Right Corner*) And load the folder by pressing the **Load Unpacked** button.

You might need to recompile the rust code with WASM (commands down below).

Compile the program yourself with the following commands:

## Install tools
You need **Rust** installed on your machine (Install Guide Here: https://www.rust-lang.org/tools/install)
Or you can run the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*Cargo Installations*:
```sh
cargo install cargo-generate
cargo install wasm-pack
```

### Windows
```
Cargo compile everything
```

### Linux
```
Cargo compile everything
```


## Compile the extension

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
