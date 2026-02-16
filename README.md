<h1 align="center">shrimp-checker</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/shrimp-checker/releases/latest"><img src="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker?style=flat-square" alt="Latest Release"/></a>
  <a href="https://github.com/wielorzeczownik/shrimp-checker/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square" alt="License: MIT"/></a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/shrimp-checker/main/assets/logo.png" alt="shrimp-checker logo" width="200" />
</p>

I saw something like this on Twitter but couldn't find the original meme, so I built this app myself.

> Shrimp emoji source: <a href="https://emojipedia.org/joypixels/6.0/shrimp">JoyPixels 6.0</a>.

## Run from GitHub Release binaries

Each release includes prebuilt archives for Linux, macOS, and Windows.
Latest release: [GitHub Releases](https://github.com/wielorzeczownik/ahe-ics/releases/latest)

1. Download the asset for your platform from the latest release page.
2. Extract it.
3. Start the binary.

Example (Linux/macOS):

```bash
./shrimp-checker
```

Example (Windows PowerShell):

```powershell
.\shrimp-checker.exe
```

Release artifact names follow:

- `shrimp-checker-<version>-x86_64-unknown-linux-gnu.tar.gz` - Linux (Intel/AMD 64-bit)
- `shrimp-checker-<version>-aarch64-unknown-linux-gnu.tar.gz` - Linux (ARM64, e.g. Raspberry Pi 64-bit)
- `shrimp-checker-<version>-x86_64-apple-darwin.tar.gz` - macOS on Intel Macs
- `shrimp-checker-<version>-aarch64-apple-darwin.tar.gz` - macOS on Apple Silicon (M1/M2/M3)
- `shrimp-checker-<version>-x86_64-pc-windows-msvc.zip` - Windows 64-bit (x86_64)
