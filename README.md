# Test Hermes

This is a simple Rust application that opens a text file and displays its content with a GUI.

## Features

- Open text files via native file dialog
- Display file contents in a scrollable text area
- Error handling with crash.log generation
- Windows 11 compatible
- Optimized build for size

## Building

To build this application locally, you need Rust installed on your system. Run:

```
cargo build --release
```

The executable will be created at `target/release/testhermes.exe`.

## Usage

1. Run the executable
2. Click "File" → "Open" to select a text file
3. The file content will be displayed in the main window
4. In case of errors, a crash.log file is generated in the same directory

## Compatibility

This application is designed for Windows 11 and uses the glow backend for better CI compatibility.

## Release Information

The latest release can be found at: https://github.com/imtheyoyo/testhermes/releases/tag/v0.3.4