# LogRythm Message Formatter
**Ruxamat**™ (*Name is Work In Progress*), is a formatter for LogRythm's WebConsole Platform. It runs as a chrome-extension, and reformats the LogMessage box, into something a bit prettier and easier for the eyes. It can reformat some of the log messages, and also highligt important text ex. Usernames and IP adresses. It is written in Rust and WebAssembly.
# Installation
In order to run the chrome extension, you need to add it to your extensions folder. You can do that by going to **Extensions -> Manage Extensions** -> **Enable Developer Mode** (*Click the Button in the Upper Right Corner*) And load the folder by pressing the **Load Unpacked** button.

You might need to recompile the rust code with WASM (commands down below).

## Install tools
You need `Rustup` installed on your machine (Install Guide Here: https://www.rust-lang.org/tools/install).
Or you can run the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
You also need to install the following with cargo.
```sh
cargo install cargo-generate
cargo install wasm-pack
```

## Compile the extension

You cannot use `cargo build` to compile your crate. Use the `build.sh` script instead:
```sh
sh build.sh
```
Once compiled, the target files are ready to be used in the `pkg` folder. And this is the folder you import as the extension.

# How To Use
Install the extension with the above steps. Once the extension is running in the browser, everything should work. The features can be editted from a config file, and toggled on and off by clicking the button in the extensions dropdown menu.

# Features
- **Highlighting**: Highlights important keywords such as USERNAMES, ADDRESSES...
- **Formatting**: Format the text, by creating spacing, and newlines. Also remove empty fields for better calrity.
- **Keyboard Shortcuts**: Might be implemented.

# Contributions
Feel free to contribute to the project or entirely rewrite it if you dont feel like programming in rust.

**Created by**: *Hector Schreiner Schousbo*
