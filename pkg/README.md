# LogRhythm LogMessage Formatter
RustyRhythm *(Name is Work in Progress)* is a Chrome extension that enhances LogRhythm's WebConsole platform by reformatting log messages into a more readable and visually appealing format. It highlights important text — such as **usernames and IP addresses** — and restructures log messages for improved clarity.

**Built with:** Rust 🦀 + WebAssembly ⚡

# Installation
## **Add the Extension to Chrome**
1. Open Chrome and navigate to:
   **Extensions → Manage Extensions**
2. **Enable Developer Mode** (toggle the button in the upper-right corner).
3. Click **Load Unpacked** and select the `pkg` folder.

---

## Install Dependencies

The extension can be installed under `releases` (TODO!). But if you want to compile it yourself, you'll need `Rustup` installed on your machine. Install it using:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

🔗[Rust Installation Guide](https://www.rust-lang.org/tools/install)

Then, install the required tools:
```sh
cargo install cargo-generate
cargo install wasm-pack
```

## Compile the Extension
Instead of using `cargo build`, run the provided build script:
```
sh build.sh
```
Once compiled, your extension files will be available in the `pkg` folder. This is the folder you need to import into Chrome.

# How to Use
1. Install the extension using the steps above.
2. The extension will automatically enhance the LogMessage box in the WebConsole.
3. Configure features via the config file.
4. Toggle features on/off from the extension dropdown menu.

# ✨ Features
- [x] **Highlighting:** Automatically highlights important data like usernames and IP-Addresses.
- [x] **Custom Deletion and Replacement Rules:** Delete certain patterns and words in the alarm text.
- [x] **JSON Reformatting:** Reformats Json fields in the alarmtext so it looks more visually appealing.
- [x] **Config file:** Configure the extension with an easy to configure `config.json`
- [ ] **ON/OFF Toggle Keyboard Shortcut:** (*Planned feature!*)
- [ ] **Popup.html:** Nicer User Interface. (*Planned feature!*)
- [ ] **User Customization:** The user should be able to customize formatting, color, highligtwords... (*Planned feature!*)

# Config Example
You can find more information by reading the [Documentation](https://github.com/HectorSchreiner/RustyRhythm/blob/master/CONFIG_DOCUMENTATION.md)
```json
{
  "highlight_rules": [
    {
      "rule_type": "exact",
      "pattern": "word_i_want_to_be_red",
      "style": "color:red;font-weight:bold;"
    },
    {
      "rule_type": "regex",
      "pattern": "\\b(?:\\d{1,3}\\.){3}\\d{1,3}\\b",
      "style": "color:red;font-weight:bold;"
    }
  ],
  "deletion_rules": [
    {
      "rule_type": "exact",
      "pattern": "DEBUG"
    },
    {
      "rule_type": "regex",
      "pattern": "<\\w> = <>"
    }
  ],
  "change_rules": [
    {
      "rule_type": "exact",
      "pattern": "red",
      "replacement": "blue"
    },
    {
      "rule_type": "regex",
      "pattern": "\\something\\b",
      "replacement": "somethingelse"
    }
  ]
}

```

# Demonstration
Test File for WASM
Before:

{"timestamp": "2024-02-06T12:35:10Z", "event": "file_access", "user": "john_doe", "file": "/etc/passwd", "ip": "192.168.1.1"},{"timestamp": "2024-02-06T12:35:10Z", "event": "file_access", "user": "john_doe", "file": "/etc/passwd"} unusefuldata

After RustyRythm:
```
{
  "event": "file_access",
  "file": "/etc/passwd",
  "ip": "192.168.1.1",
  "timestamp": "2024-02-06T12:35:10Z",
  "user": "john_doe"
},{
  "event": "file_access",
  "file": "/etc/passwd",
  "timestamp": "2024-02-06T12:35:10Z",
  "user": "john_doe"
}
```

# 💡 Contributions
Contributions are welcome! If you want to conribute with some changes, you can fork the repository, and submit a pull request. If you prefer another language over Rust, feel free to rewrite it...

**Created by:** *Hector Schreiner Schousbo*
